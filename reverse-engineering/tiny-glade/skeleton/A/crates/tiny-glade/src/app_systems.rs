// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/app_systems.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 724a8bd9dc7a423694872a78e6353a4aae295dd5fca6fae22f21a87c83a9d1f0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: app_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 62, 66, 128, 248
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_state; // namespace candidate
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
// use system_wall_constructor; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":20748,"rva":2429584,"va":5371138704}
// 编译实例 RVAs: 0x251290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=9
//     target_aliases=84 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2203455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8c160deff","size":5,"soft_fail":false,"target_va":5368936912,"va":5371140362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2201505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e85f68deff","size":5,"soft_fail":false,"target_va":5368937872,"va":5371139372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b6800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1462066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8324f1600","size":5,"soft_fail":false,"target_va":5372602368,"va":5371140297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b6d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1463386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e85a541600","size":5,"soft_fail":false,"target_va":5372603680,"va":5371140289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1465576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8e85c1600","size":5,"soft_fail":false,"target_va":5372605232,"va":5371139651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7630
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1466391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e817601600","size":5,"soft_fail":false,"target_va":5372606000,"va":5371139604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1467318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8b6631600","size":5,"soft_fail":false,"target_va":5372606784,"va":5371139461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1610594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e862931800","size":5,"soft_fail":false,"target_va":5372749696,"va":5371139097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31837813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e875cee501","size":5,"soft_fail":false,"target_va":5402978144,"va":5371140326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t258675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e873f20300","size":5,"soft_fail":false,"target_va":5371398320,"va":5371139640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-2122255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8f19ddfff","size":5,"soft_fail":false,"target_va":5369017104,"va":5371139354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-1944216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e86855e2ff","size":5,"soft_fail":false,"target_va":5369194544,"va":5371138755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::in_set
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t474745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8793e0700","size":5,"soft_fail":false,"target_va":5371614704,"va":5371139954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t823204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371138704,"raw_hex":"e8a48f0c00","size":5,"soft_fail":false,"target_va":5371961952,"va":5371138743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":128,"record_offset":21268,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":128,"record_offset":20896,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":6399,"source_line":444,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":128,"record_offset":21196,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs
//     evidence={"module":128,"record_offset":21396,"parent_record_offset":20748,"ancestor_record_offset":20748,"inlinee":10732,"source_line":84,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":20748,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":21060,"parent_record_offset":21036,"ancestor_record_offset":20748,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":20980,"parent_record_offset":20932,"ancestor_record_offset":20748,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":128,"record_offset":21012,"parent_record_offset":20980,"ancestor_record_offset":20748,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::NodeConfigs::chain_ignore_deferred_inner
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":128,"record_offset":21160,"parent_record_offset":21136,"ancestor_record_offset":20748,"inlinee":6384,"source_line":246,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$10::into_configs
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":128,"record_offset":20932,"parent_record_offset":20896,"ancestor_record_offset":20748,"inlinee":6381,"source_line":530,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::chain_ignore_deferred
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":128,"record_offset":21136,"parent_record_offset":20896,"ancestor_record_offset":20748,"inlinee":6383,"source_line":515,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":128,"record_offset":21236,"parent_record_offset":21196,"ancestor_record_offset":20748,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":21088,"parent_record_offset":21060,"ancestor_record_offset":20748,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F88
// 首条位置: {"module":62,"inlinee":8072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":62,"record_offset":19424,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00062.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_state/src/state/resources.rs
//     evidence={"module":62,"record_offset":19480,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":8074,"source_line":130,"archive":"_pdb/A/modules/00062.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":62,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":19396,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00062.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":128,"record_offset":21968,"rva":2431616,"va":5371140736}
// 编译实例 RVAs: 0x251a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t431791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8af960600","size":5,"soft_fail":false,"target_va":5371581072,"va":5371149276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t444687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e80fc90600","size":5,"soft_fail":false,"target_va":5371593872,"va":5371149180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t447695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8cfd40600","size":5,"soft_fail":false,"target_va":5371596704,"va":5371149004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t550994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e852680800","size":5,"soft_fail":false,"target_va":5371700080,"va":5371149081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=11
//     target_aliases=16 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1482449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8d19e1600","size":5,"soft_fail":false,"target_va":5372625024,"va":5371142570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3250256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e850983100","size":5,"soft_fail":false,"target_va":5374395248,"va":5371144987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t261053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8bdfb0300","size":5,"soft_fail":false,"target_va":5371403296,"va":5371142238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventRegistry::register_event
//     owner=unresolved_namespace; count=29
//     evidence={"assembly":"\tcallq\t-486649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e80793f8ff","size":5,"soft_fail":false,"target_va":5370654368,"va":5371141012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4141560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8f8313f00","size":5,"soft_fail":false,"target_va":5375284496,"va":5371142931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::snapping::SnappingMode::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3181899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e84b8d3000","size":5,"soft_fail":false,"target_va":5374326352,"va":5371144448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3671502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e8ce053800","size":5,"soft_fail":false,"target_va":5374813760,"va":5371142253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t6223671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371140736,"raw_hex":"e837f75e00","size":5,"soft_fail":false,"target_va":5377366224,"va":5371142548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":128,"record_offset":22156,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10733,"source_line":337,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::editing_highlight::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/editing_highlight.rs
//     evidence={"module":128,"record_offset":35536,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10749,"source_line":7,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":128,"record_offset":35508,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10747,"source_line":93,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":128,"record_offset":33048,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10741,"source_line":7,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":128,"record_offset":35040,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10745,"source_line":15,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":128,"record_offset":34572,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10743,"source_line":20,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":128,"record_offset":48968,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10761,"source_line":15,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::deferred_terrain_history::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/deferred_terrain_history.rs
//     evidence={"module":128,"record_offset":48004,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10755,"source_line":10,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::deferred_wall_history::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/deferred_wall_history.rs
//     evidence={"module":128,"record_offset":47976,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10753,"source_line":14,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::draw_wall::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/draw_wall.rs
//     evidence={"module":128,"record_offset":47948,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10751,"source_line":34,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::finish_wall::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/finish_wall.rs
//     evidence={"module":128,"record_offset":31444,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10737,"source_line":19,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::ui_continue_wall::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/ui_continue_wall.rs
//     evidence={"module":128,"record_offset":33020,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10739,"source_line":47,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":128,"record_offset":48032,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10757,"source_line":44,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":128,"record_offset":48500,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10759,"source_line":48,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":128,"record_offset":50048,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10765,"source_line":99,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":128,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":21968,"inlinee":10735,"source_line":304,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":128,"record_offset":22240,"parent_record_offset":22208,"ancestor_record_offset":21968,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":128,"record_offset":23652,"parent_record_offset":23628,"ancestor_record_offset":21968,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":128,"record_offset":23628,"parent_record_offset":23604,"ancestor_record_offset":21968,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":128,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":21968,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":128,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":21968,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":128,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":21968,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":128,"record_offset":23416,"parent_record_offset":23352,"ancestor_record_offset":21968,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":128,"record_offset":23504,"parent_record_offset":23480,"ancestor_record_offset":21968,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":128,"record_offset":22208,"parent_record_offset":22180,"ancestor_record_offset":21968,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":128,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":21968,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=29
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":128,"record_offset":23604,"parent_record_offset":23580,"ancestor_record_offset":21968,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":128,"record_offset":22960,"parent_record_offset":22936,"ancestor_record_offset":21968,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":128,"record_offset":33364,"parent_record_offset":33336,"ancestor_record_offset":21968,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":128,"record_offset":33416,"parent_record_offset":33392,"ancestor_record_offset":21968,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":128,"record_offset":33392,"parent_record_offset":33336,"ancestor_record_offset":21968,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":128,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":21968,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":128,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":21968,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":128,"record_offset":22428,"parent_record_offset":22404,"ancestor_record_offset":21968,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":128,"record_offset":22512,"parent_record_offset":22488,"ancestor_record_offset":21968,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":128,"record_offset":22452,"parent_record_offset":22428,"ancestor_record_offset":21968,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":128,"record_offset":33440,"parent_record_offset":33416,"ancestor_record_offset":21968,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":128,"record_offset":22624,"parent_record_offset":22600,"ancestor_record_offset":21968,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":128,"record_offset":23228,"parent_record_offset":22208,"ancestor_record_offset":21968,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":128,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":21968,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":128,"record_offset":23196,"parent_record_offset":22240,"ancestor_record_offset":21968,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":128,"record_offset":23480,"parent_record_offset":23300,"ancestor_record_offset":21968,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":34456,"parent_record_offset":33048,"ancestor_record_offset":21968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":128,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":21968,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":128,"record_offset":22768,"parent_record_offset":22744,"ancestor_record_offset":21968,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":128,"record_offset":33464,"parent_record_offset":33440,"ancestor_record_offset":21968,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":128,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":21968,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":128,"record_offset":23388,"parent_record_offset":23352,"ancestor_record_offset":21968,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":128,"record_offset":22984,"parent_record_offset":22960,"ancestor_record_offset":21968,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":128,"record_offset":49468,"parent_record_offset":48968,"ancestor_record_offset":21968,"inlinee":10763,"source_line":234,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":128,"record_offset":22912,"parent_record_offset":22888,"ancestor_record_offset":21968,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":128,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":21968,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":128,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":21968,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":128,"record_offset":22888,"parent_record_offset":22692,"ancestor_record_offset":21968,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":128,"record_offset":34528,"parent_record_offset":34504,"ancestor_record_offset":21968,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22800,"parent_record_offset":22716,"ancestor_record_offset":21968,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22744,"parent_record_offset":22716,"ancestor_record_offset":21968,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":23144,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":21968,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22340,"parent_record_offset":22316,"ancestor_record_offset":21968,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22316,"parent_record_offset":22288,"ancestor_record_offset":21968,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":21968,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":128,"record_offset":22572,"parent_record_offset":22544,"ancestor_record_offset":21968,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":128,"record_offset":22600,"parent_record_offset":22572,"ancestor_record_offset":21968,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":128,"record_offset":22660,"parent_record_offset":22544,"ancestor_record_offset":21968,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":128,"record_offset":22544,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":128,"record_offset":22692,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":128,"record_offset":22404,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=58
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":128,"record_offset":22488,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":128,"record_offset":23060,"parent_record_offset":22364,"ancestor_record_offset":21968,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":128,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":21968,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":128,"record_offset":33524,"parent_record_offset":33092,"ancestor_record_offset":21968,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":128,"record_offset":33092,"parent_record_offset":33048,"ancestor_record_offset":21968,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":128,"record_offset":33140,"parent_record_offset":33116,"ancestor_record_offset":21968,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":128,"record_offset":33216,"parent_record_offset":33188,"ancestor_record_offset":21968,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":128,"record_offset":33336,"parent_record_offset":33188,"ancestor_record_offset":21968,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":128,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":21968,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":128,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":21968,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":128,"record_offset":33268,"parent_record_offset":33240,"ancestor_record_offset":21968,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":128,"record_offset":33188,"parent_record_offset":33164,"ancestor_record_offset":21968,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":128,"record_offset":33164,"parent_record_offset":33140,"ancestor_record_offset":21968,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00128.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::walls_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":248,"record_offset":5912,"rva":3921968,"va":5372631088}
// 编译实例 RVAs: 0x3bd830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd290
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1053653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e82becefff","size":5,"soft_fail":false,"target_va":5371581072,"va":5372634720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1037819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8052af0ff","size":5,"soft_fail":false,"target_va":5371596704,"va":5372634518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-934536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e878bdf1ff","size":5,"soft_fail":false,"target_va":5371700080,"va":5372634611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140339720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-542137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e847baf7ff","size":5,"soft_fail":false,"target_va":5372090144,"va":5372632276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-233626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8666ffcff","size":5,"soft_fail":false,"target_va":5372398320,"va":5372631941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-204370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8aee1fcff","size":5,"soft_fail":false,"target_va":5372427072,"va":5372631437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140395ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-164033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e83f7ffdff","size":5,"soft_fail":false,"target_va":5372468128,"va":5372632156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=7
//     target_aliases=16 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8e7e7ffff","size":5,"soft_fail":false,"target_va":5372625024,"va":5372631188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e81bedffff","size":5,"soft_fail":false,"target_va":5372630832,"va":5372635664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t117262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e80eca0100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372632429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t118072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e838cd0100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372631635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2740184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8d8cf2900","size":5,"soft_fail":false,"target_va":5375372576,"va":5372632387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140669fd0
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2802304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e880c22a00","size":5,"soft_fail":false,"target_va":5375434704,"va":5372632395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140669fe0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2802484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e834c32a00","size":5,"soft_fail":false,"target_va":5375434720,"va":5372632231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3653590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8d6bf3700","size":5,"soft_fail":false,"target_va":5376286288,"va":5372632693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30269002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e84adecd01","size":5,"soft_fail":false,"target_va":5402900112,"va":5372631105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30272505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8f9ebcd01","size":5,"soft_fail":false,"target_va":5402906304,"va":5372633794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30960993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8616dd801","size":5,"soft_fail":false,"target_va":5403593344,"va":5372632346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30961097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8c96dd801","size":5,"soft_fail":false,"target_va":5403593360,"va":5372632258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30344330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e88a04cf01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372633809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t30345054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e85e07cf01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372633821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30346056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8480bcf01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372634067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove_all
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2654079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e87f7f2800","size":5,"soft_fail":false,"target_va":5375286176,"va":5372632092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::reset_max_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2653811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8737e2800","size":5,"soft_fail":false,"target_va":5375285920,"va":5372632104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wooden_gates::Gates::clear
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1958241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e861e11d00","size":5,"soft_fail":false,"target_va":5374590368,"va":5372632122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2181624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e8f8492100","size":5,"soft_fail":false,"target_va":5374813760,"va":5372632131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t4735053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372631088,"raw_hex":"e84d404800","size":5,"soft_fail":false,"target_va":5377366224,"va":5372631166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372631088,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5372632366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":248,"record_offset":11480,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":248,"record_offset":7604,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":248,"record_offset":11408,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":248,"record_offset":8188,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":248,"record_offset":14192,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":248,"record_offset":11180,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":248,"record_offset":11892,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":248,"record_offset":6084,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":10741,"source_line":7,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":248,"record_offset":8668,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":10745,"source_line":15,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":248,"record_offset":7664,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":10743,"source_line":20,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":248,"record_offset":9732,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":10765,"source_line":99,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":248,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":5912,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":248,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":5912,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":248,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":5912,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":248,"record_offset":11608,"parent_record_offset":11580,"ancestor_record_offset":5912,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":248,"record_offset":8580,"parent_record_offset":8364,"ancestor_record_offset":5912,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":248,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":5912,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":248,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":5912,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":248,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":5912,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":248,"record_offset":11736,"parent_record_offset":11504,"ancestor_record_offset":5912,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":248,"record_offset":11532,"parent_record_offset":11504,"ancestor_record_offset":5912,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":248,"record_offset":11504,"parent_record_offset":11480,"ancestor_record_offset":5912,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":248,"record_offset":7632,"parent_record_offset":7604,"ancestor_record_offset":5912,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":248,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5912,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":248,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":5912,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":248,"record_offset":6412,"parent_record_offset":6360,"ancestor_record_offset":5912,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":248,"record_offset":11236,"parent_record_offset":11204,"ancestor_record_offset":5912,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":248,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":5912,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":248,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":5912,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":248,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":5912,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":248,"record_offset":8456,"parent_record_offset":8428,"ancestor_record_offset":5912,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":248,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":5912,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":248,"record_offset":7340,"parent_record_offset":7284,"ancestor_record_offset":5912,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":248,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5912,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":248,"record_offset":7488,"parent_record_offset":6084,"ancestor_record_offset":5912,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":248,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":5912,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":248,"record_offset":8520,"parent_record_offset":8396,"ancestor_record_offset":5912,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":248,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":5912,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":248,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":5912,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":248,"record_offset":11956,"parent_record_offset":11920,"ancestor_record_offset":5912,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":248,"record_offset":11320,"parent_record_offset":11292,"ancestor_record_offset":5912,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":248,"record_offset":11348,"parent_record_offset":11320,"ancestor_record_offset":5912,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":248,"record_offset":11264,"parent_record_offset":11236,"ancestor_record_offset":5912,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":248,"record_offset":11204,"parent_record_offset":11180,"ancestor_record_offset":5912,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":248,"record_offset":11920,"parent_record_offset":11892,"ancestor_record_offset":5912,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":248,"record_offset":10232,"parent_record_offset":9732,"ancestor_record_offset":5912,"inlinee":10763,"source_line":234,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":8396,"parent_record_offset":8364,"ancestor_record_offset":5912,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":5912,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":8364,"parent_record_offset":8308,"ancestor_record_offset":5912,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":5912,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":8428,"parent_record_offset":8396,"ancestor_record_offset":5912,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":248,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":5912,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":248,"record_offset":6544,"parent_record_offset":6124,"ancestor_record_offset":5912,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":248,"record_offset":6124,"parent_record_offset":6084,"ancestor_record_offset":5912,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":248,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5912,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":248,"record_offset":6248,"parent_record_offset":6220,"ancestor_record_offset":5912,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":248,"record_offset":6360,"parent_record_offset":6220,"ancestor_record_offset":5912,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":248,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5912,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":248,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5912,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":248,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":5912,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":248,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":5912,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00248.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":248,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":5912,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00248.jsonl.gz"}
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
// 完整结构及原始类型记录: app_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: app_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: app_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 127; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >,bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>", "bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >", "bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>", "bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>", "bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_events
// 信息 ID: 0738f18fec225c9e9b3c53fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf6b6 -> 0x251a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems
// 信息 ID: da0fd6744b8aab6dcf2f940b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3cf6de -> 0x251290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 信息 ID: bf59357d96c89fec1152f861
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2244e3aa9ca98f1554fc8390: void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// [泛型宿主/闭包/直接引用方参数候选] cb840d8a55515fa596c3c1b1: void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::app_systems::impl$0::add_wall_systems", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::config::impl$10::into_configs<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)>,void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 28 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; module=62, inline_record=19396; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; module=66, inline_record=1530420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::walls_on_glade_unload
// 信息 ID: 74693b78d81d6e72b2139609
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ad1569bdd3f3f49074b4416c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)> @ 0x82882 -> 0x3bd830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)> > @ 0xd19b9 -> 0x3bd830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
