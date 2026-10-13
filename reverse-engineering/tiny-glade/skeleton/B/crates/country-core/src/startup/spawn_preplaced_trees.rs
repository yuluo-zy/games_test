// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/spawn_preplaced_trees.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9f0d8be23c1aa96161f82dc732f148843816ed661f545f59db14e2660520fa93
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_preplaced_trees.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 481
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":481,"record_offset":2468,"rva":8790640,"va":5377499760}
// 编译实例 RVAs: 0x862270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5144350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e81e7f4e00","size":5,"soft_fail":false,"target_va":5382644528,"va":5377500173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2602890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e87648d8ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5377503621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2600807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e89950d8ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5377505282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2418845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e86317dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377500344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140663570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2098010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8a6fcdfff","size":5,"soft_fail":false,"target_va":5375407472,"va":5377505477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1983972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e81cbae1ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377499999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t91208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e848640100","size":5,"soft_fail":false,"target_va":5377593904,"va":5377502691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5327470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e86e4a5100","size":5,"soft_fail":false,"target_va":5382827728,"va":5377500253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eec970
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6856023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8579d6800","size":5,"soft_fail":false,"target_va":5384358256,"va":5377502228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e68e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11026734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e82e41a800","size":5,"soft_fail":false,"target_va":5388527840,"va":5377501101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12203241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8e934ba00","size":5,"soft_fail":false,"target_va":5389704432,"va":5377501186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414062b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12205508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8c43dba00","size":5,"soft_fail":false,"target_va":5389705904,"va":5377500391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141556080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13580350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e83e38cf00","size":5,"soft_fail":false,"target_va":5391081600,"va":5377501245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3143263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8a109d0ff","size":5,"soft_fail":false,"target_va":5374358848,"va":5377502106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33844724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8f46d0402","size":5,"soft_fail":false,"target_va":5411347040,"va":5377502311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2179529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e837bedeff","size":5,"soft_fail":false,"target_va":5375322928,"va":5377502452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4922867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8f31d4b00","size":5,"soft_fail":false,"target_va":5382424224,"va":5377501352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10994042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e87ac1a700","size":5,"soft_fail":false,"target_va":5388495600,"va":5377501553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34039254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8d6650702","size":5,"soft_fail":false,"target_va":5411542064,"va":5377502805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34038806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e816640702","size":5,"soft_fail":false,"target_va":5411541904,"va":5377503093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34040011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8cb680702","size":5,"soft_fail":false,"target_va":5411542880,"va":5377502864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5205153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8a16c4f00","size":5,"soft_fail":false,"target_va":5382705472,"va":5377500314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5243830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377499760,"raw_hex":"e8b6035000","size":5,"soft_fail":false,"target_va":5382744080,"va":5377500245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$90::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":481,"record_offset":5704,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":26646,"source_line":2533,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":5512,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":481,"record_offset":5568,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":481,"record_offset":7744,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":26267,"source_line":10,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":481,"record_offset":9536,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":481,"record_offset":3284,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":481,"record_offset":2720,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":481,"record_offset":8944,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7396,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7860,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":481,"record_offset":3560,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":481,"record_offset":7368,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":481,"record_offset":4972,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":481,"record_offset":4884,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":481,"record_offset":7088,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":481,"record_offset":6980,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":481,"record_offset":7132,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":481,"record_offset":7216,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":481,"record_offset":3768,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":26641,"source_line":269,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::TreeStorage::insert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":481,"record_offset":9592,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":26648,"source_line":66,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":481,"record_offset":8468,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":481,"record_offset":3592,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":481,"record_offset":3476,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":481,"record_offset":4052,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":4020,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":3880,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":481,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2468,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":481,"record_offset":4480,"parent_record_offset":4424,"ancestor_record_offset":2468,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":9168,"parent_record_offset":9136,"ancestor_record_offset":2468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":2468,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":2468,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":4452,"parent_record_offset":4424,"ancestor_record_offset":2468,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":2468,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":7648,"parent_record_offset":7616,"ancestor_record_offset":2468,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":2468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":2468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":7616,"parent_record_offset":7588,"ancestor_record_offset":2468,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":481,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":2468,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":481,"record_offset":9824,"parent_record_offset":9788,"ancestor_record_offset":2468,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":481,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":2468,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":481,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":2468,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":9136,"parent_record_offset":9088,"ancestor_record_offset":2468,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2468,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":9088,"parent_record_offset":9044,"ancestor_record_offset":2468,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":7560,"parent_record_offset":7524,"ancestor_record_offset":2468,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":481,"record_offset":2932,"parent_record_offset":2832,"ancestor_record_offset":2468,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":481,"record_offset":9016,"parent_record_offset":8976,"ancestor_record_offset":2468,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":481,"record_offset":8976,"parent_record_offset":8944,"ancestor_record_offset":2468,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":481,"record_offset":9044,"parent_record_offset":8976,"ancestor_record_offset":2468,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":481,"record_offset":2800,"parent_record_offset":2752,"ancestor_record_offset":2468,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":481,"record_offset":2832,"parent_record_offset":2800,"ancestor_record_offset":2468,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":481,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":2468,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":2468,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7524,"parent_record_offset":7492,"ancestor_record_offset":2468,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7436,"parent_record_offset":7396,"ancestor_record_offset":2468,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":481,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":2468,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":481,"record_offset":2752,"parent_record_offset":2720,"ancestor_record_offset":2468,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":481,"record_offset":9680,"parent_record_offset":9592,"ancestor_record_offset":2468,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":481,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":2468,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":481,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":2468,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":481,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":2468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":481,"record_offset":3352,"parent_record_offset":3316,"ancestor_record_offset":2468,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":481,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2468,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":481,"record_offset":2876,"parent_record_offset":2832,"ancestor_record_offset":2468,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":481,"record_offset":10084,"parent_record_offset":10056,"ancestor_record_offset":2468,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":481,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":2468,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":481,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":2468,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":481,"record_offset":9708,"parent_record_offset":9592,"ancestor_record_offset":2468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":481,"record_offset":3440,"parent_record_offset":3316,"ancestor_record_offset":2468,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":481,"record_offset":3648,"parent_record_offset":3616,"ancestor_record_offset":2468,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":481,"record_offset":3676,"parent_record_offset":3616,"ancestor_record_offset":2468,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":481,"record_offset":4256,"parent_record_offset":4052,"ancestor_record_offset":2468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":481,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":2468,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":481,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":2468,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":481,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":2468,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":481,"record_offset":5644,"parent_record_offset":5568,"ancestor_record_offset":2468,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":481,"record_offset":5608,"parent_record_offset":5568,"ancestor_record_offset":2468,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":481,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":2468,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":481,"record_offset":9256,"parent_record_offset":9088,"ancestor_record_offset":2468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":481,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":2468,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":481,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":2468,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":481,"record_offset":2988,"parent_record_offset":2832,"ancestor_record_offset":2468,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":481,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2468,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":481,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2468,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":481,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":2468,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":481,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":2468,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":481,"record_offset":7280,"parent_record_offset":7216,"ancestor_record_offset":2468,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":481,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":2468,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":2468,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":9416,"parent_record_offset":9380,"ancestor_record_offset":2468,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":2468,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":2468,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":2468,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":2468,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":481,"record_offset":9472,"parent_record_offset":9380,"ancestor_record_offset":2468,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::TreeStorageIndex::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":481,"record_offset":9652,"parent_record_offset":9592,"ancestor_record_offset":2468,"inlinee":26334,"source_line":59,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":481,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":2468,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":481,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":2468,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/spawn_preplaced_trees.rs
//     evidence={"module":481,"record_offset":7024,"parent_record_offset":6980,"ancestor_record_offset":2468,"inlinee":26650,"source_line":53,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":481,"record_offset":3704,"parent_record_offset":3616,"ancestor_record_offset":2468,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":481,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":2468,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":481,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":2468,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":481,"record_offset":9624,"parent_record_offset":9592,"ancestor_record_offset":2468,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":481,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":2468,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":481,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":2468,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":481,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":2468,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":3844,"parent_record_offset":3792,"ancestor_record_offset":2468,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":2468,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":2468,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":481,"record_offset":4088,"parent_record_offset":4052,"ancestor_record_offset":2468,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00481.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x681A
// 首条位置: {"module":481,"inlinee":26650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":481,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":7024,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00481.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 信息 ID: 9668e65972e7a8937ab6807b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2b68b79ac4bfb711ecb9cf37: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)> > @ 0x9be21e -> 0x862270 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)> > > @ 0x14d2ee6 -> 0x862270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 信息 ID: 7f1228a58079d32972108722
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees; module=481, inline_record=7024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
