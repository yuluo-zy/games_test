// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/add_birdnests.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4a45448b41da924952e564249822a325a6b8d4bdffae0f6bdf441315588e719f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_birdnests.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3511
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::add_birdnests::add_birdnests
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3511,"record_offset":3060,"rva":28125296,"va":5396834416}
// 编译实例 RVAs: 0x1ad2870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25526920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8787d7afe","size":5,"soft_fail":false,"target_va":5371308800,"va":5396835715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6142856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e888bb5d00","size":5,"soft_fail":false,"target_va":5402978688,"va":5396835827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8c8f5ffff","size":5,"soft_fail":false,"target_va":5396833136,"va":5396835747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6145189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8a5c45d00","size":5,"soft_fail":false,"target_va":5402980128,"va":5396834934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e84bfbffff","size":5,"soft_fail":false,"target_va":5396833328,"va":5396834528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6146046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8fec75d00","size":5,"soft_fail":false,"target_va":5402980656,"va":5396834605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21070830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8127cbefe","size":5,"soft_fail":false,"target_va":5375764720,"va":5396835545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20478191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e81187c7fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5396835098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20474310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e83a96c7fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5396834865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20478369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e85f86c7fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5396834508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h93c6532eedf5b01e (.llvm.12250732642055207085)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17912250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e846aeeefe","size":5,"soft_fail":false,"target_va":5378922640,"va":5396834885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18866700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e8f41de0fe","size":5,"soft_fail":false,"target_va":5377968928,"va":5396835623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18874091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396834416,"raw_hex":"e81501e0fe","size":5,"soft_fail":false,"target_va":5377961504,"va":5396835590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3511,"record_offset":6032,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3511,"record_offset":7048,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3511,"record_offset":6452,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3511,"record_offset":6256,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":5872,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3511,"record_offset":6864,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3511,"record_offset":6168,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":4824,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":3236,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":3812,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3511,"record_offset":6104,"parent_record_offset":6072,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3511,"record_offset":6072,"parent_record_offset":6032,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3511,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3511,"record_offset":6764,"parent_record_offset":6480,"ancestor_record_offset":3060,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3511,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":3060,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3511,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":3060,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3511,"record_offset":6480,"parent_record_offset":6452,"ancestor_record_offset":3060,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3511,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":3060,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3511,"record_offset":6736,"parent_record_offset":6480,"ancestor_record_offset":3060,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3511,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":3060,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3511,"record_offset":6652,"parent_record_offset":6480,"ancestor_record_offset":3060,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5196,"parent_record_offset":5164,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5368,"parent_record_offset":5196,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3511,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3511,"record_offset":5300,"parent_record_offset":5276,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3511,"record_offset":3448,"parent_record_offset":3352,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3511,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3511,"record_offset":6920,"parent_record_offset":6888,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3511,"record_offset":5060,"parent_record_offset":5032,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3511,"record_offset":4408,"parent_record_offset":3948,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3511,"record_offset":5108,"parent_record_offset":4932,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3511,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3511,"record_offset":5432,"parent_record_offset":5136,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3511,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3511,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":3060,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3511,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3511,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3511,"record_offset":4368,"parent_record_offset":4336,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3511,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3511,"record_offset":5832,"parent_record_offset":4896,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3511,"record_offset":3384,"parent_record_offset":3352,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3511,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3511,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3511,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":5972,"parent_record_offset":5872,"ancestor_record_offset":3060,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":6528,"parent_record_offset":6480,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3511,"record_offset":4112,"parent_record_offset":4004,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3511,"record_offset":3352,"parent_record_offset":3324,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3511,"record_offset":3844,"parent_record_offset":3812,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3511,"record_offset":3976,"parent_record_offset":3948,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3511,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3511,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3511,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3511,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3511,"record_offset":4440,"parent_record_offset":3916,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3708,"parent_record_offset":3548,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3268,"parent_record_offset":3236,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3324,"parent_record_offset":3292,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3511,"record_offset":3292,"parent_record_offset":3268,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3511,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3511,"record_offset":4252,"parent_record_offset":4004,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3511,"record_offset":4224,"parent_record_offset":4004,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3511,"record_offset":4056,"parent_record_offset":4004,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3511,"record_offset":6976,"parent_record_offset":6888,"ancestor_record_offset":3060,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3511,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":3060,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":4860,"parent_record_offset":4824,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3511,"record_offset":4972,"parent_record_offset":4932,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":5136,"parent_record_offset":4932,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3511,"record_offset":5404,"parent_record_offset":5136,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3511,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":3060,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3511,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3511,"record_offset":5032,"parent_record_offset":5000,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3511,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03511.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3511,"record_offset":4896,"parent_record_offset":4860,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03511.jsonl.gz"}
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
// 完整结构及原始类型记录: add_birdnests.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_birdnests.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_birdnests.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::add_birdnests::add_birdnests
// 信息 ID: 4787c08bf5059a55fc5e4443
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9f6988cca5a30b051860c810: void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>)> > @ 0x14c166a -> 0x1ad2870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<system_clutter::clutter::load_clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>)> @ 0x1ae1ca3 -> 0x1ad2870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
