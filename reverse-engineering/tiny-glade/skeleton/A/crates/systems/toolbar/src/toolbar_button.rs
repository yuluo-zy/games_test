// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/toolbar_button.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ee961df7b40f4ed7b2f558e04ad2a0479ee7c1ef9a6d719ed2e27e9b617a8b45
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_button.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3641
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::toolbar_button_keep_focus
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3641,"record_offset":1880,"rva":28668144,"va":5397377264}
// 编译实例 RVAs: 0x1b570f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5599783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e827725500","size":5,"soft_fail":false,"target_va":5402978688,"va":5397378900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5601183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e89f775500","size":5,"soft_fail":false,"target_va":5402980128,"va":5397378940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e87bfbffff","size":5,"soft_fail":false,"target_va":5397376176,"va":5397377328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5603246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e8ae7f5500","size":5,"soft_fail":false,"target_va":5402980656,"va":5397377405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21020942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e8f23ebffe","size":5,"soft_fail":false,"target_va":5376356912,"va":5397377849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21017109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e8eb4dbffe","size":5,"soft_fail":false,"target_va":5376360560,"va":5397377664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21021125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e83b3ebffe","size":5,"soft_fail":false,"target_va":5376357744,"va":5397378864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21021163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e8153ebffe","size":5,"soft_fail":false,"target_va":5376356144,"va":5397377302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h6a28a4a69af7475d (.llvm.7250520506814577795)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17755701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397377264,"raw_hex":"e8cb11f1fe","size":5,"soft_fail":false,"target_va":5379621984,"va":5397377680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":5236,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":5320,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":5596,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":4836,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":4636,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3641,"record_offset":5292,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":6344,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":3640,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":2052,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":2628,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":1880,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":1880,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":5384,"parent_record_offset":5344,"ancestor_record_offset":1880,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":5624,"parent_record_offset":5596,"ancestor_record_offset":1880,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":1880,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":1880,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":1880,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":4864,"parent_record_offset":4836,"ancestor_record_offset":1880,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":4664,"parent_record_offset":4636,"ancestor_record_offset":1880,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":4908,"parent_record_offset":4864,"ancestor_record_offset":1880,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":5200,"parent_record_offset":4864,"ancestor_record_offset":1880,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":1880,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":5116,"parent_record_offset":4864,"ancestor_record_offset":1880,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3641,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":1880,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3641,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":1880,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":1880,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":1880,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":1880,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":1880,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":1880,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":4160,"parent_record_offset":3988,"ancestor_record_offset":1880,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3641,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":1880,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3641,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":1880,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3641,"record_offset":5456,"parent_record_offset":5408,"ancestor_record_offset":1880,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3641,"record_offset":2264,"parent_record_offset":2168,"ancestor_record_offset":1880,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":1880,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":3224,"parent_record_offset":2764,"ancestor_record_offset":1880,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":3908,"parent_record_offset":3736,"ancestor_record_offset":1880,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":5520,"parent_record_offset":5408,"ancestor_record_offset":1880,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":5656,"parent_record_offset":5624,"ancestor_record_offset":1880,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3641,"record_offset":2576,"parent_record_offset":2548,"ancestor_record_offset":1880,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":4224,"parent_record_offset":3936,"ancestor_record_offset":1880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":1880,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":1880,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":1880,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1880,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":3184,"parent_record_offset":3152,"ancestor_record_offset":1880,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":1880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1880,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":1880,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":1880,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3641,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":1880,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":5056,"parent_record_offset":4992,"ancestor_record_offset":1880,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1880,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":1880,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":4992,"parent_record_offset":4864,"ancestor_record_offset":1880,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":2928,"parent_record_offset":2820,"ancestor_record_offset":1880,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3641,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1880,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3641,"record_offset":2660,"parent_record_offset":2628,"ancestor_record_offset":1880,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3641,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":1880,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3641,"record_offset":2764,"parent_record_offset":2732,"ancestor_record_offset":1880,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3641,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":1880,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3641,"record_offset":2732,"parent_record_offset":2708,"ancestor_record_offset":1880,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3641,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1880,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3641,"record_offset":3256,"parent_record_offset":2732,"ancestor_record_offset":1880,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2364,"parent_record_offset":2332,"ancestor_record_offset":1880,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2524,"parent_record_offset":2364,"ancestor_record_offset":1880,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1880,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2084,"parent_record_offset":2052,"ancestor_record_offset":1880,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1880,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2140,"parent_record_offset":2108,"ancestor_record_offset":1880,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3641,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1880,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3641,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":1880,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3641,"record_offset":3068,"parent_record_offset":2820,"ancestor_record_offset":1880,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3641,"record_offset":3040,"parent_record_offset":2820,"ancestor_record_offset":1880,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3641,"record_offset":2872,"parent_record_offset":2820,"ancestor_record_offset":1880,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":1880,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":1880,"inlinee":7257,"source_line":601,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":6956,"parent_record_offset":6696,"ancestor_record_offset":1880,"inlinee":7265,"source_line":602,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3641,"record_offset":3772,"parent_record_offset":3736,"ancestor_record_offset":1880,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":3936,"parent_record_offset":3736,"ancestor_record_offset":1880,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3641,"record_offset":4196,"parent_record_offset":3936,"ancestor_record_offset":1880,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3641,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":1880,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3641,"record_offset":3832,"parent_record_offset":3800,"ancestor_record_offset":1880,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3641,"record_offset":3736,"parent_record_offset":3704,"ancestor_record_offset":1880,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3641,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":1880,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_app_mode_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3641,"record_offset":8740,"rva":28669968,"va":5397379088}
// 编译实例 RVAs: 0x1b57810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5538403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397379088,"raw_hex":"e863825400","size":5,"soft_fail":false,"target_va":5402919072,"va":5397380664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26072729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397379088,"raw_hex":"e8672972fe","size":5,"soft_fail":false,"target_va":5371306912,"va":5397379636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20135052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397379088,"raw_hex":"e874c3ccfe","size":5,"soft_fail":false,"target_va":5377245296,"va":5397380343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5591336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397379088,"raw_hex":"e828515500","size":5,"soft_fail":false,"target_va":5402972080,"va":5397380739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20133978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397379088,"raw_hex":"e8a6c7ccfe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397380501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":12936,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":8912,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":9452,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":11668,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":9732,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":12096,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13056,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13148,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":10044,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":11900,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mode_manager::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":3641,"record_offset":12868,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":23523,"source_line":45,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":13432,"parent_record_offset":13404,"ancestor_record_offset":8740,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":13404,"parent_record_offset":13368,"ancestor_record_offset":8740,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":9604,"parent_record_offset":9572,"ancestor_record_offset":8740,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":13368,"parent_record_offset":13336,"ancestor_record_offset":8740,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":9572,"parent_record_offset":9524,"ancestor_record_offset":8740,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":13484,"parent_record_offset":13288,"ancestor_record_offset":8740,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":9524,"parent_record_offset":9480,"ancestor_record_offset":8740,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":13336,"parent_record_offset":13288,"ancestor_record_offset":8740,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":13652,"parent_record_offset":13288,"ancestor_record_offset":8740,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3641,"record_offset":10256,"parent_record_offset":10112,"ancestor_record_offset":8740,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":8740,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":9672,"parent_record_offset":9452,"ancestor_record_offset":8740,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":8740,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3641,"record_offset":10112,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":8740,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":9480,"parent_record_offset":9452,"ancestor_record_offset":8740,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":8976,"parent_record_offset":8936,"ancestor_record_offset":8740,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":11704,"parent_record_offset":11668,"ancestor_record_offset":8740,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":8740,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":8740,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":8740,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":8740,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":8740,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":8740,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":8740,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":12124,"parent_record_offset":12096,"ancestor_record_offset":8740,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":8740,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":8740,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":10928,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":10532,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":45933,"source_line":179,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":12412,"parent_record_offset":12124,"ancestor_record_offset":8740,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3641,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":8740,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":10656,"parent_record_offset":10632,"ancestor_record_offset":8740,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":8740,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":8740,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":8740,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":10860,"parent_record_offset":10580,"ancestor_record_offset":8740,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":12384,"parent_record_offset":12124,"ancestor_record_offset":8740,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":10608,"parent_record_offset":10580,"ancestor_record_offset":8740,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":12780,"parent_record_offset":12752,"ancestor_record_offset":8740,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":10504,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13212,"parent_record_offset":13180,"ancestor_record_offset":8740,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13088,"parent_record_offset":13056,"ancestor_record_offset":8740,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13244,"parent_record_offset":13212,"ancestor_record_offset":8740,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":13180,"parent_record_offset":13148,"ancestor_record_offset":8740,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3641,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":8740,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":13684,"parent_record_offset":13244,"ancestor_record_offset":8740,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":13288,"parent_record_offset":13244,"ancestor_record_offset":8740,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3641,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":8740,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3641,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":8740,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3641,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":8740,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3641,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":8740,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3641,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":8740,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":8740,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":11740,"parent_record_offset":11704,"ancestor_record_offset":8740,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":9784,"parent_record_offset":9756,"ancestor_record_offset":8740,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":8740,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3641,"record_offset":9044,"parent_record_offset":9000,"ancestor_record_offset":8740,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3641,"record_offset":10200,"parent_record_offset":10112,"ancestor_record_offset":8740,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":11772,"parent_record_offset":11740,"ancestor_record_offset":8740,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":8740,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":8740,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":8740,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":11828,"parent_record_offset":11740,"ancestor_record_offset":8740,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":9864,"parent_record_offset":9784,"ancestor_record_offset":8740,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":11860,"parent_record_offset":11704,"ancestor_record_offset":8740,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3641,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":8740,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":8740,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":13568,"parent_record_offset":13288,"ancestor_record_offset":8740,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":8740,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":12212,"parent_record_offset":12176,"ancestor_record_offset":8740,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":9636,"parent_record_offset":9524,"ancestor_record_offset":8740,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":8740,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3641,"record_offset":10476,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":9108,"parent_record_offset":9000,"ancestor_record_offset":8740,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":12560,"parent_record_offset":12536,"ancestor_record_offset":8740,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":9296,"parent_record_offset":9000,"ancestor_record_offset":8740,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":8740,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":8740,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":8740,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":9144,"parent_record_offset":9108,"ancestor_record_offset":8740,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3641,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":8740,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":8740,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":9940,"parent_record_offset":9888,"ancestor_record_offset":8740,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::mode_manager::impl$26::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":3641,"record_offset":12900,"parent_record_offset":12868,"ancestor_record_offset":8740,"inlinee":22145,"source_line":37,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":10288,"parent_record_offset":10068,"ancestor_record_offset":8740,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":8740,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":8740,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":8740,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3641,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":8740,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::switch_toolbar_category_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3641,"record_offset":14188,"rva":28671632,"va":5397380752}
// 编译实例 RVAs: 0x1b57e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5536895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397380752,"raw_hex":"e87f7c5400","size":5,"soft_fail":false,"target_va":5402919072,"va":5397382172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26076395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397380752,"raw_hex":"e8151b72fe","size":5,"soft_fail":false,"target_va":5371304880,"va":5397381270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20136735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397380752,"raw_hex":"e8e1bcccfe","size":5,"soft_fail":false,"target_va":5377245296,"va":5397382026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5589829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397380752,"raw_hex":"e8454b5500","size":5,"soft_fail":false,"target_va":5402972080,"va":5397382246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20135543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397380752,"raw_hex":"e889c1ccfe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397382066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":18156,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":14368,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":14908,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":16688,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":15184,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":17096,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":19196,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":18220,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":15496,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":16900,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":3641,"record_offset":18124,"parent_record_offset":14188,"ancestor_record_offset":14188,"inlinee":25490,"source_line":33,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":18500,"parent_record_offset":18472,"ancestor_record_offset":14188,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":18472,"parent_record_offset":18436,"ancestor_record_offset":14188,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":15056,"parent_record_offset":15024,"ancestor_record_offset":14188,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":14188,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":15024,"parent_record_offset":14980,"ancestor_record_offset":14188,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":18552,"parent_record_offset":18356,"ancestor_record_offset":14188,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":14980,"parent_record_offset":14936,"ancestor_record_offset":14188,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":18404,"parent_record_offset":18356,"ancestor_record_offset":14188,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":18720,"parent_record_offset":18356,"ancestor_record_offset":14188,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3641,"record_offset":15704,"parent_record_offset":15560,"ancestor_record_offset":14188,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":14188,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":15124,"parent_record_offset":14908,"ancestor_record_offset":14188,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":18184,"parent_record_offset":18156,"ancestor_record_offset":14188,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3641,"record_offset":15560,"parent_record_offset":15520,"ancestor_record_offset":14188,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":14188,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":14188,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":14432,"parent_record_offset":14392,"ancestor_record_offset":14188,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":16728,"parent_record_offset":16688,"ancestor_record_offset":14188,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":15208,"parent_record_offset":15184,"ancestor_record_offset":14188,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":14188,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":14188,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":14188,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":17232,"parent_record_offset":17204,"ancestor_record_offset":14188,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":14188,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":14188,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":17132,"parent_record_offset":17096,"ancestor_record_offset":14188,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":14188,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":14188,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":17736,"parent_record_offset":17132,"ancestor_record_offset":14188,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":15980,"parent_record_offset":15520,"ancestor_record_offset":14188,"inlinee":45933,"source_line":179,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":17204,"parent_record_offset":17132,"ancestor_record_offset":14188,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3641,"record_offset":16332,"parent_record_offset":16308,"ancestor_record_offset":14188,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":16104,"parent_record_offset":16080,"ancestor_record_offset":14188,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":14188,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":16152,"parent_record_offset":16128,"ancestor_record_offset":14188,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":14188,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":16308,"parent_record_offset":16028,"ancestor_record_offset":14188,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":18084,"parent_record_offset":17132,"ancestor_record_offset":14188,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":14188,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":14188,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":15952,"parent_record_offset":15520,"ancestor_record_offset":14188,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":18284,"parent_record_offset":18252,"ancestor_record_offset":14188,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":19228,"parent_record_offset":19196,"ancestor_record_offset":14188,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":18316,"parent_record_offset":18284,"ancestor_record_offset":14188,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":18252,"parent_record_offset":18220,"ancestor_record_offset":14188,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3641,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":14188,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":18752,"parent_record_offset":18316,"ancestor_record_offset":14188,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":18356,"parent_record_offset":18316,"ancestor_record_offset":14188,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3641,"record_offset":15768,"parent_record_offset":15736,"ancestor_record_offset":14188,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3641,"record_offset":19252,"parent_record_offset":19228,"ancestor_record_offset":14188,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3641,"record_offset":15672,"parent_record_offset":15648,"ancestor_record_offset":14188,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3641,"record_offset":18684,"parent_record_offset":18660,"ancestor_record_offset":14188,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":15260,"parent_record_offset":15236,"ancestor_record_offset":14188,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":16768,"parent_record_offset":16728,"ancestor_record_offset":14188,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":15236,"parent_record_offset":15208,"ancestor_record_offset":14188,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":14188,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3641,"record_offset":14500,"parent_record_offset":14456,"ancestor_record_offset":14188,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3641,"record_offset":15648,"parent_record_offset":15560,"ancestor_record_offset":14188,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":16796,"parent_record_offset":16768,"ancestor_record_offset":14188,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":14188,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":16176,"parent_record_offset":16152,"ancestor_record_offset":14188,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":16128,"parent_record_offset":16104,"ancestor_record_offset":14188,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":15316,"parent_record_offset":15236,"ancestor_record_offset":14188,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":16616,"parent_record_offset":16508,"ancestor_record_offset":14188,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":16856,"parent_record_offset":16728,"ancestor_record_offset":14188,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3641,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":14188,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14188,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":18636,"parent_record_offset":18356,"ancestor_record_offset":14188,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":14188,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":17888,"parent_record_offset":17832,"ancestor_record_offset":14188,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":15088,"parent_record_offset":14980,"ancestor_record_offset":14188,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":18660,"parent_record_offset":18636,"ancestor_record_offset":14188,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3641,"record_offset":15924,"parent_record_offset":15520,"ancestor_record_offset":14188,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":14564,"parent_record_offset":14456,"ancestor_record_offset":14188,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":17372,"parent_record_offset":17348,"ancestor_record_offset":14188,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":14752,"parent_record_offset":14456,"ancestor_record_offset":14188,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":15616,"parent_record_offset":15592,"ancestor_record_offset":14188,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":14188,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":14188,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":14600,"parent_record_offset":14564,"ancestor_record_offset":14188,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3641,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14188,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":14188,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":15392,"parent_record_offset":15340,"ancestor_record_offset":14188,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":15736,"parent_record_offset":15520,"ancestor_record_offset":14188,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":14188,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":14188,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3641,"record_offset":15796,"parent_record_offset":15768,"ancestor_record_offset":14188,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3641,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":14188,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_button::tod_v2_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3641,"record_offset":19292,"rva":28673136,"va":5397382256}
// 编译实例 RVAs: 0x1b58470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5536309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397382256,"raw_hex":"e8357a5400","size":5,"soft_fail":false,"target_va":5402919072,"va":5397382758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20137420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397382256,"raw_hex":"e834baccfe","size":5,"soft_fail":false,"target_va":5377245296,"va":5397382711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5289897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397382256,"raw_hex":"e85748afff","size":5,"soft_fail":false,"target_va":5392092768,"va":5397382660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5588576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397382256,"raw_hex":"e860465500","size":5,"soft_fail":false,"target_va":5402972080,"va":5397383499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20136496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397382256,"raw_hex":"e8d0bdccfe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397383019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":21392,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":23436,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":19456,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":22196,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":20960,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":22720,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":21456,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":19992,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":21328,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3641,"record_offset":22524,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3641,"record_offset":21360,"parent_record_offset":19292,"ancestor_record_offset":19292,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":20444,"parent_record_offset":20416,"ancestor_record_offset":19292,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":20416,"parent_record_offset":20384,"ancestor_record_offset":19292,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":19292,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3641,"record_offset":20384,"parent_record_offset":20356,"ancestor_record_offset":19292,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":20188,"parent_record_offset":20144,"ancestor_record_offset":19292,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":20496,"parent_record_offset":20144,"ancestor_record_offset":19292,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":20356,"parent_record_offset":20144,"ancestor_record_offset":19292,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3641,"record_offset":20328,"parent_record_offset":20144,"ancestor_record_offset":19292,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":19292,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3641,"record_offset":23460,"parent_record_offset":23436,"ancestor_record_offset":19292,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":19292,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":19520,"parent_record_offset":19480,"ancestor_record_offset":19292,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":22240,"parent_record_offset":22196,"ancestor_record_offset":19292,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3641,"record_offset":20988,"parent_record_offset":20960,"ancestor_record_offset":19292,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":23132,"parent_record_offset":23108,"ancestor_record_offset":19292,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3641,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":19292,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":19292,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":22596,"parent_record_offset":22572,"ancestor_record_offset":19292,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3641,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":19292,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3641,"record_offset":22548,"parent_record_offset":22524,"ancestor_record_offset":19292,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":23108,"parent_record_offset":22748,"ancestor_record_offset":19292,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3641,"record_offset":22796,"parent_record_offset":22748,"ancestor_record_offset":19292,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3641,"record_offset":23068,"parent_record_offset":23044,"ancestor_record_offset":19292,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":22872,"parent_record_offset":22848,"ancestor_record_offset":19292,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3641,"record_offset":22920,"parent_record_offset":22896,"ancestor_record_offset":19292,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":23044,"parent_record_offset":22820,"ancestor_record_offset":19292,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":23400,"parent_record_offset":22748,"ancestor_record_offset":19292,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":22848,"parent_record_offset":22820,"ancestor_record_offset":19292,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":19292,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3641,"record_offset":23316,"parent_record_offset":22748,"ancestor_record_offset":19292,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":20064,"parent_record_offset":20028,"ancestor_record_offset":19292,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":21492,"parent_record_offset":21456,"ancestor_record_offset":19292,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":20100,"parent_record_offset":20064,"ancestor_record_offset":19292,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3641,"record_offset":20028,"parent_record_offset":19992,"ancestor_record_offset":19292,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3641,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":19292,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":20528,"parent_record_offset":20100,"ancestor_record_offset":19292,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3641,"record_offset":20144,"parent_record_offset":20100,"ancestor_record_offset":19292,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3641,"record_offset":21520,"parent_record_offset":21492,"ancestor_record_offset":19292,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3641,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":19292,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":21056,"parent_record_offset":21028,"ancestor_record_offset":19292,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":22284,"parent_record_offset":22240,"ancestor_record_offset":19292,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":21028,"parent_record_offset":20988,"ancestor_record_offset":19292,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3641,"record_offset":21148,"parent_record_offset":21120,"ancestor_record_offset":19292,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3641,"record_offset":19588,"parent_record_offset":19544,"ancestor_record_offset":19292,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":22328,"parent_record_offset":22284,"ancestor_record_offset":19292,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":21084,"parent_record_offset":21056,"ancestor_record_offset":19292,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":19292,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":22396,"parent_record_offset":22284,"ancestor_record_offset":19292,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":21120,"parent_record_offset":21028,"ancestor_record_offset":19292,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":21836,"parent_record_offset":21708,"ancestor_record_offset":19292,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3641,"record_offset":22428,"parent_record_offset":22240,"ancestor_record_offset":19292,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3641,"record_offset":22992,"parent_record_offset":22968,"ancestor_record_offset":19292,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":19884,"parent_record_offset":19860,"ancestor_record_offset":19292,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3641,"record_offset":20244,"parent_record_offset":20144,"ancestor_record_offset":19292,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":21268,"parent_record_offset":21240,"ancestor_record_offset":19292,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3641,"record_offset":23228,"parent_record_offset":23192,"ancestor_record_offset":19292,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3641,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19292,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":19652,"parent_record_offset":19544,"ancestor_record_offset":19292,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":22944,"parent_record_offset":22920,"ancestor_record_offset":19292,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3641,"record_offset":19836,"parent_record_offset":19544,"ancestor_record_offset":19292,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":19292,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":22968,"parent_record_offset":22944,"ancestor_record_offset":19292,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":19292,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3641,"record_offset":19684,"parent_record_offset":19652,"ancestor_record_offset":19292,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3641,"record_offset":19860,"parent_record_offset":19836,"ancestor_record_offset":19292,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":21240,"parent_record_offset":21208,"ancestor_record_offset":19292,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03641.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3641,"record_offset":21208,"parent_record_offset":21148,"ancestor_record_offset":19292,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03641.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_button.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 32
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::EventReader", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::EventRegistry::register_event<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::EventRegistry::register_event", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)", "void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>", "tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > > >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >),void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)>,tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::UiInteractionState>,ref$<toolbar::resouces_and_components::HideIfMenuIsOpen>,enum2$<core::option::Option<ref$<country_core::components::UiTutorialHide> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::event::EventWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,bevy_ecs::query::filter::Without<toolbar::resouces_and_components::UiShowIfParentIsInFocus> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventWriter<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::system::system_param::Local<std::collections::hash::set::HashSet<bevy_ecs::entity::Entity,std::hash::random::RandomState> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::toolbar::ToolbarUiIsInFocus>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomAppear> > > >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiJustBecameInFocusEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchAppModeButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,tuple$<> >,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiSlider>,tuple$<> >,bevy_ecs::system::query::Query<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSlider> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiCustomSize> > > >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::UiInteractionState>,ref_mut$<toolbar::resouces_and_components::UiAnimationState>,ref_mut$<toolbar::resouces_and_components::UiCustomAppear> >,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<toolbar::resouces_and_components::UiPlaySelectAnimation>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> > > >,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiSliderReset> > > >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::system::query::Query<ref$<toolbar::resouces_and_components::UiLocked>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<toolbar::resouces_and_components::UiPlayLockedAnimation>,ref_mut$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "constructor": "bevy_ecs::event::Events", "arguments": ["toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$4::init_state<bevy_ecs::event::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns> >", "constructor": "bevy_ecs::system::system_param::impl$4::init_state", "arguments": ["bevy_ecs::event::Events<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)", "void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_button.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_button.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_app_mode_button
// 信息 ID: f8a8ab7891a12a72d966a756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9d67a1788534f583d6f15a38: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x15e6434 -> 0x1b57810 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchAppModeButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> @ 0x165941b -> 0x1b57810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::switch_toolbar_category_button
// 信息 ID: 15531190d5d359b615834f98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2cfe95b34e2244155260f265: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> > @ 0x15da710 -> 0x1b57e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::SwitchToolbarCategoryButton>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::system::commands::Commands)> @ 0x165999b -> 0x1b57e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::tod_v2_button
// 信息 ID: 948ea8b88cda3b87a14228f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8e7d9a59094894a5bc333ceb: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> > @ 0x15e4188 -> 0x1b58470 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::TimeOfDayButtonV2>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::system::commands::Commands)> @ 0x1659e6e -> 0x1b58470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_button::toolbar_button_keep_focus
// 信息 ID: d2fb681cce27e0cd35a20551
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 00a3a9d29ea27df4e759d5bb: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > @ 0x15e878b -> 0x1b570f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref_mut$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> @ 0x165a14e -> 0x1b570f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
