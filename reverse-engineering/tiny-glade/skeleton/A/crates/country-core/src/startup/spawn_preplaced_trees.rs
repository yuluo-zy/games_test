// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/spawn_preplaced_trees.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 18a5ddfb98256b3ae404cef863b04628466387f87bf9cf20b3477644b3e7dac9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_preplaced_trees.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 483
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":483,"record_offset":12800,"rva":6856720,"va":5375565840}
// 编译实例 RVAs: 0x68a010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3915458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e83e41c4ff","size":5,"soft_fail":false,"target_va":5371652976,"va":5375568429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3255309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8f353ceff","size":5,"soft_fail":false,"target_va":5372314208,"va":5375569512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3250938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e80665ceff","size":5,"soft_fail":false,"target_va":5372320512,"va":5375571445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2816614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e89a05d5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375566321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=4
//     target_aliases=28 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-395179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e855f8f9ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5375566230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140651430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-233772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8d46efcff","size":5,"soft_fail":false,"target_va":5375333424,"va":5375567191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8a50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t319289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e839df0400","size":5,"soft_fail":false,"target_va":5375887952,"va":5375568658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8e70
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t382899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8b3d70500","size":5,"soft_fail":false,"target_va":5375954544,"va":5375571640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91b30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6320458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e84a716000","size":5,"soft_fail":false,"target_va":5381888816,"va":5375568353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d426d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7045369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8f9806b00","size":5,"soft_fail":false,"target_va":5382612688,"va":5375567314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d43270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7049281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e841906b00","size":5,"soft_fail":false,"target_va":5382615664,"va":5375566378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141024e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10070012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8fca79900","size":5,"soft_fail":false,"target_va":5385637408,"va":5375567391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27351459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8a359a101","size":5,"soft_fail":false,"target_va":5402919072,"va":5375567608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4262450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8cef5beff","size":5,"soft_fail":false,"target_va":5371305776,"va":5375568221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8806955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e82b628600","size":5,"soft_fail":false,"target_va":5384374480,"va":5375567520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8792407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e857298600","size":5,"soft_fail":false,"target_va":5384359968,"va":5375567556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1678894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e82e9e1900","size":5,"soft_fail":false,"target_va":5377246528,"va":5375567629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27410199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8173fa201","size":5,"soft_fail":false,"target_va":5402979152,"va":5375568948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27410020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8643ea201","size":5,"soft_fail":false,"target_va":5402978944,"va":5375568919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27413401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8994ba201","size":5,"soft_fail":false,"target_va":5402980128,"va":5375566722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1787615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8df461b00","size":5,"soft_fail":false,"target_va":5377353776,"va":5375566156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1839320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e8d8101c00","size":5,"soft_fail":false,"target_va":5377405616,"va":5375566291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1873549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375565840,"raw_hex":"e88d961c00","size":5,"soft_fail":false,"target_va":5377439776,"va":5375566222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$82::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":483,"record_offset":15432,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":27266,"source_line":2321,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":483,"record_offset":15264,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":483,"record_offset":15320,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":483,"record_offset":17304,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":483,"record_offset":19808,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":13052,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":19292,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":13328,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":483,"record_offset":17420,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":483,"record_offset":13500,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":483,"record_offset":13528,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":483,"record_offset":14764,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":483,"record_offset":14676,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":483,"record_offset":17024,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":483,"record_offset":16920,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":483,"record_offset":17068,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":483,"record_offset":17152,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":483,"record_offset":13584,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":21871,"source_line":200,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::TreeStorage::insert
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":483,"record_offset":19864,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":25771,"source_line":65,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":483,"record_offset":18532,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":483,"record_offset":13556,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":483,"record_offset":13416,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":483,"record_offset":17276,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":483,"record_offset":13868,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13836,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13696,"parent_record_offset":12800,"ancestor_record_offset":12800,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":483,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":12800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":483,"record_offset":14288,"parent_record_offset":14232,"ancestor_record_offset":12800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":12800,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":14260,"parent_record_offset":14232,"ancestor_record_offset":12800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":17620,"parent_record_offset":17588,"ancestor_record_offset":12800,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":19500,"parent_record_offset":19468,"ancestor_record_offset":12800,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":17588,"parent_record_offset":17560,"ancestor_record_offset":12800,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":483,"record_offset":14232,"parent_record_offset":14204,"ancestor_record_offset":12800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":483,"record_offset":20040,"parent_record_offset":20004,"ancestor_record_offset":12800,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":483,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":12800,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":483,"record_offset":19656,"parent_record_offset":19632,"ancestor_record_offset":12800,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":483,"record_offset":19468,"parent_record_offset":19424,"ancestor_record_offset":12800,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":483,"record_offset":17700,"parent_record_offset":17532,"ancestor_record_offset":12800,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":483,"record_offset":19424,"parent_record_offset":19380,"ancestor_record_offset":12800,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":483,"record_offset":17560,"parent_record_offset":17532,"ancestor_record_offset":12800,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":483,"record_offset":19348,"parent_record_offset":19324,"ancestor_record_offset":12800,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":483,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":12800,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":12800,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":19380,"parent_record_offset":19292,"ancestor_record_offset":12800,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":13116,"parent_record_offset":13076,"ancestor_record_offset":12800,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":483,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":12800,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":483,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":12800,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":483,"record_offset":17496,"parent_record_offset":17468,"ancestor_record_offset":12800,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":483,"record_offset":17444,"parent_record_offset":17420,"ancestor_record_offset":12800,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":483,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":12800,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":483,"record_offset":17732,"parent_record_offset":17496,"ancestor_record_offset":12800,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":483,"record_offset":17532,"parent_record_offset":17496,"ancestor_record_offset":12800,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":483,"record_offset":15488,"parent_record_offset":15460,"ancestor_record_offset":12800,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":483,"record_offset":16720,"parent_record_offset":16696,"ancestor_record_offset":12800,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":483,"record_offset":16988,"parent_record_offset":16960,"ancestor_record_offset":12800,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":483,"record_offset":13188,"parent_record_offset":13140,"ancestor_record_offset":12800,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":483,"record_offset":20276,"parent_record_offset":20248,"ancestor_record_offset":12800,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":483,"record_offset":18560,"parent_record_offset":18532,"ancestor_record_offset":12800,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":483,"record_offset":19924,"parent_record_offset":19864,"ancestor_record_offset":12800,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":483,"record_offset":13252,"parent_record_offset":13140,"ancestor_record_offset":12800,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":483,"record_offset":13380,"parent_record_offset":13352,"ancestor_record_offset":12800,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=82
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":483,"record_offset":14072,"parent_record_offset":13868,"ancestor_record_offset":12800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":483,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":12800,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":483,"record_offset":15400,"parent_record_offset":15320,"ancestor_record_offset":12800,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":483,"record_offset":20072,"parent_record_offset":20040,"ancestor_record_offset":12800,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":483,"record_offset":15360,"parent_record_offset":15320,"ancestor_record_offset":12800,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":483,"record_offset":19532,"parent_record_offset":19424,"ancestor_record_offset":12800,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":483,"record_offset":15516,"parent_record_offset":15488,"ancestor_record_offset":12800,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":483,"record_offset":15544,"parent_record_offset":15516,"ancestor_record_offset":12800,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":483,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":12800,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":483,"record_offset":17216,"parent_record_offset":17152,"ancestor_record_offset":12800,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":483,"record_offset":15460,"parent_record_offset":15432,"ancestor_record_offset":12800,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":17356,"parent_record_offset":17328,"ancestor_record_offset":12800,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":19692,"parent_record_offset":19656,"ancestor_record_offset":12800,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":13440,"parent_record_offset":13416,"ancestor_record_offset":12800,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":12800,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":12800,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":12800,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":483,"record_offset":19748,"parent_record_offset":19656,"ancestor_record_offset":12800,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":483,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":12800,"inlinee":21875,"source_line":44,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":483,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":12800,"inlinee":21873,"source_line":153,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/spawn_preplaced_trees.rs
//     evidence={"module":483,"record_offset":16960,"parent_record_offset":16920,"ancestor_record_offset":12800,"inlinee":27268,"source_line":52,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":483,"record_offset":19896,"parent_record_offset":19864,"ancestor_record_offset":12800,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":483,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":12800,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":483,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":12800,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":483,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":12800,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13660,"parent_record_offset":13608,"ancestor_record_offset":12800,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":12800,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":12800,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":483,"record_offset":13904,"parent_record_offset":13868,"ancestor_record_offset":12800,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00483.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A84
// 首条位置: {"module":483,"inlinee":27268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":483,"record_offset":16988,"parent_record_offset":16960,"ancestor_record_offset":16960,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00483.jsonl.gz"}
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
// 完整结构及原始类型记录: spawn_preplaced_trees.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spawn_preplaced_trees.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spawn_preplaced_trees.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 信息 ID: 96ceb684691f78985deb75d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)> @ 0x66369e -> 0x68a010 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)> > @ 0xe3ff43 -> 0x68a010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 信息 ID: 19d399204f061ba5950c8f1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees; module=483, inline_record=16960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
