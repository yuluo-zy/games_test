// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c1644e34a1f2cd4ffe4616b3e48c05fcaa6d8e67fea87fbd75f61b87cf75bcd4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 386, 2761, 2772, 2775, 2776, 2777, 3869, 3900
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::impl$0::add_decorator
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":386,"record_offset":536,"rva":5257312,"va":5373966432}
// 编译实例 RVAs: 0x503860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=1
//     target_aliases=1000 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5245728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8e0f4afff","size":5,"soft_fail":false,"target_va":5368722320,"va":5373968043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=2
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5034632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8782db3ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5373971539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5031919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e81138b3ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5373969786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bcaa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2391253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e82b83dbff","size":5,"soft_fail":false,"target_va":5371579040,"va":5373970288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bcbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2391147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e89583dbff","size":5,"soft_fail":false,"target_va":5371579360,"va":5373970502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2377541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8bbb8dbff","size":5,"soft_fail":false,"target_va":5371593280,"va":5373970816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2374683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8e5c3dbff","size":5,"soft_fail":false,"target_va":5371596368,"va":5373971046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2270936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e82859ddff","size":5,"soft_fail":false,"target_va":5371700208,"va":5373971139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402daba0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2269269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8ab5fddff","size":5,"soft_fail":false,"target_va":5371702176,"va":5373971440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402dac90
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2268824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e86861ddff","size":5,"soft_fail":false,"target_va":5371702416,"va":5373971235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=29
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1342252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8d484ebff","size":5,"soft_fail":false,"target_va":5372625024,"va":5373967271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7130
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1911131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e85b291d00","size":5,"soft_fail":false,"target_va":5375881520,"va":5373970384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b0cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4899752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8a8c34a00","size":5,"soft_fail":false,"target_va":5378870464,"va":5373970707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8342155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e88b4a7f00","size":5,"soft_fail":false,"target_va":5382313504,"va":5373971344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8343141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8654e7f00","size":5,"soft_fail":false,"target_va":5382313760,"va":5373970614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d15290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8457568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8600d8100","size":5,"soft_fail":false,"target_va":5382427280,"va":5373969707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d168c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8463229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e87d238100","size":5,"soft_fail":false,"target_va":5382432960,"va":5373969726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d1a760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8479246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e80e628100","size":5,"soft_fail":false,"target_va":5382448992,"va":5373969741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=23
//     evidence={"assembly":"\tcallq\t8446114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8a2e08000","size":5,"soft_fail":false,"target_va":5382413360,"va":5373967241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8444373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8d5d98000","size":5,"soft_fail":false,"target_va":5382414096,"va":5373969718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8316567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e897e67e00","size":5,"soft_fail":false,"target_va":5382283296,"va":5373966724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8484083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8f3748100","size":5,"soft_fail":false,"target_va":5382453856,"va":5373969768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8488637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8bd868100","size":5,"soft_fail":false,"target_va":5382458544,"va":5373969902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8452805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e8c5fa8000","size":5,"soft_fail":false,"target_va":5382422496,"va":5373969686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::h83c4c6090bd0392f (.llvm.8483650753040762543)
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8445751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e837df8000","size":5,"soft_fail":false,"target_va":5382415424,"va":5373969668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=29
//     evidence={"assembly":"\tcallq\t3398970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373966432,"raw_hex":"e83add3300","size":5,"soft_fail":false,"target_va":5377366224,"va":5373967249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":386,"record_offset":748,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":386,"record_offset":20420,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":386,"record_offset":20568,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":386,"record_offset":18464,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21827,"source_line":10,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":386,"record_offset":9684,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21805,"source_line":10,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$94::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":386,"record_offset":5392,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21791,"source_line":416,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":386,"record_offset":5916,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21795,"source_line":41,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":386,"record_offset":15404,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21823,"source_line":4,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":386,"record_offset":5420,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21793,"source_line":26,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":386,"record_offset":11692,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21815,"source_line":63,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$27::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":386,"record_offset":12180,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21817,"source_line":107,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":386,"record_offset":11172,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21811,"source_line":13,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":386,"record_offset":19928,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21833,"source_line":28,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":386,"record_offset":10232,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21809,"source_line":34,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::backup::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/backup.rs
//     evidence={"module":386,"record_offset":16376,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21825,"source_line":17,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":386,"record_offset":14396,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21819,"source_line":220,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":386,"record_offset":14884,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21821,"source_line":225,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::chimney_assemble_bricks::impl$11::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_assemble_bricks.rs
//     evidence={"module":386,"record_offset":19440,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21831,"source_line":57,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::chimney_assemble_bricks::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_assemble_bricks.rs
//     evidence={"module":386,"record_offset":18952,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21829,"source_line":54,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":386,"record_offset":8572,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21799,"source_line":60,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":386,"record_offset":8048,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21797,"source_line":23,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_decorator_affordance_dispatch::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs
//     evidence={"module":386,"record_offset":11664,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21813,"source_line":79,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_move_decorator::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_move_decorator.rs
//     evidence={"module":386,"record_offset":10204,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":21807,"source_line":69,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":386,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":536,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":386,"record_offset":20444,"parent_record_offset":20420,"ancestor_record_offset":536,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":386,"record_offset":20500,"parent_record_offset":20472,"ancestor_record_offset":536,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":386,"record_offset":20472,"parent_record_offset":20444,"ancestor_record_offset":536,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":386,"record_offset":832,"parent_record_offset":800,"ancestor_record_offset":536,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":386,"record_offset":20608,"parent_record_offset":20568,"ancestor_record_offset":536,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":386,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":536,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":386,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":536,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":386,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":536,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":386,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":536,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":386,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":536,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":386,"record_offset":2008,"parent_record_offset":1944,"ancestor_record_offset":536,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":386,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":536,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":386,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":536,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":386,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":536,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":386,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":536,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":386,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":536,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":386,"record_offset":5748,"parent_record_offset":5720,"ancestor_record_offset":536,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":386,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":536,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":386,"record_offset":5776,"parent_record_offset":5720,"ancestor_record_offset":536,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":386,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":536,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":386,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":536,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":386,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":536,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":386,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":536,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":386,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":536,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":386,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":536,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":386,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":536,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":386,"record_offset":1820,"parent_record_offset":800,"ancestor_record_offset":536,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":386,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":536,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":386,"record_offset":1788,"parent_record_offset":832,"ancestor_record_offset":536,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":386,"record_offset":2072,"parent_record_offset":1892,"ancestor_record_offset":536,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":386,"record_offset":7820,"parent_record_offset":5916,"ancestor_record_offset":536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":386,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":536,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":386,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":536,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":386,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":536,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":386,"record_offset":1944,"parent_record_offset":1916,"ancestor_record_offset":536,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":386,"record_offset":1980,"parent_record_offset":1944,"ancestor_record_offset":536,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":386,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":536,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":386,"record_offset":17360,"parent_record_offset":16376,"ancestor_record_offset":536,"inlinee":21805,"source_line":10,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$5::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":386,"record_offset":9716,"parent_record_offset":9684,"ancestor_record_offset":536,"inlinee":9065,"source_line":15,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":386,"record_offset":16424,"parent_record_offset":16376,"ancestor_record_offset":536,"inlinee":21793,"source_line":26,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":386,"record_offset":17856,"parent_record_offset":16376,"ancestor_record_offset":536,"inlinee":21815,"source_line":63,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":386,"record_offset":15440,"parent_record_offset":15404,"ancestor_record_offset":536,"inlinee":10763,"source_line":234,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":386,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":536,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":386,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":536,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":386,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":536,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":386,"record_offset":1480,"parent_record_offset":1284,"ancestor_record_offset":536,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":386,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":536,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":386,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":536,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":1392,"parent_record_offset":1308,"ancestor_record_offset":536,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":1336,"parent_record_offset":1308,"ancestor_record_offset":536,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":1736,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":536,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":536,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":536,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":536,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":386,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":536,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":386,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":536,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":386,"record_offset":1252,"parent_record_offset":1136,"ancestor_record_offset":536,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":386,"record_offset":1136,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":386,"record_offset":1284,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":386,"record_offset":996,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":386,"record_offset":1080,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":386,"record_offset":1652,"parent_record_offset":956,"ancestor_record_offset":536,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":386,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":536,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":386,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":536,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":386,"record_offset":6396,"parent_record_offset":5964,"ancestor_record_offset":536,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":386,"record_offset":5456,"parent_record_offset":5420,"ancestor_record_offset":536,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":386,"record_offset":6428,"parent_record_offset":5916,"ancestor_record_offset":536,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":386,"record_offset":5512,"parent_record_offset":5484,"ancestor_record_offset":536,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":386,"record_offset":5600,"parent_record_offset":5568,"ancestor_record_offset":536,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":386,"record_offset":5720,"parent_record_offset":5568,"ancestor_record_offset":536,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":386,"record_offset":5484,"parent_record_offset":5456,"ancestor_record_offset":536,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":386,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":536,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":386,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":536,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":386,"record_offset":5568,"parent_record_offset":5540,"ancestor_record_offset":536,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":386,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":536,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":386,"record_offset":14916,"parent_record_offset":14884,"ancestor_record_offset":536,"inlinee":21819,"source_line":220,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":386,"record_offset":8608,"parent_record_offset":8572,"ancestor_record_offset":536,"inlinee":21801,"source_line":19,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":386,"record_offset":9076,"parent_record_offset":8572,"ancestor_record_offset":536,"inlinee":21803,"source_line":24,"archive":"_pdb/A/modules/00386.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorators_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3869,"record_offset":256,"rva":29813184,"va":5398522304}
// 编译实例 RVAs: 0x1c6e9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=1
//     target_aliases=1000 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29801158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e83a4539fe","size":5,"soft_fail":false,"target_va":5368722320,"va":5398523473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ff9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28768684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8540649fe","size":5,"soft_fail":false,"target_va":5369756144,"va":5398524823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bcaa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26943442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e82ee064fe","size":5,"soft_fail":false,"target_va":5371579040,"va":5398522477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bcbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26945889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e89fd664fe","size":5,"soft_fail":false,"target_va":5371579360,"va":5398525244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26931850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8760d65fe","size":5,"soft_fail":false,"target_va":5371593872,"va":5398525717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26825390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e852ad66fe","size":5,"soft_fail":false,"target_va":5371700208,"va":5398525593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26289883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e825d96efe","size":5,"soft_fail":false,"target_va":5372234512,"va":5398524390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140393060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26065893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e81b4472fe","size":5,"soft_fail":false,"target_va":5372457056,"va":5398522944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=16
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25897389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e853d674fe","size":5,"soft_fail":false,"target_va":5372625024,"va":5398522408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25772841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8d7bc76fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398522548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7130
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22643595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8757ca6fe","size":5,"soft_fail":false,"target_va":5375881520,"va":5398525110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b0cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19655022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e89216d4fe","size":5,"soft_fail":false,"target_va":5378870464,"va":5398525481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf91a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16212252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8e49e08ff","size":5,"soft_fail":false,"target_va":5382312352,"va":5398524599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16211610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e866a108ff","size":5,"soft_fail":false,"target_va":5382313760,"va":5398525365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d05030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16162214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e85a6209ff","size":5,"soft_fail":false,"target_va":5382361136,"va":5398523345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5070387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8335e4d00","size":5,"soft_fail":false,"target_va":5403593360,"va":5398522968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t-21156167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398522304,"raw_hex":"e8b92ebdfe","size":5,"soft_fail":false,"target_va":5377366224,"va":5398522386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3869,"record_offset":896,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3869,"record_offset":956,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":3869,"record_offset":13000,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21827,"source_line":10,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":3869,"record_offset":1852,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21795,"source_line":41,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":3869,"record_offset":9824,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21823,"source_line":4,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3869,"record_offset":428,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21793,"source_line":26,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":3869,"record_offset":11844,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10761,"source_line":15,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":3869,"record_offset":8756,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21811,"source_line":13,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":3869,"record_offset":6768,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21809,"source_line":34,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":3869,"record_offset":5460,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21799,"source_line":60,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":3869,"record_offset":4056,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21797,"source_line":23,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs
//     evidence={"module":3869,"record_offset":13740,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":111926,"source_line":86,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3869,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3869,"record_offset":1580,"parent_record_offset":1300,"ancestor_record_offset":256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3869,"record_offset":4772,"parent_record_offset":4740,"ancestor_record_offset":256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3869,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3869,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3869,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3869,"record_offset":780,"parent_record_offset":744,"ancestor_record_offset":256,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3869,"record_offset":2200,"parent_record_offset":2144,"ancestor_record_offset":256,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3869,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3869,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3869,"record_offset":1480,"parent_record_offset":1420,"ancestor_record_offset":256,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3869,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":256,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3869,"record_offset":4804,"parent_record_offset":4772,"ancestor_record_offset":256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3869,"record_offset":744,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3869,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3869,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3869,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3869,"record_offset":1520,"parent_record_offset":1332,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3869,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::default
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":3869,"record_offset":9860,"parent_record_offset":9824,"ancestor_record_offset":256,"inlinee":10763,"source_line":234,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3869,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":256,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1332,"parent_record_offset":1300,"ancestor_record_offset":256,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":256,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1300,"parent_record_offset":1244,"ancestor_record_offset":256,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1272,"parent_record_offset":1244,"ancestor_record_offset":256,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1392,"parent_record_offset":1332,"ancestor_record_offset":256,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3869,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":256,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":3869,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":256,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3869,"record_offset":2332,"parent_record_offset":1900,"ancestor_record_offset":256,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3869,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":3869,"record_offset":2364,"parent_record_offset":1852,"ancestor_record_offset":256,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3869,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":256,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3869,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":256,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3869,"record_offset":688,"parent_record_offset":548,"ancestor_record_offset":256,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3869,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":256,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3869,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":256,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3869,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3869,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":256,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3869,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":256,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":3869,"record_offset":5500,"parent_record_offset":5460,"ancestor_record_offset":256,"inlinee":21801,"source_line":19,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":3869,"record_offset":5968,"parent_record_offset":5460,"ancestor_record_offset":256,"inlinee":21803,"source_line":24,"archive":"_pdb/A/modules/03869.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::impl$1::snapping_direction
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F66
// 首条位置: {"module":2775,"inlinee":89958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::try_merge_decorators
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F41
// 首条位置: {"module":2772,"inlinee":89921}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":2396,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2632,"parent_record_offset":2396,"ancestor_record_offset":2396,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2396,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":2396,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2772,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2396,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":2772,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":2396,"inlinee":28256,"source_line":104,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2396,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":2772,"record_offset":2780,"parent_record_offset":2728,"ancestor_record_offset":2396,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":2396,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2772,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":2396,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2772,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2396,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2396,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":2396,"inlinee":89922,"source_line":296,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::try_merge_decorators::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2772,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2396,"inlinee":89889,"source_line":301,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::try_merge_decorators::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F42
// 首条位置: {"module":2772,"inlinee":89922}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2772,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2532,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$19::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2772,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2532,"inlinee":25468,"source_line":17,"archive":"_pdb/A/modules/02772.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::try_merge_decorators::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F21
// 首条位置: {"module":2761,"inlinee":89889}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>", "bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>", "bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>", "bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>

// ------------------------------------------------------------------
// 函数: system_decorator::decorators_on_glade_unload
// 信息 ID: f63e51f87b35b1f97d30ef7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ab605e2930b9f10c496a2751: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>)> > @ 0x17c5b1b -> 0x1c6e9c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>)> @ 0x17dd8e6 -> 0x1c6e9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::impl$0::add_decorator
// 信息 ID: 4a120ab4429b04dc831d2b92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d1b86 -> 0x503860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::impl$1::snapping_direction
// 信息 ID: 612c1ffac5291441f3bbf851
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=1416; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=2984; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=4244; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=5508; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=3100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::try_merge_decorators
// 信息 ID: 48645eaeb35e63534b1a15a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position; module=2772, inline_record=2396; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=2644; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>; module=2777, inline_record=1896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::try_merge_decorators::closure$0
// 信息 ID: dc900a9781f5060df4050a03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::try_merge_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position; module=2772, inline_record=2532; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=2772; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>; module=2777, inline_record=2032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::try_merge_decorators::closure$1
// 信息 ID: 29bca0788b6911a6a5fa59de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::try_merge_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position; module=2772, inline_record=2752; 不是独立调用边。
// [PDB 内联宿主] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner; module=2776, inline_record=2992; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>; module=2777, inline_record=2252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
