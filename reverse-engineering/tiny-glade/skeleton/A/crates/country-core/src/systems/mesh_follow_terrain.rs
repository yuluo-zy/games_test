// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/mesh_follow_terrain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bd184c49e1b0df55504ef745d119b33acc5c97c7aec5c14646c688e32125f76e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh_follow_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 471, 1395
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::mesh_follow_terrain::mesh_follow_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":471,"record_offset":16856,"rva":6673040,"va":5375382160}
// 编译实例 RVAs: 0x65d290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_follow_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-647008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8a020f6ff","size":5,"soft_fail":false,"target_va":5374736096,"va":5375383099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7edc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_follow_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7477148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e89c177200","size":5,"soft_fail":false,"target_va":5382860224,"va":5375383071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_follow_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7925543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e827ef7800","size":5,"soft_fail":false,"target_va":5383308144,"va":5375382596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27595509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8f512a501","size":5,"soft_fail":false,"target_va":5402978688,"va":5375383174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27595747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8e313a501","size":5,"soft_fail":false,"target_va":5402978912,"va":5375383160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e857d7ffff","size":5,"soft_fail":false,"target_va":5375372880,"va":5375383284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27597478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8a61aa501","size":5,"soft_fail":false,"target_va":5402980128,"va":5375382645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e83adcffff","size":5,"soft_fail":false,"target_va":5375373072,"va":5375382225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27598349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e80d1ea501","size":5,"soft_fail":false,"target_va":5402980656,"va":5375382302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-815403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8d58ef3ff","size":5,"soft_fail":false,"target_va":5374567616,"va":5375383014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9185905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e8712a8c00","size":5,"soft_fail":false,"target_va":5384568944,"va":5375383034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t974099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e813dd0e00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375382808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t977979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e83bec0e00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375382576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t973934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375382160,"raw_hex":"e86edc0e00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375382205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-10261","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375382160,"raw_hex":"e9ebd7ffff","size":5,"soft_fail":false,"target_va":5375372880,"va":5375383136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":471,"record_offset":20224,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":471,"record_offset":19856,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":471,"record_offset":19656,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":471,"record_offset":20372,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":471,"record_offset":20316,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":20280,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":18624,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":17036,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":17612,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":16856,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":471,"record_offset":20248,"parent_record_offset":20224,"ancestor_record_offset":16856,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":471,"record_offset":20160,"parent_record_offset":19880,"ancestor_record_offset":16856,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":471,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":16856,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":471,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":16856,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":471,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":16856,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":471,"record_offset":19684,"parent_record_offset":19656,"ancestor_record_offset":16856,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":471,"record_offset":20132,"parent_record_offset":19880,"ancestor_record_offset":16856,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":471,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":16856,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":471,"record_offset":20048,"parent_record_offset":19880,"ancestor_record_offset":16856,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":19008,"parent_record_offset":18980,"ancestor_record_offset":16856,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":19060,"parent_record_offset":19036,"ancestor_record_offset":16856,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":19036,"parent_record_offset":19008,"ancestor_record_offset":16856,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":18948,"parent_record_offset":18920,"ancestor_record_offset":16856,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":18980,"parent_record_offset":18948,"ancestor_record_offset":16856,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":19152,"parent_record_offset":18980,"ancestor_record_offset":16856,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":19264,"parent_record_offset":19240,"ancestor_record_offset":16856,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":471,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":16856,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":471,"record_offset":20420,"parent_record_offset":20396,"ancestor_record_offset":16856,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":471,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":16856,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":471,"record_offset":17248,"parent_record_offset":17152,"ancestor_record_offset":16856,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":18844,"parent_record_offset":18816,"ancestor_record_offset":16856,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":18208,"parent_record_offset":17748,"ancestor_record_offset":16856,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":18892,"parent_record_offset":18720,"ancestor_record_offset":16856,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":471,"record_offset":17560,"parent_record_offset":17532,"ancestor_record_offset":16856,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":19216,"parent_record_offset":18920,"ancestor_record_offset":16856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":471,"record_offset":19804,"parent_record_offset":19780,"ancestor_record_offset":16856,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":20012,"parent_record_offset":19988,"ancestor_record_offset":16856,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":17984,"parent_record_offset":17960,"ancestor_record_offset":16856,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":16856,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":18168,"parent_record_offset":18136,"ancestor_record_offset":16856,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":16856,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":471,"record_offset":19616,"parent_record_offset":18688,"ancestor_record_offset":16856,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":471,"record_offset":17184,"parent_record_offset":17152,"ancestor_record_offset":16856,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":471,"record_offset":19756,"parent_record_offset":19732,"ancestor_record_offset":16856,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":471,"record_offset":17212,"parent_record_offset":17184,"ancestor_record_offset":16856,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":471,"record_offset":19780,"parent_record_offset":19756,"ancestor_record_offset":16856,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":19988,"parent_record_offset":19924,"ancestor_record_offset":16856,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":16856,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":18264,"parent_record_offset":18240,"ancestor_record_offset":16856,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":19924,"parent_record_offset":19880,"ancestor_record_offset":16856,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":17912,"parent_record_offset":17804,"ancestor_record_offset":16856,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":471,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":16856,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":471,"record_offset":17644,"parent_record_offset":17612,"ancestor_record_offset":16856,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":471,"record_offset":17776,"parent_record_offset":17748,"ancestor_record_offset":16856,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":471,"record_offset":17748,"parent_record_offset":17716,"ancestor_record_offset":16856,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":16856,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":16856,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":16856,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":18240,"parent_record_offset":17716,"ancestor_record_offset":16856,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17348,"parent_record_offset":17316,"ancestor_record_offset":16856,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17508,"parent_record_offset":17348,"ancestor_record_offset":16856,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17316,"parent_record_offset":17292,"ancestor_record_offset":16856,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17068,"parent_record_offset":17036,"ancestor_record_offset":16856,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":16856,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17124,"parent_record_offset":17092,"ancestor_record_offset":16856,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":16856,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":17804,"parent_record_offset":17776,"ancestor_record_offset":16856,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":18052,"parent_record_offset":17804,"ancestor_record_offset":16856,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":18024,"parent_record_offset":17804,"ancestor_record_offset":16856,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":17856,"parent_record_offset":17804,"ancestor_record_offset":16856,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":18656,"parent_record_offset":18624,"ancestor_record_offset":16856,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":471,"record_offset":18756,"parent_record_offset":18720,"ancestor_record_offset":16856,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":18920,"parent_record_offset":18720,"ancestor_record_offset":16856,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":19188,"parent_record_offset":18920,"ancestor_record_offset":16856,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":471,"record_offset":18784,"parent_record_offset":18756,"ancestor_record_offset":16856,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":471,"record_offset":18816,"parent_record_offset":18784,"ancestor_record_offset":16856,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":18720,"parent_record_offset":18688,"ancestor_record_offset":16856,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":18688,"parent_record_offset":18656,"ancestor_record_offset":16856,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFD88
// 首条位置: {"module":1395,"inlinee":64904}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1395,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":1508,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1395,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1508,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1395,"record_offset":1884,"parent_record_offset":1828,"ancestor_record_offset":1508,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1395,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1508,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1395,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1508,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1395,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1508,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1395,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1508,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1395,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1508,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1395,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":1508,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1395,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1508,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1395,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":1508,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1395,"record_offset":1576,"parent_record_offset":1540,"ancestor_record_offset":1508,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1976,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1804,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1508,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1748,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":2336,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1692,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1395,"record_offset":1920,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":1395,"record_offset":2012,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1395,"record_offset":2188,"parent_record_offset":2072,"ancestor_record_offset":1508,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1395,"record_offset":2040,"parent_record_offset":1612,"ancestor_record_offset":1508,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/01395.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1395,"record_offset":1612,"parent_record_offset":1576,"ancestor_record_offset":1508,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/01395.jsonl.gz"}
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
// 完整结构及原始类型记录: mesh_follow_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh_follow_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh_follow_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>

// ------------------------------------------------------------------
// 函数: country_core::systems::mesh_follow_terrain::mesh_follow_terrain
// 信息 ID: 82d93a11d96de0b92c4ced2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5070fd6c583d9d792e45262f: void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> > @ 0xc1f28 -> 0x65d290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> @ 0x1742dd -> 0x65d290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure$0
// 信息 ID: 3bd02c3a7816977e3a0bc239
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::mesh_follow_terrain::mesh_follow_terrain", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$0::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$0::positions::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$0::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$0::positions::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$0::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$0::positions::closure_env$0,core::iter::adapters::map::map_fold::closure_env$0<glam::f32::vec3::Vec3,array$<f32,3>,tuple$<>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<f32,3>,alloc::vec::impl$19::extend_trusted::closure_env$0<array$<f32,3>,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$0::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> > > > > >; module=1395, inline_record=1508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
