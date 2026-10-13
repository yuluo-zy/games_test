// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui/signifiers/animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 670b3c664eb21abbf4a2caa843a9a08b2ec224423fc5cba95091f8e5cde70174
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 429
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::animation::signifier_animation_t
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":429,"record_offset":3196,"rva":6022768,"va":5374731888}
// 编译实例 RVAs: 0x5be670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8575798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e836db8200","size":5,"soft_fail":false,"target_va":5383308144,"va":5374732341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28245447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e8c7fdae01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374733236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e827f5ffff","size":5,"soft_fail":false,"target_va":5374730624,"va":5374733396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28247733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e8b506af01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374732390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e878fbffff","size":5,"soft_fail":false,"target_va":5374730816,"va":5374731971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28248610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e8220aaf01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374732041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1624353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e821c91800","size":5,"soft_fail":false,"target_va":5376356912,"va":5374732554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1628234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e84ad81800","size":5,"soft_fail":false,"target_va":5376360560,"va":5374732321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1624197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e885c81800","size":5,"soft_fail":false,"target_va":5376356144,"va":5374731942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2204968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374731888,"raw_hex":"e828a52100","size":5,"soft_fail":false,"target_va":5376938064,"va":5374733091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-2605","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374731888,"raw_hex":"e9d3f5ffff","size":5,"soft_fail":false,"target_va":5374730624,"va":5374733224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":429,"record_offset":7928,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":6216,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":429,"record_offset":6016,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":429,"record_offset":8040,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":8128,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":4968,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":3380,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":3956,"parent_record_offset":3196,"ancestor_record_offset":3196,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":429,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":3196,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":6624,"parent_record_offset":6596,"ancestor_record_offset":3196,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":7596,"parent_record_offset":7564,"ancestor_record_offset":3196,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":6928,"parent_record_offset":6896,"ancestor_record_offset":3196,"inlinee":6327,"source_line":1372,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":3196,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":7652,"parent_record_offset":7564,"ancestor_record_offset":3196,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":3196,"inlinee":24860,"source_line":1566,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":6596,"parent_record_offset":6488,"ancestor_record_offset":3196,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":3196,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":3196,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":3196,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":6248,"parent_record_offset":6216,"ancestor_record_offset":3196,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":429,"record_offset":6044,"parent_record_offset":6016,"ancestor_record_offset":3196,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":429,"record_offset":7564,"parent_record_offset":6248,"ancestor_record_offset":3196,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":429,"record_offset":6488,"parent_record_offset":6248,"ancestor_record_offset":3196,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":429,"record_offset":7492,"parent_record_offset":7468,"ancestor_record_offset":3196,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":3196,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":3196,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":3196,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":6812,"parent_record_offset":6748,"ancestor_record_offset":3196,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":3196,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":7524,"parent_record_offset":7236,"ancestor_record_offset":3196,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":7468,"parent_record_offset":7236,"ancestor_record_offset":3196,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":6308,"parent_record_offset":6248,"ancestor_record_offset":3196,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":3196,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":3196,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":3196,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":6404,"parent_record_offset":6248,"ancestor_record_offset":3196,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":429,"record_offset":7756,"parent_record_offset":7728,"ancestor_record_offset":3196,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":429,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":3196,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":3196,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":3196,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":3196,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":3196,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":3196,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5512,"parent_record_offset":5340,"ancestor_record_offset":3196,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":3196,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":429,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":3196,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":429,"record_offset":3592,"parent_record_offset":3496,"ancestor_record_offset":3196,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":3196,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":3196,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":4552,"parent_record_offset":4092,"ancestor_record_offset":3196,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":5252,"parent_record_offset":5076,"ancestor_record_offset":3196,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":429,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":3196,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":5576,"parent_record_offset":5280,"ancestor_record_offset":3196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":429,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":3196,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3196,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":3196,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":4512,"parent_record_offset":4480,"ancestor_record_offset":3196,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":5468,"parent_record_offset":5444,"ancestor_record_offset":3196,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":429,"record_offset":5976,"parent_record_offset":5040,"ancestor_record_offset":3196,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":429,"record_offset":3528,"parent_record_offset":3496,"ancestor_record_offset":3196,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":429,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3196,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":429,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":3196,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":429,"record_offset":7392,"parent_record_offset":7360,"ancestor_record_offset":3196,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":429,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":3196,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":3196,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":3196,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":6340,"parent_record_offset":6248,"ancestor_record_offset":3196,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":4256,"parent_record_offset":4148,"ancestor_record_offset":3196,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3196,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":3988,"parent_record_offset":3956,"ancestor_record_offset":3196,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":3196,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":4092,"parent_record_offset":4060,"ancestor_record_offset":3196,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3196,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3196,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3196,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":4584,"parent_record_offset":4060,"ancestor_record_offset":3196,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":3196,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3852,"parent_record_offset":3692,"ancestor_record_offset":3196,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":3196,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":3196,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3196,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":3196,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":3196,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":4148,"parent_record_offset":4120,"ancestor_record_offset":3196,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":4396,"parent_record_offset":4148,"ancestor_record_offset":3196,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":4368,"parent_record_offset":4148,"ancestor_record_offset":3196,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":4200,"parent_record_offset":4148,"ancestor_record_offset":3196,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":429,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":3196,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":5004,"parent_record_offset":4968,"ancestor_record_offset":3196,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":429,"record_offset":5116,"parent_record_offset":5076,"ancestor_record_offset":3196,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":5280,"parent_record_offset":5076,"ancestor_record_offset":3196,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":5548,"parent_record_offset":5280,"ancestor_record_offset":3196,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":429,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":3196,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":429,"record_offset":5176,"parent_record_offset":5144,"ancestor_record_offset":3196,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":5076,"parent_record_offset":5040,"ancestor_record_offset":3196,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":5040,"parent_record_offset":5004,"ancestor_record_offset":3196,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::animation::signifier_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":429,"record_offset":8216,"rva":6024320,"va":5374733440}
// 编译实例 RVAs: 0x5bec80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8574262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e836d58200","size":5,"soft_fail":false,"target_va":5383308144,"va":5374733877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28243867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e89bf7ae01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374734816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28244307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e853f9ae01","size":5,"soft_fail":false,"target_va":5402979152,"va":5374734840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28244537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e839faae01","size":5,"soft_fail":false,"target_va":5402979344,"va":5374734802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e817efffff","size":5,"soft_fail":false,"target_va":5374730624,"va":5374734948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28246197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e8b500af01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374733926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e875f5ffff","size":5,"soft_fail":false,"target_va":5374730816,"va":5374733510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28247071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e81f04af01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374733580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1622817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e821c31800","size":5,"soft_fail":false,"target_va":5376356912,"va":5374734090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1626698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e84ad21800","size":5,"soft_fail":false,"target_va":5376360560,"va":5374733857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1622658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374733440,"raw_hex":"e882c21800","size":5,"soft_fail":false,"target_va":5376356144,"va":5374733481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-4168","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374733440,"raw_hex":"e9b8efffff","size":5,"soft_fail":false,"target_va":5374730624,"va":5374734787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":429,"record_offset":12788,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":11236,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":429,"record_offset":11036,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::origin
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":429,"record_offset":12844,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":24861,"source_line":196,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":9984,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":8396,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":8972,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":429,"record_offset":12920,"parent_record_offset":12880,"ancestor_record_offset":8216,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":429,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":8216,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":12512,"parent_record_offset":12484,"ancestor_record_offset":8216,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":11640,"parent_record_offset":11608,"ancestor_record_offset":8216,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":11524,"parent_record_offset":11500,"ancestor_record_offset":8216,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":12604,"parent_record_offset":12484,"ancestor_record_offset":8216,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":12688,"parent_record_offset":12484,"ancestor_record_offset":8216,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":11608,"parent_record_offset":11500,"ancestor_record_offset":8216,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":429,"record_offset":11668,"parent_record_offset":11640,"ancestor_record_offset":8216,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":8216,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":8216,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":429,"record_offset":11264,"parent_record_offset":11236,"ancestor_record_offset":8216,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":429,"record_offset":11064,"parent_record_offset":11036,"ancestor_record_offset":8216,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":429,"record_offset":12484,"parent_record_offset":11264,"ancestor_record_offset":8216,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":429,"record_offset":11500,"parent_record_offset":11264,"ancestor_record_offset":8216,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":429,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":8216,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":8216,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":8216,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":8216,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":11828,"parent_record_offset":11764,"ancestor_record_offset":8216,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":429,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":8216,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":12076,"parent_record_offset":11908,"ancestor_record_offset":8216,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":11320,"parent_record_offset":11264,"ancestor_record_offset":8216,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":8216,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":8216,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":8216,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":429,"record_offset":11416,"parent_record_offset":11264,"ancestor_record_offset":8216,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":429,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":8216,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":429,"record_offset":12716,"parent_record_offset":12688,"ancestor_record_offset":8216,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10388,"parent_record_offset":10360,"ancestor_record_offset":8216,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":8216,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10416,"parent_record_offset":10388,"ancestor_record_offset":8216,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10328,"parent_record_offset":10296,"ancestor_record_offset":8216,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10360,"parent_record_offset":10328,"ancestor_record_offset":8216,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10532,"parent_record_offset":10360,"ancestor_record_offset":8216,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":429,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":8216,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":429,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":8216,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":429,"record_offset":8608,"parent_record_offset":8512,"ancestor_record_offset":8216,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":10220,"parent_record_offset":10192,"ancestor_record_offset":8216,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":8216,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":9568,"parent_record_offset":9108,"ancestor_record_offset":8216,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":429,"record_offset":10268,"parent_record_offset":10092,"ancestor_record_offset":8216,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":429,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":8216,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":10596,"parent_record_offset":10296,"ancestor_record_offset":8216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":429,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":8216,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":8216,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":8216,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":429,"record_offset":9528,"parent_record_offset":9496,"ancestor_record_offset":8216,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":429,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":8216,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":429,"record_offset":10996,"parent_record_offset":10056,"ancestor_record_offset":8216,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":429,"record_offset":8544,"parent_record_offset":8512,"ancestor_record_offset":8216,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":429,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":8216,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":429,"record_offset":12948,"parent_record_offset":12920,"ancestor_record_offset":8216,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":429,"record_offset":8572,"parent_record_offset":8544,"ancestor_record_offset":8216,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":429,"record_offset":12976,"parent_record_offset":12948,"ancestor_record_offset":8216,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":429,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":8216,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":8216,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":8216,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":11352,"parent_record_offset":11264,"ancestor_record_offset":8216,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":429,"record_offset":9272,"parent_record_offset":9164,"ancestor_record_offset":8216,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":8512,"parent_record_offset":8484,"ancestor_record_offset":8216,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":429,"record_offset":9004,"parent_record_offset":8972,"ancestor_record_offset":8216,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":8216,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":429,"record_offset":9108,"parent_record_offset":9076,"ancestor_record_offset":8216,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":8216,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8216,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":8216,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":429,"record_offset":9600,"parent_record_offset":9076,"ancestor_record_offset":8216,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8708,"parent_record_offset":8676,"ancestor_record_offset":8216,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8868,"parent_record_offset":8708,"ancestor_record_offset":8216,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":8216,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8428,"parent_record_offset":8396,"ancestor_record_offset":8216,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":8216,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8484,"parent_record_offset":8452,"ancestor_record_offset":8216,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":429,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":8216,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":8216,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":9412,"parent_record_offset":9164,"ancestor_record_offset":8216,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":9384,"parent_record_offset":9164,"ancestor_record_offset":8216,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":429,"record_offset":9216,"parent_record_offset":9164,"ancestor_record_offset":8216,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::WidgetVolumeShape::get_origin
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":429,"record_offset":12880,"parent_record_offset":12844,"ancestor_record_offset":8216,"inlinee":23533,"source_line":302,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":429,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":8216,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":10020,"parent_record_offset":9984,"ancestor_record_offset":8216,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":429,"record_offset":10132,"parent_record_offset":10092,"ancestor_record_offset":8216,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":10296,"parent_record_offset":10092,"ancestor_record_offset":8216,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":429,"record_offset":10568,"parent_record_offset":10296,"ancestor_record_offset":8216,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":429,"record_offset":10160,"parent_record_offset":10132,"ancestor_record_offset":8216,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":429,"record_offset":10192,"parent_record_offset":10160,"ancestor_record_offset":8216,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":10092,"parent_record_offset":10056,"ancestor_record_offset":8216,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00429.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":429,"record_offset":10056,"parent_record_offset":10020,"ancestor_record_offset":8216,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00429.jsonl.gz"}
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
// 完整结构及原始类型记录: animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::animation::signifier_animation
// 信息 ID: 6b4b0adca5e94618cfcc39f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 707f5472713a8ad2baa199d0: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)> @ 0xcde85b -> 0x5bec80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::ui::signifiers::SignifierAnimation>,ref_mut$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierId> > >,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>)> > @ 0xe78736 -> 0x5bec80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::animation::signifier_animation_t
// 信息 ID: 73a1a4f214e4a485f9c9413c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e279b7af7ce4475a3647486: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0xcde9fb -> 0x5be670 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::ui::signifiers::SignifierAnimation>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> > >,enum2$<core::option::Option<ref_mut$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xe829c6 -> 0x5be670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
