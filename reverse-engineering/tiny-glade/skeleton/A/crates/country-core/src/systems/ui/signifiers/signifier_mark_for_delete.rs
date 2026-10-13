// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0e74f6d8fe04e2c37ae7083c95f5bc74c1e33479af75e0587d32d6b14e260112
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: signifier_mark_for_delete.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2829
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":6864,"rva":25258256,"va":5393967376}
// 编译实例 RVAs: 0x1816910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24355104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8e05e8cfe","size":5,"soft_fail":false,"target_va":5369613520,"va":5393968619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21343082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e89654bafe","size":5,"soft_fail":false,"target_va":5372625024,"va":5393968101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21219299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e81d38bcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393969006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c21e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18172544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e880b5eafe","size":5,"soft_fail":false,"target_va":5375795680,"va":5393968219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7130
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18087675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e80501ecfe","size":5,"soft_fail":false,"target_va":5375881520,"va":5393969190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14316959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8618a25ff","size":5,"soft_fail":false,"target_va":5379651280,"va":5393968234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10659705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e887585dff","size":5,"soft_fail":false,"target_va":5383308144,"va":5393967844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8950201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8b9918800","size":5,"soft_fail":false,"target_va":5402919072,"va":5393968866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16723520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8c0d100ff","size":5,"soft_fail":false,"target_va":5377245296,"va":5393968811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9003016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e808608900","size":5,"soft_fail":false,"target_va":5402972080,"va":5393969059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16722361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e847d600ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5393968884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9009610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8ca798900","size":5,"soft_fail":false,"target_va":5402978688,"va":5393969073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e870e5ffff","size":5,"soft_fail":false,"target_va":5393962224,"va":5393969019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9012230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e806848900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393967893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e84cecffff","size":5,"soft_fail":false,"target_va":5393962416,"va":5393967455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9013126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e886878900","size":5,"soft_fail":false,"target_va":5402980656,"va":5393967525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17611149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e87346f3fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5393968056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17607269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e89b55f3fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5393967824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17611286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e8ea45f3fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5393967425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16601317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393967376,"raw_hex":"e81baf02ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5393967536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":11756,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":13672,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":10812,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2829,"record_offset":10612,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":13644,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":13764,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":10292,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":10376,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifier_config_hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2829,"record_offset":10500,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":60131,"source_line":61,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9260,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":7096,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":8248,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":2829,"record_offset":12812,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":2829,"record_offset":10556,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":2829,"record_offset":7680,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":8743,"source_line":126,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":14184,"parent_record_offset":14156,"ancestor_record_offset":6864,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":14156,"parent_record_offset":13940,"ancestor_record_offset":6864,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11248,"parent_record_offset":11220,"ancestor_record_offset":6864,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11220,"parent_record_offset":11188,"ancestor_record_offset":6864,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":6864,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":6864,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":10992,"parent_record_offset":10948,"ancestor_record_offset":6864,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11300,"parent_record_offset":10948,"ancestor_record_offset":6864,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11160,"parent_record_offset":10948,"ancestor_record_offset":6864,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11132,"parent_record_offset":10948,"ancestor_record_offset":6864,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":6864,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12156,"parent_record_offset":12124,"ancestor_record_offset":6864,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":6864,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12740,"parent_record_offset":12660,"ancestor_record_offset":6864,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12124,"parent_record_offset":12016,"ancestor_record_offset":6864,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":6864,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":6864,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":6864,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":6864,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2829,"record_offset":10640,"parent_record_offset":10612,"ancestor_record_offset":6864,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":12660,"parent_record_offset":11780,"ancestor_record_offset":6864,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":12016,"parent_record_offset":11780,"ancestor_record_offset":6864,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2829,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":6864,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12256,"parent_record_offset":12232,"ancestor_record_offset":6864,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12232,"parent_record_offset":12208,"ancestor_record_offset":6864,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":6864,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12344,"parent_record_offset":12280,"ancestor_record_offset":6864,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":6864,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":12592,"parent_record_offset":12424,"ancestor_record_offset":6864,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":11836,"parent_record_offset":11780,"ancestor_record_offset":6864,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":6864,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":6864,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":6864,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":11932,"parent_record_offset":11780,"ancestor_record_offset":6864,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":10876,"parent_record_offset":10844,"ancestor_record_offset":6864,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":13704,"parent_record_offset":13672,"ancestor_record_offset":6864,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":10908,"parent_record_offset":10876,"ancestor_record_offset":6864,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":10844,"parent_record_offset":10812,"ancestor_record_offset":6864,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":2829,"record_offset":11332,"parent_record_offset":10908,"ancestor_record_offset":6864,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":2829,"record_offset":10948,"parent_record_offset":10908,"ancestor_record_offset":6864,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2829,"record_offset":12768,"parent_record_offset":12740,"ancestor_record_offset":6864,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2829,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":6864,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9644,"parent_record_offset":9616,"ancestor_record_offset":6864,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":8072,"parent_record_offset":8044,"ancestor_record_offset":6864,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":6864,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":8100,"parent_record_offset":8044,"ancestor_record_offset":6864,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":6864,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9616,"parent_record_offset":9584,"ancestor_record_offset":6864,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9788,"parent_record_offset":9616,"ancestor_record_offset":6864,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":6864,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2829,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":6864,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2829,"record_offset":13424,"parent_record_offset":13400,"ancestor_record_offset":6864,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2829,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":6864,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2829,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":6864,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2829,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":6864,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2829,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":6864,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2829,"record_offset":14032,"parent_record_offset":14004,"ancestor_record_offset":6864,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2829,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":6864,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2829,"record_offset":7316,"parent_record_offset":7220,"ancestor_record_offset":6864,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2829,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":6864,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":7996,"parent_record_offset":7968,"ancestor_record_offset":6864,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":6864,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":8844,"parent_record_offset":8384,"ancestor_record_offset":6864,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":9528,"parent_record_offset":9356,"ancestor_record_offset":6864,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2829,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":6864,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":9852,"parent_record_offset":9556,"ancestor_record_offset":6864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":6864,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":6864,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":14096,"parent_record_offset":13972,"ancestor_record_offset":6864,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":11048,"parent_record_offset":10948,"ancestor_record_offset":6864,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":6864,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":6864,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":6864,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8804,"parent_record_offset":8772,"ancestor_record_offset":6864,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":6864,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":6864,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":10252,"parent_record_offset":9324,"ancestor_record_offset":6864,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":7252,"parent_record_offset":7220,"ancestor_record_offset":6864,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":6864,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":7280,"parent_record_offset":7252,"ancestor_record_offset":6864,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2829,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":6864,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":10440,"parent_record_offset":10376,"ancestor_record_offset":6864,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":6864,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":6864,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":11868,"parent_record_offset":11780,"ancestor_record_offset":6864,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8548,"parent_record_offset":8440,"ancestor_record_offset":6864,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":7220,"parent_record_offset":7192,"ancestor_record_offset":6864,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":8280,"parent_record_offset":8248,"ancestor_record_offset":6864,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":8412,"parent_record_offset":8384,"ancestor_record_offset":6864,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":8384,"parent_record_offset":8352,"ancestor_record_offset":6864,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":6864,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":6864,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":6864,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8876,"parent_record_offset":8352,"ancestor_record_offset":6864,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7416,"parent_record_offset":7384,"ancestor_record_offset":6864,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7576,"parent_record_offset":7416,"ancestor_record_offset":6864,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":6864,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7132,"parent_record_offset":7096,"ancestor_record_offset":6864,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7600,"parent_record_offset":7576,"ancestor_record_offset":6864,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7192,"parent_record_offset":7160,"ancestor_record_offset":6864,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":6864,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":6864,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8688,"parent_record_offset":8440,"ancestor_record_offset":6864,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8660,"parent_record_offset":8440,"ancestor_record_offset":6864,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8492,"parent_record_offset":8440,"ancestor_record_offset":6864,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2829,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":6864,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2829,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":6864,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2829,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":6864,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2829,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":6864,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2829,"record_offset":7764,"parent_record_offset":7736,"ancestor_record_offset":6864,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2829,"record_offset":12916,"parent_record_offset":12884,"ancestor_record_offset":6864,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13584,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":6864,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":12944,"parent_record_offset":12884,"ancestor_record_offset":6864,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13972,"parent_record_offset":13940,"ancestor_record_offset":6864,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":6864,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":6864,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13940,"parent_record_offset":13884,"ancestor_record_offset":6864,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":6864,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":14004,"parent_record_offset":13972,"ancestor_record_offset":6864,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2829,"record_offset":13232,"parent_record_offset":13204,"ancestor_record_offset":6864,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2829,"record_offset":13260,"parent_record_offset":13232,"ancestor_record_offset":6864,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2829,"record_offset":13320,"parent_record_offset":13204,"ancestor_record_offset":6864,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2829,"record_offset":13204,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13036,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":6864,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2829,"record_offset":13352,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2829,"record_offset":13064,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2829,"record_offset":13148,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2829,"record_offset":13492,"parent_record_offset":12992,"ancestor_record_offset":6864,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":2829,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":6864,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":2829,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":6864,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":2829,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":6864,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2829,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":6864,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9292,"parent_record_offset":9260,"ancestor_record_offset":6864,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2829,"record_offset":9392,"parent_record_offset":9356,"ancestor_record_offset":6864,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9556,"parent_record_offset":9356,"ancestor_record_offset":6864,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9824,"parent_record_offset":9556,"ancestor_record_offset":6864,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":2829,"record_offset":7708,"parent_record_offset":7680,"ancestor_record_offset":6864,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2829,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":6864,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2829,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":6864,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2829,"record_offset":8044,"parent_record_offset":7876,"ancestor_record_offset":6864,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2829,"record_offset":7792,"parent_record_offset":7764,"ancestor_record_offset":6864,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2829,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":6864,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2829,"record_offset":7968,"parent_record_offset":7936,"ancestor_record_offset":6864,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":7876,"parent_record_offset":7848,"ancestor_record_offset":6864,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":6864,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":14248,"rva":25260192,"va":5393969312}
// 编译实例 RVAs: 0x18170a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: signifier_mark_for_delete.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10661610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e816515dff","size":5,"soft_fail":false,"target_va":5383308144,"va":5393969749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16725006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8f2cb00ff","size":5,"soft_fail":false,"target_va":5377245296,"va":5393970297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9001668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8c45a8900","size":5,"soft_fail":false,"target_va":5402972080,"va":5393970407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16664799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e821b701ff","size":5,"soft_fail":false,"target_va":5377305552,"va":5393970346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9008262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e886748900","size":5,"soft_fail":false,"target_va":5402978688,"va":5393970421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e82be0ffff","size":5,"soft_fail":false,"target_va":5393962224,"va":5393970368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9010325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8957c8900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393969798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8c1e4ffff","size":5,"soft_fail":false,"target_va":5393962416,"va":5393969386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9011195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8fb7f8900","size":5,"soft_fail":false,"target_va":5402980656,"va":5393969456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17613055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e8013ff3fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5393969962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17609174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e82a4ef3fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5393969729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17613218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393969312,"raw_hex":"e85e3ef3fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5393969357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":17256,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":18364,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2829,"record_offset":17056,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":18308,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":16024,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":14436,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":15012,"parent_record_offset":14248,"ancestor_record_offset":14248,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":14248,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17776,"parent_record_offset":17744,"ancestor_record_offset":14248,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":18064,"parent_record_offset":17744,"ancestor_record_offset":14248,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17380,"parent_record_offset":17352,"ancestor_record_offset":14248,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17444,"parent_record_offset":17416,"ancestor_record_offset":14248,"inlinee":24860,"source_line":1566,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17744,"parent_record_offset":17720,"ancestor_record_offset":14248,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17808,"parent_record_offset":17776,"ancestor_record_offset":14248,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":14248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":14248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":2829,"record_offset":17284,"parent_record_offset":17256,"ancestor_record_offset":14248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2829,"record_offset":17084,"parent_record_offset":17056,"ancestor_record_offset":14248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":17352,"parent_record_offset":17284,"ancestor_record_offset":14248,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":17720,"parent_record_offset":17284,"ancestor_record_offset":14248,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2829,"record_offset":18256,"parent_record_offset":18232,"ancestor_record_offset":14248,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":14248,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":14248,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":14248,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":17984,"parent_record_offset":17920,"ancestor_record_offset":14248,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":18164,"parent_record_offset":18140,"ancestor_record_offset":14248,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":18232,"parent_record_offset":18064,"ancestor_record_offset":14248,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":17540,"parent_record_offset":17284,"ancestor_record_offset":14248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":18092,"parent_record_offset":18064,"ancestor_record_offset":14248,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":17836,"parent_record_offset":17808,"ancestor_record_offset":14248,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":17492,"parent_record_offset":17468,"ancestor_record_offset":14248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2829,"record_offset":17636,"parent_record_offset":17284,"ancestor_record_offset":14248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":2829,"record_offset":18400,"parent_record_offset":18364,"ancestor_record_offset":14248,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2829,"record_offset":17416,"parent_record_offset":17380,"ancestor_record_offset":14248,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2829,"record_offset":18424,"parent_record_offset":18400,"ancestor_record_offset":14248,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16408,"parent_record_offset":16380,"ancestor_record_offset":14248,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":14248,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":14248,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16348,"parent_record_offset":16320,"ancestor_record_offset":14248,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16380,"parent_record_offset":16348,"ancestor_record_offset":14248,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16552,"parent_record_offset":16380,"ancestor_record_offset":14248,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":16664,"parent_record_offset":16640,"ancestor_record_offset":14248,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2829,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":14248,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2829,"record_offset":14648,"parent_record_offset":14552,"ancestor_record_offset":14248,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":16244,"parent_record_offset":16216,"ancestor_record_offset":14248,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":14248,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":15608,"parent_record_offset":15148,"ancestor_record_offset":14248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":16292,"parent_record_offset":16120,"ancestor_record_offset":14248,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2829,"record_offset":14960,"parent_record_offset":14932,"ancestor_record_offset":14248,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":16616,"parent_record_offset":16320,"ancestor_record_offset":14248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":14248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":14248,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":14248,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":15568,"parent_record_offset":15536,"ancestor_record_offset":14248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":16508,"parent_record_offset":16484,"ancestor_record_offset":14248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":17016,"parent_record_offset":16088,"ancestor_record_offset":14248,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":14584,"parent_record_offset":14552,"ancestor_record_offset":14248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":14248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":14612,"parent_record_offset":14584,"ancestor_record_offset":14248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2829,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":14248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":14248,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":14248,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":17572,"parent_record_offset":17284,"ancestor_record_offset":14248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":15312,"parent_record_offset":15204,"ancestor_record_offset":14248,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":14552,"parent_record_offset":14524,"ancestor_record_offset":14248,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":15044,"parent_record_offset":15012,"ancestor_record_offset":14248,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":15176,"parent_record_offset":15148,"ancestor_record_offset":14248,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":14248,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":14248,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14248,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":14248,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":15640,"parent_record_offset":15116,"ancestor_record_offset":14248,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":14248,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14908,"parent_record_offset":14748,"ancestor_record_offset":14248,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":14248,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14468,"parent_record_offset":14436,"ancestor_record_offset":14248,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":14248,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14524,"parent_record_offset":14492,"ancestor_record_offset":14248,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":14248,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":15204,"parent_record_offset":15176,"ancestor_record_offset":14248,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":15452,"parent_record_offset":15204,"ancestor_record_offset":14248,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":15424,"parent_record_offset":15204,"ancestor_record_offset":14248,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":15256,"parent_record_offset":15204,"ancestor_record_offset":14248,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs
//     evidence={"module":2829,"record_offset":18332,"parent_record_offset":18308,"ancestor_record_offset":14248,"inlinee":90351,"source_line":74,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2829,"record_offset":18188,"parent_record_offset":18164,"ancestor_record_offset":14248,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":16056,"parent_record_offset":16024,"ancestor_record_offset":14248,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2829,"record_offset":16156,"parent_record_offset":16120,"ancestor_record_offset":14248,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":16320,"parent_record_offset":16120,"ancestor_record_offset":14248,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":16588,"parent_record_offset":16320,"ancestor_record_offset":14248,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2829,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":14248,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2829,"record_offset":16216,"parent_record_offset":16184,"ancestor_record_offset":14248,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":16120,"parent_record_offset":16088,"ancestor_record_offset":14248,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":16088,"parent_record_offset":16056,"ancestor_record_offset":14248,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160EF
// 首条位置: {"module":2829,"inlinee":90351}
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
// 完整结构及原始类型记录: signifier_mark_for_delete.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: signifier_mark_for_delete.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: signifier_mark_for_delete.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked
// 信息 ID: 707cb558550958774491d1de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3a523619da97fec4785f2e23: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0xe450fb -> 0x18170a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x112a8bd -> 0x18170a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked::closure$0
// 信息 ID: d1fd61877cff7fd81abdfcf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked; module=2829, inline_record=18332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete
// 信息 ID: f83dfa9b655466bcdf32944b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e9e6c5fcd41d8a2cb144ae4f: void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands)> > @ 0xe682ab -> 0x1816910 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands)> @ 0x112a74d -> 0x1816910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
