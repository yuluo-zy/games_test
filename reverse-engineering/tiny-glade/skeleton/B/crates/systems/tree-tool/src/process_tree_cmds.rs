// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/tree-tool/src/process_tree_cmds.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ecf8cd21148e56a3f38da52214339767486d34967e9ed0866664ed5c7db7be2d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_tree_cmds.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3660
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_tree; // namespace candidate
//   package identities: [{"name":"system-tree","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::process_tree_cmds::process_tree_cmds
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3660,"record_offset":488,"rva":36074544,"va":5404783664}
// 编译实例 RVAs: 0x2267430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22139993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8a72baefe","size":5,"soft_fail":false,"target_va":5382644528,"va":5404784516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29267777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8bf6841fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5404783804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26448911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8f16b6cfe","size":5,"soft_fail":false,"target_va":5378336416,"va":5404785322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25504429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e853d57afe","size":5,"soft_fail":false,"target_va":5379280368,"va":5404784792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a38460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25358921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8b70d7dfe","size":5,"soft_fail":false,"target_va":5379425376,"va":5404784292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a9fa70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24936803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e89d7e83fe","size":5,"soft_fail":false,"target_va":5379848816,"va":5404785614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ae8320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24639677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8430788fe","size":5,"soft_fail":false,"target_va":5380145952,"va":5404785624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22901724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8248ca2fe","size":5,"soft_fail":false,"target_va":5381883040,"va":5404784759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141556080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13704631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e849e22eff","size":5,"soft_fail":false,"target_va":5391081600,"va":5404786226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: process_tree_cmds.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7061611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e86bc06b00","size":5,"soft_fail":false,"target_va":5411847152,"va":5404785536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-30428334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e852b32ffe","size":5,"soft_fail":false,"target_va":5374356528,"va":5404784857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t6980729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e879846a00","size":5,"soft_fail":false,"target_va":5411767792,"va":5404787058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-29465404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8c4643efe","size":5,"soft_fail":false,"target_va":5375322928,"va":5404788327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22533056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8402ca8fe","size":5,"soft_fail":false,"target_va":5382254496,"va":5404787547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22361479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e879caaafe","size":5,"soft_fail":false,"target_va":5382424224,"va":5404785698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6754748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e8bc116700","size":5,"soft_fail":false,"target_va":5411542880,"va":5404788127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::tree_storage::TreeStorage::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25360869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e81b067dfe","size":5,"soft_fail":false,"target_va":5379425632,"va":5404786496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::tree_storage::TreeStorage::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25361823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e861027dfe","size":5,"soft_fail":false,"target_va":5379425696,"va":5404787514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_tree::ui_place_tree::random_tree_prefab
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-tree","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404783664,"raw_hex":"e81e200000","size":5,"soft_fail":false,"target_va":5404795248,"va":5404787021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$90::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3660,"record_offset":5736,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":26646,"source_line":2533,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":2144,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":4140,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3660,"record_offset":1804,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3660,"record_offset":744,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3660,"record_offset":1452,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":10968,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":10940,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":24591,"source_line":455,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":6700,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":7040,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":11324,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":2492,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3660,"record_offset":2284,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3660,"record_offset":8184,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3660,"record_offset":12840,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3660,"record_offset":5708,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":31632,"source_line":289,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3660,"record_offset":6644,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":24820,"source_line":284,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":3660,"record_offset":6672,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3660,"record_offset":716,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3660,"record_offset":7632,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3660,"record_offset":7688,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3660,"record_offset":5596,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3660,"record_offset":2316,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3660,"record_offset":2200,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":1644,"parent_record_offset":1612,"ancestor_record_offset":488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":488,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":488,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":6944,"parent_record_offset":6912,"ancestor_record_offset":488,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":6912,"parent_record_offset":6884,"ancestor_record_offset":488,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3660,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":488,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":1612,"parent_record_offset":1568,"ancestor_record_offset":488,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":488,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":488,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":1568,"parent_record_offset":1524,"ancestor_record_offset":488,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":6856,"parent_record_offset":6820,"ancestor_record_offset":488,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":1056,"parent_record_offset":832,"ancestor_record_offset":488,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":2912,"parent_record_offset":2580,"ancestor_record_offset":488,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":1768,"parent_record_offset":1480,"ancestor_record_offset":488,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":8244,"parent_record_offset":8212,"ancestor_record_offset":488,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":488,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3660,"record_offset":2580,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3660,"record_offset":1480,"parent_record_offset":1452,"ancestor_record_offset":488,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3660,"record_offset":1524,"parent_record_offset":1480,"ancestor_record_offset":488,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3660,"record_offset":808,"parent_record_offset":768,"ancestor_record_offset":488,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3660,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":488,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3660,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":488,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":488,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":12212,"parent_record_offset":11356,"ancestor_record_offset":488,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":488,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":12664,"parent_record_offset":11356,"ancestor_record_offset":488,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":488,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":3384,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":488,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":3876,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":488,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3660,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":488,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":3488,"parent_record_offset":3464,"ancestor_record_offset":488,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":488,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":3624,"parent_record_offset":3600,"ancestor_record_offset":488,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":488,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":12260,"parent_record_offset":12236,"ancestor_record_offset":488,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":3124,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3660,"record_offset":3812,"parent_record_offset":3408,"ancestor_record_offset":488,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3660,"record_offset":3756,"parent_record_offset":3408,"ancestor_record_offset":488,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":488,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3660,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":488,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":6788,"parent_record_offset":6760,"ancestor_record_offset":488,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":488,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":488,"inlinee":13700,"source_line":266,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":6732,"parent_record_offset":6700,"ancestor_record_offset":488,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3660,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":488,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":2524,"parent_record_offset":2492,"ancestor_record_offset":488,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3660,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":488,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3660,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":488,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3660,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":488,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3660,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":488,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3660,"record_offset":5800,"parent_record_offset":5768,"ancestor_record_offset":488,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3660,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":488,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3660,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":488,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3660,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":488,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3660,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":488,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3660,"record_offset":872,"parent_record_offset":832,"ancestor_record_offset":488,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3660,"record_offset":2856,"parent_record_offset":2580,"ancestor_record_offset":488,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3660,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":488,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3660,"record_offset":5648,"parent_record_offset":5596,"ancestor_record_offset":488,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3660,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":488,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3660,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":488,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":488,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":488,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":3600,"parent_record_offset":3464,"ancestor_record_offset":488,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":1936,"parent_record_offset":1856,"ancestor_record_offset":488,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3660,"record_offset":2372,"parent_record_offset":2340,"ancestor_record_offset":488,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3660,"record_offset":2400,"parent_record_offset":2340,"ancestor_record_offset":488,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3660,"record_offset":8556,"parent_record_offset":8356,"ancestor_record_offset":488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3660,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":488,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3660,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":488,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3660,"record_offset":1732,"parent_record_offset":1568,"ancestor_record_offset":488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3660,"record_offset":4076,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3660,"record_offset":4104,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3660,"record_offset":5832,"parent_record_offset":5800,"ancestor_record_offset":488,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3660,"record_offset":5864,"parent_record_offset":5832,"ancestor_record_offset":488,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3660,"record_offset":928,"parent_record_offset":832,"ancestor_record_offset":488,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3660,"record_offset":1296,"parent_record_offset":832,"ancestor_record_offset":488,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":488,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":2652,"parent_record_offset":2612,"ancestor_record_offset":488,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":3680,"parent_record_offset":3648,"ancestor_record_offset":488,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":488,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":956,"parent_record_offset":928,"ancestor_record_offset":488,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3660,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":488,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":488,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":2012,"parent_record_offset":1960,"ancestor_record_offset":488,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":488,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3660,"record_offset":5768,"parent_record_offset":5736,"ancestor_record_offset":488,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3660,"record_offset":8688,"parent_record_offset":8652,"ancestor_record_offset":488,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3660,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":488,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3660,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":488,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3660,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":488,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3660,"record_offset":8744,"parent_record_offset":8652,"ancestor_record_offset":488,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3660,"record_offset":2944,"parent_record_offset":2524,"ancestor_record_offset":488,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3660,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":488,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3660,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":488,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3660,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":488,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3660,"record_offset":2428,"parent_record_offset":2340,"ancestor_record_offset":488,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3660,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":488,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3660,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":488,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3660,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":488,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::process_tree_cmds::despawn_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3660,"record_offset":12900,"rva":36079312,"va":5404788432}
// 编译实例 RVAs: 0x22686d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-30433797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404788432,"raw_hex":"e8fb9d2ffe","size":5,"soft_fail":false,"target_va":5374354992,"va":5404788784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3660,"record_offset":13444,"parent_record_offset":12900,"ancestor_record_offset":12900,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3660,"record_offset":13900,"parent_record_offset":12900,"ancestor_record_offset":12900,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":13040,"parent_record_offset":12900,"ancestor_record_offset":12900,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":12900,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":12900,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3660,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":12900,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":13668,"parent_record_offset":13608,"ancestor_record_offset":12900,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":12900,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":13228,"parent_record_offset":13196,"ancestor_record_offset":12900,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":12900,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":12900,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":13608,"parent_record_offset":13544,"ancestor_record_offset":12900,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3660,"record_offset":13196,"parent_record_offset":13172,"ancestor_record_offset":12900,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":16164,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":15416,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":15500,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":16444,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3660,"record_offset":15332,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3660,"record_offset":13860,"parent_record_offset":13480,"ancestor_record_offset":12900,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3660,"record_offset":13480,"parent_record_offset":13444,"ancestor_record_offset":12900,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3660,"record_offset":13544,"parent_record_offset":13480,"ancestor_record_offset":12900,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":15088,"parent_record_offset":15064,"ancestor_record_offset":12900,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":15616,"parent_record_offset":15592,"ancestor_record_offset":12900,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":12900,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":12900,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":12900,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3660,"record_offset":15040,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3660,"record_offset":13148,"parent_record_offset":13064,"ancestor_record_offset":12900,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3660,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12900,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3660,"record_offset":13932,"parent_record_offset":13900,"ancestor_record_offset":12900,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3660,"record_offset":15592,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3660,"record_offset":14332,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3660,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":12900,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":14472,"parent_record_offset":14444,"ancestor_record_offset":12900,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":14444,"parent_record_offset":14416,"ancestor_record_offset":12900,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3660,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":12900,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14844,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14416,"parent_record_offset":14384,"ancestor_record_offset":12900,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14384,"parent_record_offset":14356,"ancestor_record_offset":12900,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14248,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":12900,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14164,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3660,"record_offset":14768,"parent_record_offset":14744,"ancestor_record_offset":12900,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3660,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":12900,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":12900,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":12900,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3660,"record_offset":13092,"parent_record_offset":13064,"ancestor_record_offset":12900,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3660,"record_offset":14564,"parent_record_offset":14500,"ancestor_record_offset":12900,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":14588,"parent_record_offset":14564,"ancestor_record_offset":12900,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3660,"record_offset":14628,"parent_record_offset":14444,"ancestor_record_offset":12900,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3660,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":12900,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3660,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":12900,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3660,"record_offset":14048,"parent_record_offset":14012,"ancestor_record_offset":12900,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3660,"record_offset":13820,"parent_record_offset":13608,"ancestor_record_offset":12900,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3660,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":12900,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3660,"record_offset":16360,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":16408,"parent_record_offset":16384,"ancestor_record_offset":12900,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":14212,"parent_record_offset":14188,"ancestor_record_offset":12900,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3660,"record_offset":16384,"parent_record_offset":16360,"ancestor_record_offset":12900,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3660,"record_offset":13368,"parent_record_offset":13196,"ancestor_record_offset":12900,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":12900,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":14012,"parent_record_offset":13932,"ancestor_record_offset":12900,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3660,"record_offset":14076,"parent_record_offset":14012,"ancestor_record_offset":12900,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03660.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3660,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":12900,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03660.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>

// ------------------------------------------------------------------
// 函数: system_tree::process_tree_cmds::despawn_trees
// 信息 ID: 8fe134132988bb9b52038a43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 23789a810da4d2cc79c03c62: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > > @ 0x1bcb60b -> 0x22686d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::TreeAnimation>,ref$<country_core::components::TreeMarkedForDeletion> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > @ 0x1bd83a3 -> 0x22686d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::process_tree_cmds::process_tree_cmds
// 信息 ID: 12d4c59a4209e836c7e7b898
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 70d91eefc505bbcddfe956c1: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > > @ 0x1bc9fbb -> 0x2267430 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::prefab::PrefabInstanceData>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::tree_audio::PlayTreeSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>)> > @ 0x1bd8093 -> 0x2267430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
