// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/manage_clutter_entities.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 77a32eb1fde357dea98e901ecbd468e7bf1d8122a64408131a80b59abcea1477
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: manage_clutter_entities.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2274, 2281, 2287, 3510, 3537, 4261
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3510,"record_offset":3060,"rva":28115040,"va":5396824160}
// 编译实例 RVAs: 0x1ad0060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=4
//     target_aliases=1889 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28108115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8ad1a53fe","size":5,"soft_fail":false,"target_va":5368720672,"va":5396828782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27212040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8f8c660fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5396825555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25132082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8ce8380fe","size":5,"soft_fail":false,"target_va":5371700080,"va":5396832157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24081001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8978d90fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396830708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414e6280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6204319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e86154a1ff","size":5,"soft_fail":false,"target_va":5390623360,"va":5396827674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dd2820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: manage_clutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3155115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8ab243000","size":5,"soft_fail":false,"target_va":5399980064,"va":5396824944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t6092496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8d0f65c00","size":5,"soft_fail":false,"target_va":5402919072,"va":5396826571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25523392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8408b7afe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396828267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19581206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8ea36d5fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5396826497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6141140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8d4b45d00","size":5,"soft_fail":false,"target_va":5402972080,"va":5396830935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3146808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e838043000","size":5,"soft_fail":false,"target_va":5399975520,"va":5396828707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-19580344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8483ad5fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5396826867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6147717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e885ce5d00","size":5,"soft_fail":false,"target_va":5402978688,"va":5396830966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6147957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e875cf5d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5396830918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e803e1ffff","size":5,"soft_fail":false,"target_va":5396822880,"va":5396830808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6155398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e886ec5d00","size":5,"soft_fail":false,"target_va":5402980128,"va":5396824725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e821fbffff","size":5,"soft_fail":false,"target_va":5396823072,"va":5396824314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6156260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8e4ef5d00","size":5,"soft_fail":false,"target_va":5402980656,"va":5396824391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-10978443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8757b58ff","size":5,"soft_fail":false,"target_va":5385850032,"va":5396828470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20467984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8f0aec7fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5396824891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20464101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e81bbec7fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5396824656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20468155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e845aec7fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5396824294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h93c6532eedf5b01e (.llvm.12250732642055207085)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17902041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e827d6eefe","size":5,"soft_fail":false,"target_va":5378922640,"va":5396824676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$0
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e8f9190000","size":5,"soft_fail":false,"target_va":5396832480,"va":5396825826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14571772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396824160,"raw_hex":"e804a721ff","size":5,"soft_fail":false,"target_va":5382255664,"va":5396827431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":13604,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":14288,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":16232,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3510,"record_offset":16288,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3510,"record_offset":8944,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3510,"record_offset":6312,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":11976,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":12100,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3510,"record_offset":13632,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3510,"record_offset":6108,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3510,"record_offset":15472,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3510,"record_offset":6080,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":9056,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":16944,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":12664,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":17876,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3510,"record_offset":22400,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3510,"record_offset":15068,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3510,"record_offset":5996,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3510,"record_offset":16460,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":4928,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":3340,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":3916,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3510,"record_offset":7720,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3510,"record_offset":13956,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3510,"record_offset":14424,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3510,"record_offset":16432,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48756,"source_line":81,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterType::is_plant
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3510,"record_offset":18824,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48755,"source_line":77,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterType::is_window
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3510,"record_offset":16404,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48754,"source_line":73,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterType::is_window_shutter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3510,"record_offset":16972,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48757,"source_line":85,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::manage_clutter_entities::update_shutter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/manage_clutter_entities.rs
//     evidence={"module":3510,"record_offset":9084,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":106801,"source_line":202,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3510,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3510,"record_offset":22612,"parent_record_offset":22520,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":3060,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":22548,"parent_record_offset":22520,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10540,"parent_record_offset":10508,"ancestor_record_offset":3060,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":14360,"parent_record_offset":14328,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":3060,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":14328,"parent_record_offset":14288,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":10620,"parent_record_offset":10452,"ancestor_record_offset":3060,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":3060,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":18396,"parent_record_offset":18048,"ancestor_record_offset":3060,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":3060,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3510,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":3060,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3510,"record_offset":22472,"parent_record_offset":22448,"ancestor_record_offset":3060,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3510,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":3060,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3510,"record_offset":10156,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":3060,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3060,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":3060,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":6712,"parent_record_offset":6680,"ancestor_record_offset":3060,"inlinee":6327,"source_line":1372,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":6980,"parent_record_offset":6680,"ancestor_record_offset":3060,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":7392,"parent_record_offset":7308,"ancestor_record_offset":3060,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":3060,"inlinee":24860,"source_line":1566,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":6680,"parent_record_offset":6572,"ancestor_record_offset":3060,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3510,"record_offset":6740,"parent_record_offset":6712,"ancestor_record_offset":3060,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3510,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3060,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3510,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":3060,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3510,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":3060,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3510,"record_offset":15812,"parent_record_offset":15496,"ancestor_record_offset":3060,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3510,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":3060,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3510,"record_offset":7308,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::init_fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3510,"record_offset":13744,"parent_record_offset":13712,"ancestor_record_offset":3060,"inlinee":45758,"source_line":170,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3510,"record_offset":6572,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3510,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":3060,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3510,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":3060,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3510,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":3060,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3510,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":3060,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3510,"record_offset":6900,"parent_record_offset":6836,"ancestor_record_offset":3060,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3510,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":3060,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":7268,"parent_record_offset":6980,"ancestor_record_offset":3060,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":7212,"parent_record_offset":6980,"ancestor_record_offset":3060,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":6392,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":3060,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":3060,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":3060,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3510,"record_offset":6488,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":3060,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10212,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10248,"parent_record_offset":10212,"ancestor_record_offset":3060,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":3060,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10308,"parent_record_offset":10248,"ancestor_record_offset":3060,"inlinee":21464,"source_line":216,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":3060,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10348,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3510,"record_offset":10652,"parent_record_offset":10420,"ancestor_record_offset":3060,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3510,"record_offset":10452,"parent_record_offset":10420,"ancestor_record_offset":3060,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3510,"record_offset":7496,"parent_record_offset":7468,"ancestor_record_offset":3060,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3510,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":3060,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3510,"record_offset":10280,"parent_record_offset":10248,"ancestor_record_offset":3060,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5492,"parent_record_offset":5320,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3510,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3510,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":3060,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3510,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":3060,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3510,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":3060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3510,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":3060,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3510,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":3060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3510,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":3060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3510,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":3060,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3510,"record_offset":15532,"parent_record_offset":15496,"ancestor_record_offset":3060,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3510,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":3060,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3510,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3510,"record_offset":23144,"parent_record_offset":23116,"ancestor_record_offset":3060,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3510,"record_offset":23168,"parent_record_offset":23144,"ancestor_record_offset":3060,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3510,"record_offset":22576,"parent_record_offset":22548,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3510,"record_offset":3552,"parent_record_offset":3456,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3510,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":3060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":9364,"parent_record_offset":9228,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":12776,"parent_record_offset":12736,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":4512,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3510,"record_offset":5228,"parent_record_offset":5048,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3510,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3510,"record_offset":5556,"parent_record_offset":5256,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3510,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3510,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":3060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3510,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":3060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3510,"record_offset":17396,"parent_record_offset":17136,"ancestor_record_offset":3060,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3510,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":3060,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3510,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3510,"record_offset":16344,"parent_record_offset":16288,"ancestor_record_offset":3060,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3510,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3510,"record_offset":4472,"parent_record_offset":4440,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3510,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":3060,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3510,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3510,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":3060,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3510,"record_offset":5956,"parent_record_offset":5008,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3510,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":3060,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3510,"record_offset":15208,"parent_record_offset":15116,"ancestor_record_offset":3060,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3510,"record_offset":3488,"parent_record_offset":3456,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3510,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3510,"record_offset":3516,"parent_record_offset":3488,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3510,"record_offset":7136,"parent_record_offset":7104,"ancestor_record_offset":3060,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3510,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3510,"record_offset":16044,"parent_record_offset":15980,"ancestor_record_offset":3060,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3510,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3510,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3510,"record_offset":6424,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3510,"record_offset":4216,"parent_record_offset":4108,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3510,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3510,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3510,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3510,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3510,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3510,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3510,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3510,"record_offset":4544,"parent_record_offset":4020,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3652,"parent_record_offset":3620,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3812,"parent_record_offset":3652,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3372,"parent_record_offset":3340,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3428,"parent_record_offset":3396,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3510,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3510,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3510,"record_offset":4356,"parent_record_offset":4108,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3510,"record_offset":4328,"parent_record_offset":4108,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3510,"record_offset":4160,"parent_record_offset":4108,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3510,"record_offset":9896,"parent_record_offset":9796,"ancestor_record_offset":3060,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":9760,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":9796,"parent_record_offset":9760,"ancestor_record_offset":3060,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":10120,"parent_record_offset":9796,"ancestor_record_offset":3060,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9160,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":3060,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9296,"parent_record_offset":9264,"ancestor_record_offset":3060,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9264,"parent_record_offset":9228,"ancestor_record_offset":3060,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":3060,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9200,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":3060,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3510,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":3060,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":7752,"parent_record_offset":7720,"ancestor_record_offset":3060,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":7784,"parent_record_offset":7752,"ancestor_record_offset":3060,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":3060,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":8556,"parent_record_offset":8416,"ancestor_record_offset":3060,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":3060,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3510,"record_offset":7964,"parent_record_offset":7932,"ancestor_record_offset":3060,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":3060,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8444,"parent_record_offset":8416,"ancestor_record_offset":3060,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":3060,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14856,"parent_record_offset":14508,"ancestor_record_offset":3060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8776,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14088,"parent_record_offset":14028,"ancestor_record_offset":3060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14508,"parent_record_offset":14472,"ancestor_record_offset":3060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8812,"parent_record_offset":8016,"ancestor_record_offset":3060,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":3060,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":7992,"parent_record_offset":7932,"ancestor_record_offset":3060,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":7816,"parent_record_offset":7784,"ancestor_record_offset":3060,"inlinee":17280,"source_line":1571,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14004,"parent_record_offset":13980,"ancestor_record_offset":3060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":3060,"inlinee":17281,"source_line":1565,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":23084,"parent_record_offset":23052,"ancestor_record_offset":3060,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14060,"parent_record_offset":14028,"ancestor_record_offset":3060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":3060,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":3060,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":23052,"parent_record_offset":22996,"ancestor_record_offset":3060,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":23024,"parent_record_offset":22996,"ancestor_record_offset":3060,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":3060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":23116,"parent_record_offset":23084,"ancestor_record_offset":3060,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3510,"record_offset":8296,"parent_record_offset":8264,"ancestor_record_offset":3060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3510,"record_offset":8324,"parent_record_offset":8296,"ancestor_record_offset":3060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3510,"record_offset":8384,"parent_record_offset":8264,"ancestor_record_offset":3060,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3510,"record_offset":8264,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8096,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":22972,"parent_record_offset":22948,"ancestor_record_offset":3060,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":3060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3510,"record_offset":8416,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3510,"record_offset":8124,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3510,"record_offset":14116,"parent_record_offset":14088,"ancestor_record_offset":3060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3510,"record_offset":8208,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3510,"record_offset":8684,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3510,"record_offset":14172,"parent_record_offset":14116,"ancestor_record_offset":3060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3510,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":3060,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":4968,"parent_record_offset":4928,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3510,"record_offset":5092,"parent_record_offset":5048,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":5256,"parent_record_offset":5048,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3510,"record_offset":5528,"parent_record_offset":5256,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3510,"record_offset":9328,"parent_record_offset":9296,"ancestor_record_offset":3060,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3510,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3510,"record_offset":5152,"parent_record_offset":5120,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3510,"record_offset":5048,"parent_record_offset":5008,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3510,"record_offset":5008,"parent_record_offset":4968,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/manage_clutter_entities.rs
//     evidence={"module":3510,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":3060,"inlinee":106802,"source_line":82,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$5
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/manage_clutter_entities.rs
//     evidence={"module":3510,"record_offset":15560,"parent_record_offset":15532,"ancestor_record_offset":3060,"inlinee":106803,"source_line":97,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3510,"record_offset":24076,"rva":28123360,"va":5396832480}
// 编译实例 RVAs: 0x1ad20e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":24852,"parent_record_offset":24076,"ancestor_record_offset":24076,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":24252,"parent_record_offset":24076,"ancestor_record_offset":24076,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":24292,"parent_record_offset":24076,"ancestor_record_offset":24076,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":24456,"parent_record_offset":24320,"ancestor_record_offset":24076,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3510,"record_offset":24972,"parent_record_offset":24876,"ancestor_record_offset":24076,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":24876,"parent_record_offset":24852,"ancestor_record_offset":24076,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":25196,"parent_record_offset":24876,"ancestor_record_offset":24076,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":24388,"parent_record_offset":24356,"ancestor_record_offset":24076,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":24356,"parent_record_offset":24320,"ancestor_record_offset":24076,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":24320,"parent_record_offset":24292,"ancestor_record_offset":24076,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3510,"record_offset":24420,"parent_record_offset":24388,"ancestor_record_offset":24076,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D40
// 首条位置: {"module":2274,"inlinee":81216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A132
// 首条位置: {"module":3510,"inlinee":106802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":12688,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":12776,"parent_record_offset":12736,"ancestor_record_offset":12688,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":12804,"parent_record_offset":12736,"ancestor_record_offset":12688,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":12688,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3510,"record_offset":12840,"parent_record_offset":12804,"ancestor_record_offset":12688,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D66
// 首条位置: {"module":2281,"inlinee":81254}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A133
// 首条位置: {"module":3510,"inlinee":106803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":15596,"parent_record_offset":15560,"ancestor_record_offset":15560,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":15668,"parent_record_offset":15624,"ancestor_record_offset":15560,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":15728,"parent_record_offset":15624,"ancestor_record_offset":15560,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":15624,"parent_record_offset":15596,"ancestor_record_offset":15560,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3510,"record_offset":15764,"parent_record_offset":15728,"ancestor_record_offset":15560,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::manage_clutter_entities::update_shutter
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A131
// 首条位置: {"module":3510,"inlinee":106801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3510,"record_offset":10156,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10212,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10348,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":9760,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9160,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9200,"parent_record_offset":9084,"ancestor_record_offset":9084,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":9084,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10540,"parent_record_offset":10508,"ancestor_record_offset":9084,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3510,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":9084,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":10620,"parent_record_offset":10452,"ancestor_record_offset":9084,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3510,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9084,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3510,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9084,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":9084,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10248,"parent_record_offset":10212,"ancestor_record_offset":9084,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":9084,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10308,"parent_record_offset":10248,"ancestor_record_offset":9084,"inlinee":21464,"source_line":216,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3510,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":9084,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3510,"record_offset":10652,"parent_record_offset":10420,"ancestor_record_offset":9084,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3510,"record_offset":10452,"parent_record_offset":10420,"ancestor_record_offset":9084,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3510,"record_offset":10280,"parent_record_offset":10248,"ancestor_record_offset":9084,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3510,"record_offset":9364,"parent_record_offset":9228,"ancestor_record_offset":9084,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3510,"record_offset":9896,"parent_record_offset":9796,"ancestor_record_offset":9084,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":9796,"parent_record_offset":9760,"ancestor_record_offset":9084,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3510,"record_offset":10120,"parent_record_offset":9796,"ancestor_record_offset":9084,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9296,"parent_record_offset":9264,"ancestor_record_offset":9084,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9264,"parent_record_offset":9228,"ancestor_record_offset":9084,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3510,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":9084,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03510.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3510,"record_offset":9328,"parent_record_offset":9296,"ancestor_record_offset":9084,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03510.jsonl.gz"}
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
// 完整结构及原始类型记录: manage_clutter_entities.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: manage_clutter_entities.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: manage_clutter_entities.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities
// 信息 ID: 8f2a683c1d16797a6e1b7a44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] af7a2aab7a326a857abad06f: void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >),void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >)> > @ 0x14c4d14 -> 0x1ad0060 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >),void (*)(bevy_ecs::change_detection::Res<system_clutter::clutter::ClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref_mut$<system_clutter::clutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<system_clutter::clutter::Clutter> > >)> @ 0x14ea3bd -> 0x1ad0060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$0
// 信息 ID: e0433c54313ee8547f29f772
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_clutter::clutter::manage_clutter_entities::manage_clutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities @ 0x1ad06e2 -> 0x1ad20e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities @ 0x1ad1110 -> 0x1ad20e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$1
// 信息 ID: 9e04281cfbb39e8c822e664c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::clutter::manage_clutter_entities::manage_clutter_entities", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$1 <- std::collections::hash::map::impl$79::from_iter<u64,ref$<system_clutter::clutter::ClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::clutter::ClutterObject>,system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$79::from_iter<u64,ref$<system_clutter::clutter::ClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::clutter::ClutterObject>,system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$1> >; module=4261, inline_record=1472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$3
// 信息 ID: 73f04280da70800a8abacbd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_clutter::clutter::manage_clutter_entities::manage_clutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities; module=3510, inline_record=12688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$4
// 信息 ID: 1d70ddcd5711edcde789d538
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_clutter::clutter::manage_clutter_entities::manage_clutter_entities", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$4> >; module=2281, inline_record=26200; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<system_clutter::clutter::Clutter>,enum2$<core::option::Option<ref$<system_clutter::clutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<system_clutter::clutter::ClutterMarkedForDeletion> >,system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure_env$4> >; module=2281, inline_record=28272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::manage_clutter_entities::closure$5
// 信息 ID: 3a7d97cf0c5e221487335b66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_clutter::clutter::manage_clutter_entities::manage_clutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities; module=3510, inline_record=15560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::manage_clutter_entities::update_shutter
// 信息 ID: 9041617def39ca5777282569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::manage_clutter_entities::manage_clutter_entities; module=3510, inline_record=9084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
