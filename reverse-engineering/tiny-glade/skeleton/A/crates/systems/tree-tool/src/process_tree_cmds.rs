// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/tree-tool/src/process_tree_cmds.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 59fcebf48a0c5b24ca92aa8f0d53df10a1cabeb25ddc88ed8cfb31d92a41caa3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_tree_cmds.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3381
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::process_tree_cmds::process_tree_cmds
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3381,"record_offset":496,"rva":27851936,"va":5396561056}
// 编译实例 RVAs: 0x1a8fca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24913455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8d1d983fe","size":5,"soft_fail":false,"target_va":5371652976,"va":5396566426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23411367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e859c59afe","size":5,"soft_fail":false,"target_va":5373149840,"va":5396561202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22308232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8789aabfe","size":5,"soft_fail":false,"target_va":5374255072,"va":5396563299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19622009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e88797d4fe","size":5,"soft_fail":false,"target_va":5376939904,"va":5396561908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141024e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10926669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8b34559ff","size":5,"soft_fail":false,"target_va":5385637408,"va":5396564072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7030155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e88b456b00","size":5,"soft_fail":false,"target_va":5403593344,"va":5396563184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t6353822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e89ef36000","size":5,"soft_fail":false,"target_va":5402919072,"va":5396565245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-25257283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8bd9a7efe","size":5,"soft_fail":false,"target_va":5371305232,"va":5396562510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19320406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8aa31d9fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5396565697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6405786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e89abe6100","size":5,"soft_fail":false,"target_va":5402972080,"va":5396566289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3321680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e850af3200","size":5,"soft_fail":false,"target_va":5399885072,"va":5396563387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-19317151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8613ed9fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5396563674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6412620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e84cd96100","size":5,"soft_fail":false,"target_va":5402978880,"va":5396566255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6413880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e838de6100","size":5,"soft_fail":false,"target_va":5402980128,"va":5396566243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_flowery
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22307931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8a59babfe","size":5,"soft_fail":false,"target_va":5374255088,"va":5396563014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_winter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22308194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e89e9aabfe","size":5,"soft_fail":false,"target_va":5374255120,"va":5396563309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::tree_storage::TreeStorage::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21542212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8bc4ab7fe","size":5,"soft_fail":false,"target_va":5375019472,"va":5396561679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::tree_storage::TreeStorage::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21544592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e87041b7fe","size":5,"soft_fail":false,"target_va":5375019728,"va":5396564315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::tree_storage::TreeStorage::remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21545877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e86b3cb7fe","size":5,"soft_fail":false,"target_va":5375019792,"va":5396565664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19211384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e888dbdafe","size":5,"soft_fail":false,"target_va":5377353776,"va":5396565155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_tree::ui_place_tree::random_tree_prefab
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8d4160000","size":5,"soft_fail":false,"target_va":5396571072,"va":5396565223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-19622483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396561056,"raw_hex":"e8ad95d4fe","size":5,"soft_fail":false,"target_va":5376939952,"va":5396562430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$82::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3381,"record_offset":5228,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":27266,"source_line":2321,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":1892,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":3028,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":756,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":1304,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":1580,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":12416,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":12360,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":6128,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3381,"record_offset":12684,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3381,"record_offset":1984,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3381,"record_offset":11092,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3381,"record_offset":9268,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3381,"record_offset":11120,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3381,"record_offset":1948,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3381,"record_offset":13632,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3381,"record_offset":5200,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":27887,"source_line":220,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3381,"record_offset":6072,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":27890,"source_line":215,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3381,"record_offset":6100,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3381,"record_offset":728,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3381,"record_offset":8096,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3381,"record_offset":5088,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3381,"record_offset":11148,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3381,"record_offset":11008,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3381,"record_offset":3084,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3381,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":496,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3381,"record_offset":6416,"parent_record_offset":6380,"ancestor_record_offset":496,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3381,"record_offset":1452,"parent_record_offset":1420,"ancestor_record_offset":496,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3381,"record_offset":6380,"parent_record_offset":6348,"ancestor_record_offset":496,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3381,"record_offset":9664,"parent_record_offset":9640,"ancestor_record_offset":496,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":1420,"parent_record_offset":1376,"ancestor_record_offset":496,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":6496,"parent_record_offset":6300,"ancestor_record_offset":496,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":1376,"parent_record_offset":1332,"ancestor_record_offset":496,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":6348,"parent_record_offset":6300,"ancestor_record_offset":496,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":6664,"parent_record_offset":6300,"ancestor_record_offset":496,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3381,"record_offset":2196,"parent_record_offset":2052,"ancestor_record_offset":496,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":496,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":1520,"parent_record_offset":1304,"ancestor_record_offset":496,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":496,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3381,"record_offset":2052,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":496,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":496,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":820,"parent_record_offset":780,"ancestor_record_offset":496,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":496,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":13540,"parent_record_offset":12708,"ancestor_record_offset":496,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":13172,"parent_record_offset":12708,"ancestor_record_offset":496,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":13196,"parent_record_offset":13172,"ancestor_record_offset":496,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":2936,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":2472,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":23285,"source_line":1276,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":496,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3381,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":496,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3381,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":496,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3381,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":496,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3381,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":496,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3381,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":496,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3381,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":496,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":2900,"parent_record_offset":2496,"ancestor_record_offset":496,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":2844,"parent_record_offset":2496,"ancestor_record_offset":496,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":2528,"parent_record_offset":2496,"ancestor_record_offset":496,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":2444,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":6208,"parent_record_offset":6168,"ancestor_record_offset":496,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":12448,"parent_record_offset":12416,"ancestor_record_offset":496,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":6252,"parent_record_offset":6208,"ancestor_record_offset":496,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":12504,"parent_record_offset":12448,"ancestor_record_offset":496,"inlinee":21464,"source_line":216,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3381,"record_offset":6168,"parent_record_offset":6128,"ancestor_record_offset":496,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3381,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":496,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3381,"record_offset":6696,"parent_record_offset":6252,"ancestor_record_offset":496,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3381,"record_offset":6300,"parent_record_offset":6252,"ancestor_record_offset":496,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3381,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":496,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3381,"record_offset":2260,"parent_record_offset":2228,"ancestor_record_offset":496,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3381,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":496,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3381,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":496,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3381,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":496,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3381,"record_offset":3472,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3381,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":496,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3381,"record_offset":3352,"parent_record_offset":3296,"ancestor_record_offset":496,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3381,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3381,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":496,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3381,"record_offset":1632,"parent_record_offset":1604,"ancestor_record_offset":496,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3381,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":496,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3381,"record_offset":892,"parent_record_offset":844,"ancestor_record_offset":496,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3381,"record_offset":2140,"parent_record_offset":2052,"ancestor_record_offset":496,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3381,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":496,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3381,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":496,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3381,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":496,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3381,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":496,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3381,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":496,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3381,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":496,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3381,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":496,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3381,"record_offset":1712,"parent_record_offset":1632,"ancestor_record_offset":496,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3381,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":496,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3381,"record_offset":9568,"parent_record_offset":9428,"ancestor_record_offset":496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3381,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":496,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3381,"record_offset":6580,"parent_record_offset":6300,"ancestor_record_offset":496,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3381,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":496,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3381,"record_offset":1484,"parent_record_offset":1376,"ancestor_record_offset":496,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3381,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":496,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3381,"record_offset":2416,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3381,"record_offset":5300,"parent_record_offset":5276,"ancestor_record_offset":496,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3381,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":496,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3381,"record_offset":956,"parent_record_offset":844,"ancestor_record_offset":496,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3381,"record_offset":1148,"parent_record_offset":844,"ancestor_record_offset":496,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3381,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":496,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3381,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":496,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3381,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":496,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3381,"record_offset":996,"parent_record_offset":956,"ancestor_record_offset":496,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3381,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":496,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3381,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":496,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3381,"record_offset":1788,"parent_record_offset":1736,"ancestor_record_offset":496,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3381,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":496,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3381,"record_offset":9700,"parent_record_offset":9664,"ancestor_record_offset":496,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3381,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":496,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3381,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":496,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3381,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":496,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3381,"record_offset":9756,"parent_record_offset":9664,"ancestor_record_offset":496,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3381,"record_offset":3596,"parent_record_offset":3556,"ancestor_record_offset":496,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3381,"record_offset":2228,"parent_record_offset":2008,"ancestor_record_offset":496,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3381,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":496,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3381,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":496,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3381,"record_offset":2288,"parent_record_offset":2260,"ancestor_record_offset":496,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3444,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3272,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":496,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3216,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3864,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3160,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3381,"record_offset":3388,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3381,"record_offset":3528,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3381,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":496,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3381,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":496,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3381,"record_offset":3556,"parent_record_offset":3084,"ancestor_record_offset":496,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::process_tree_cmds::despawn_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3381,"record_offset":13692,"rva":27857408,"va":5396566528}
// 编译实例 RVAs: 0x1a91200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25261919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396566528,"raw_hex":"e8a1887efe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396566794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":14424,"parent_record_offset":13692,"ancestor_record_offset":13692,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3381,"record_offset":14028,"parent_record_offset":13692,"ancestor_record_offset":13692,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3381,"record_offset":13832,"parent_record_offset":13692,"ancestor_record_offset":13692,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3381,"record_offset":14612,"parent_record_offset":14580,"ancestor_record_offset":13692,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":14580,"parent_record_offset":14540,"ancestor_record_offset":13692,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3381,"record_offset":14540,"parent_record_offset":14504,"ancestor_record_offset":13692,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13692,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3381,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":13692,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3381,"record_offset":14504,"parent_record_offset":14424,"ancestor_record_offset":13692,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3381,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":13692,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3381,"record_offset":13880,"parent_record_offset":13856,"ancestor_record_offset":13692,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3381,"record_offset":13904,"parent_record_offset":13880,"ancestor_record_offset":13692,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3381,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":13692,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3381,"record_offset":13856,"parent_record_offset":13832,"ancestor_record_offset":13692,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3381,"record_offset":14332,"parent_record_offset":14052,"ancestor_record_offset":13692,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":14304,"parent_record_offset":14052,"ancestor_record_offset":13692,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":14380,"parent_record_offset":14356,"ancestor_record_offset":13692,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3381,"record_offset":14220,"parent_record_offset":14052,"ancestor_record_offset":13692,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3381,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13692,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3381,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":13692,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3381,"record_offset":14132,"parent_record_offset":14096,"ancestor_record_offset":13692,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3381,"record_offset":14644,"parent_record_offset":14540,"ancestor_record_offset":13692,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3381,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":13692,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3381,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":13692,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3381,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":13692,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3381,"record_offset":13952,"parent_record_offset":13928,"ancestor_record_offset":13692,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3381,"record_offset":14160,"parent_record_offset":14096,"ancestor_record_offset":13692,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03381.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3381,"record_offset":14096,"parent_record_offset":14052,"ancestor_record_offset":13692,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03381.jsonl.gz"}
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
// 完整结构及原始类型记录: process_tree_cmds.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: process_tree_cmds.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: process_tree_cmds.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>

// ------------------------------------------------------------------
// 函数: system_tree::process_tree_cmds::despawn_trees
// 信息 ID: e4bc47adf6327005e4fd80a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 689fd861cf4b713adfa0ef35: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > @ 0x1416458 -> 0x1a91200 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> @ 0x142e0c9 -> 0x1a91200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::process_tree_cmds::process_tree_cmds
// 信息 ID: 5dd3ff1c33cf3b24c0ffa414
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 62ccb5a5ad4bb9ab3b6d86f8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > @ 0x141872f -> 0x1a8fca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> @ 0x142dc89 -> 0x1a8fca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
