// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui_2d_follow_mouse.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 66b46a4143a03b5ab93a5d77aeefa3ec42fc11c0d74b8df76a861e850c5bfe6f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_2d_follow_mouse.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 439
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":439,"record_offset":13616,"rva":6186128,"va":5374895248}
// 编译实例 RVAs: 0x5e6490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_2d_follow_mouse.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8412474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e83a5d8000","size":5,"soft_fail":false,"target_va":5383308144,"va":5374895665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28082568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e88881ac01","size":5,"soft_fail":false,"target_va":5402978688,"va":5374896115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28083968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e80087ac01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374896155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e8cef8ffff","size":5,"soft_fail":false,"target_va":5374893472,"va":5374895309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28085265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e8118cac01","size":5,"soft_fail":false,"target_va":5402980656,"va":5374895386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1461072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e8504b1600","size":5,"soft_fail":false,"target_va":5376356912,"va":5374895835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1464906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e84a5a1600","size":5,"soft_fail":false,"target_va":5376360560,"va":5374895649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1461660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e89c4d1600","size":5,"soft_fail":false,"target_va":5376357744,"va":5374896079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1460856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374895248,"raw_hex":"e8784a1600","size":5,"soft_fail":false,"target_va":5376356144,"va":5374895283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":439,"record_offset":17028,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":439,"record_offset":16572,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":439,"record_offset":16372,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":439,"record_offset":17084,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":15376,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":13788,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":14364,"parent_record_offset":13616,"ancestor_record_offset":13616,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":439,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":13616,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":439,"record_offset":16680,"parent_record_offset":16652,"ancestor_record_offset":13616,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":439,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":13616,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":439,"record_offset":16448,"parent_record_offset":16424,"ancestor_record_offset":13616,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":439,"record_offset":16604,"parent_record_offset":16572,"ancestor_record_offset":13616,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":439,"record_offset":16400,"parent_record_offset":16372,"ancestor_record_offset":13616,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":439,"record_offset":16652,"parent_record_offset":16604,"ancestor_record_offset":13616,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":439,"record_offset":16992,"parent_record_offset":16604,"ancestor_record_offset":13616,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":439,"record_offset":16708,"parent_record_offset":16680,"ancestor_record_offset":13616,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":439,"record_offset":16908,"parent_record_offset":16604,"ancestor_record_offset":13616,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15752,"parent_record_offset":15724,"ancestor_record_offset":13616,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15804,"parent_record_offset":15780,"ancestor_record_offset":13616,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15780,"parent_record_offset":15752,"ancestor_record_offset":13616,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":13616,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":13616,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":15896,"parent_record_offset":15724,"ancestor_record_offset":13616,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":439,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":13616,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":439,"record_offset":15828,"parent_record_offset":15804,"ancestor_record_offset":13616,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":439,"record_offset":14000,"parent_record_offset":13904,"ancestor_record_offset":13616,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":439,"record_offset":15596,"parent_record_offset":15568,"ancestor_record_offset":13616,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":439,"record_offset":14960,"parent_record_offset":14500,"ancestor_record_offset":13616,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":439,"record_offset":15644,"parent_record_offset":15472,"ancestor_record_offset":13616,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":439,"record_offset":14312,"parent_record_offset":14284,"ancestor_record_offset":13616,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":439,"record_offset":15960,"parent_record_offset":15672,"ancestor_record_offset":13616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":439,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":13616,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":439,"record_offset":16872,"parent_record_offset":16848,"ancestor_record_offset":13616,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":439,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":13616,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":439,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":13616,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":439,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":13616,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":439,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":13616,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":439,"record_offset":13936,"parent_record_offset":13904,"ancestor_record_offset":13616,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":439,"record_offset":16472,"parent_record_offset":16448,"ancestor_record_offset":13616,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":439,"record_offset":13964,"parent_record_offset":13936,"ancestor_record_offset":13616,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":439,"record_offset":16496,"parent_record_offset":16472,"ancestor_record_offset":13616,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":439,"record_offset":16848,"parent_record_offset":16784,"ancestor_record_offset":13616,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":439,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":13616,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":439,"record_offset":15016,"parent_record_offset":14992,"ancestor_record_offset":13616,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":439,"record_offset":16784,"parent_record_offset":16604,"ancestor_record_offset":13616,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":439,"record_offset":14664,"parent_record_offset":14556,"ancestor_record_offset":13616,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":439,"record_offset":13904,"parent_record_offset":13876,"ancestor_record_offset":13616,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":439,"record_offset":14396,"parent_record_offset":14364,"ancestor_record_offset":13616,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":439,"record_offset":14528,"parent_record_offset":14500,"ancestor_record_offset":13616,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":439,"record_offset":14500,"parent_record_offset":14468,"ancestor_record_offset":13616,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":439,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":13616,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":439,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":13616,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":439,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13616,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":439,"record_offset":14992,"parent_record_offset":14468,"ancestor_record_offset":13616,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":14100,"parent_record_offset":14068,"ancestor_record_offset":13616,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":14260,"parent_record_offset":14100,"ancestor_record_offset":13616,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":13616,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":13820,"parent_record_offset":13788,"ancestor_record_offset":13616,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13616,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":13876,"parent_record_offset":13844,"ancestor_record_offset":13616,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":439,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":13616,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":439,"record_offset":14556,"parent_record_offset":14528,"ancestor_record_offset":13616,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":439,"record_offset":14804,"parent_record_offset":14556,"ancestor_record_offset":13616,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":439,"record_offset":14776,"parent_record_offset":14556,"ancestor_record_offset":13616,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":439,"record_offset":14608,"parent_record_offset":14556,"ancestor_record_offset":13616,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":15408,"parent_record_offset":15376,"ancestor_record_offset":13616,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":17140,"parent_record_offset":17112,"ancestor_record_offset":13616,"inlinee":7257,"source_line":601,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":17664,"parent_record_offset":17404,"ancestor_record_offset":13616,"inlinee":7265,"source_line":602,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":439,"record_offset":15508,"parent_record_offset":15472,"ancestor_record_offset":13616,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":15672,"parent_record_offset":15472,"ancestor_record_offset":13616,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":439,"record_offset":15932,"parent_record_offset":15672,"ancestor_record_offset":13616,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":439,"record_offset":15536,"parent_record_offset":15508,"ancestor_record_offset":13616,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":439,"record_offset":15568,"parent_record_offset":15536,"ancestor_record_offset":13616,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":439,"record_offset":15472,"parent_record_offset":15440,"ancestor_record_offset":13616,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00439.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":439,"record_offset":15440,"parent_record_offset":15408,"ancestor_record_offset":13616,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00439.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_2d_follow_mouse.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_2d_follow_mouse.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_2d_follow_mouse.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse
// 信息 ID: a6259af489c2dfb858ec0dc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e923ecda486e70ef335fb81e: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> > @ 0xc8eb6 -> 0x5e6490 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> @ 0x10c6bb -> 0x5e6490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
