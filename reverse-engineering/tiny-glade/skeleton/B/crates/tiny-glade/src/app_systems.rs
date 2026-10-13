// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/app_systems.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bb9bf8e56998ff9623efb91b20852942cabe01122ab1011df1101fbbd06d4539
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: app_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 102, 161, 178, 186, 189
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":72644,"rva":4796704,"va":5373505824}
// 编译实例 RVAs: 0x493120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=6
//     target_aliases=53 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4627756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8d462b9ff","size":5,"soft_fail":false,"target_va":5368879232,"va":5373506983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cbd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3962622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e80289c3ff","size":5,"soft_fail":false,"target_va":5369544000,"va":5373506617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140112d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3671077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8dbfbc7ff","size":5,"soft_fail":false,"target_va":5369834800,"va":5373505872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140272c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2230473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e837f7ddff","size":5,"soft_fail":false,"target_va":5371276320,"va":5373506788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140322d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1508401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8cffbe8ff","size":5,"soft_fail":false,"target_va":5371997504,"va":5373505900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140326b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1492876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e87438e9ff","size":5,"soft_fail":false,"target_va":5372013376,"va":5373506247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403270d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1491499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8d53de9ff","size":5,"soft_fail":false,"target_va":5372014800,"va":5373506294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403276f0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1489520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e89045e9ff","size":5,"soft_fail":false,"target_va":5372016368,"va":5373505883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140327c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1489241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8a746e9ff","size":5,"soft_fail":false,"target_va":5372017696,"va":5373506932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364d20
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1238337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8bf1aedff","size":5,"soft_fail":false,"target_va":5372267808,"va":5373506140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14050d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t499968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e800a10700","size":5,"soft_fail":false,"target_va":5374005904,"va":5373505931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054e960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t767964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8dcb70b00","size":5,"soft_fail":false,"target_va":5374273888,"va":5373505919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3633936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e8f08cc8ff","size":5,"soft_fail":false,"target_va":5369872352,"va":5373506283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1431148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e89429eaff","size":5,"soft_fail":false,"target_va":5372074800,"va":5373505943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1585034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e876d0e7ff","size":5,"soft_fail":false,"target_va":5371921568,"va":5373506597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3918259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373505824,"raw_hex":"e84d36c4ff","size":5,"soft_fail":false,"target_va":5369587904,"va":5373506158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140327ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1489579","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373505824,"raw_hex":"e95545e9ff","size":5,"soft_fail":false,"target_va":5372017376,"va":5373506950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":186,"record_offset":72788,"parent_record_offset":72644,"ancestor_record_offset":72644,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":186,"record_offset":72844,"parent_record_offset":72644,"ancestor_record_offset":72644,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs
//     evidence={"module":186,"record_offset":72944,"parent_record_offset":72644,"ancestor_record_offset":72644,"inlinee":14252,"source_line":73,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":186,"record_offset":72884,"parent_record_offset":72844,"ancestor_record_offset":72644,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35B2
// 首条位置: {"module":161,"inlinee":13746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":161,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":6340,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00161.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_state","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.16.0/src/state/resources.rs
//     evidence={"module":161,"record_offset":6424,"parent_record_offset":6340,"ancestor_record_offset":6340,"inlinee":12393,"source_line":134,"archive":"_pdb/B/modules/00161.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":161,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":6340,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00161.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":73516,"rva":4798128,"va":5373507248}
// 编译实例 RVAs: 0x4936b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ec10
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4481545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8f79dbbff","size":5,"soft_fail":false,"target_va":5369031696,"va":5373513236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4476137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e817b3bbff","size":5,"soft_fail":false,"target_va":5369036880,"va":5373513012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400520b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4467961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e807d3bbff","size":5,"soft_fail":false,"target_va":5369045168,"va":5373513124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400523e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4466825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e877d7bbff","size":5,"soft_fail":false,"target_va":5369045984,"va":5373512804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3909379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8fd58c4ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5373510174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=39
//     target_aliases=19 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3891231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8e19fc4ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373507338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3453014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8aa4fcbff","size":5,"soft_fail":false,"target_va":5370059888,"va":5373512897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a82b20
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6220401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e871ea5e00","size":5,"soft_fail":false,"target_va":5379730208,"va":5373509802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6596197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e865a66400","size":5,"soft_fail":false,"target_va":5380105744,"va":5373509542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3609457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e88fecc8ff","size":5,"soft_fail":false,"target_va":5369898528,"va":5373507980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t888369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8318e0d00","size":5,"soft_fail":false,"target_va":5374395776,"va":5373507402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3957446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8c6623c00","size":5,"soft_fail":false,"target_va":5377466256,"va":5373508805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6363056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8b0176100","size":5,"soft_fail":false,"target_va":5379871056,"va":5373507995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t9149697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8019d8b00","size":5,"soft_fail":false,"target_va":5382657856,"va":5373508154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t9245603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8a3138d00","size":5,"soft_fail":false,"target_va":5382753792,"va":5373508184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38024395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373507248,"raw_hex":"e8cb344402","size":5,"soft_fail":false,"target_va":5411537104,"va":5373512704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":186,"record_offset":73708,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::editing_highlight::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/editing_highlight.rs
//     evidence={"module":186,"record_offset":83984,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14262,"source_line":7,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":186,"record_offset":83956,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14260,"source_line":98,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::plaster::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/plaster.rs
//     evidence={"module":186,"record_offset":83224,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14258,"source_line":67,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":186,"record_offset":79444,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":11634,"source_line":10,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":186,"record_offset":82492,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":11638,"source_line":19,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":186,"record_offset":81760,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":11636,"source_line":25,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":186,"record_offset":91968,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":12904,"source_line":15,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::deferred_terrain_history::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/deferred_terrain_history.rs
//     evidence={"module":186,"record_offset":90476,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14268,"source_line":10,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::deferred_wall_history::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/deferred_wall_history.rs
//     evidence={"module":186,"record_offset":90448,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14266,"source_line":14,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::draw_wall::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/draw_wall.rs
//     evidence={"module":186,"record_offset":90420,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14264,"source_line":37,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::finish_wall::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/finish_wall.rs
//     evidence={"module":186,"record_offset":78676,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14254,"source_line":19,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::ui_continue_wall::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/ui_continue_wall.rs
//     evidence={"module":186,"record_offset":79416,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14256,"source_line":51,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":186,"record_offset":90504,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14270,"source_line":47,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":186,"record_offset":91236,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":14272,"source_line":51,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":186,"record_offset":93256,"parent_record_offset":73516,"ancestor_record_offset":73516,"inlinee":11640,"source_line":105,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":74160,"parent_record_offset":74136,"ancestor_record_offset":73516,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":74208,"parent_record_offset":74184,"ancestor_record_offset":73516,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":186,"record_offset":74184,"parent_record_offset":74160,"ancestor_record_offset":73516,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":74136,"parent_record_offset":74112,"ancestor_record_offset":73516,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":74112,"parent_record_offset":74080,"ancestor_record_offset":73516,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":74080,"parent_record_offset":74056,"ancestor_record_offset":73516,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":186,"record_offset":73732,"parent_record_offset":73708,"ancestor_record_offset":73516,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":186,"record_offset":74352,"parent_record_offset":74328,"ancestor_record_offset":73516,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":186,"record_offset":74328,"parent_record_offset":74304,"ancestor_record_offset":73516,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":186,"record_offset":73836,"parent_record_offset":73816,"ancestor_record_offset":73516,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":186,"record_offset":73884,"parent_record_offset":73860,"ancestor_record_offset":73516,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":186,"record_offset":73860,"parent_record_offset":73836,"ancestor_record_offset":73516,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":186,"record_offset":74056,"parent_record_offset":74032,"ancestor_record_offset":73516,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":186,"record_offset":73760,"parent_record_offset":73732,"ancestor_record_offset":73516,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=39
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":186,"record_offset":73816,"parent_record_offset":73792,"ancestor_record_offset":73516,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":186,"record_offset":80016,"parent_record_offset":79984,"ancestor_record_offset":73516,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":186,"record_offset":80076,"parent_record_offset":80044,"ancestor_record_offset":73516,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":186,"record_offset":93000,"parent_record_offset":92944,"ancestor_record_offset":73516,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":186,"record_offset":79872,"parent_record_offset":79848,"ancestor_record_offset":73516,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":186,"record_offset":80044,"parent_record_offset":79984,"ancestor_record_offset":73516,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":186,"record_offset":73972,"parent_record_offset":73912,"ancestor_record_offset":73516,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":186,"record_offset":73792,"parent_record_offset":73760,"ancestor_record_offset":73516,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":186,"record_offset":73992,"parent_record_offset":73972,"ancestor_record_offset":73516,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":186,"record_offset":74280,"parent_record_offset":73760,"ancestor_record_offset":73516,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":186,"record_offset":74032,"parent_record_offset":73860,"ancestor_record_offset":73516,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":186,"record_offset":79736,"parent_record_offset":79708,"ancestor_record_offset":73516,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":186,"record_offset":92916,"parent_record_offset":92888,"ancestor_record_offset":73516,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":81644,"parent_record_offset":79444,"ancestor_record_offset":73516,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":186,"record_offset":79824,"parent_record_offset":79708,"ancestor_record_offset":73516,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":79900,"parent_record_offset":79872,"ancestor_record_offset":73516,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":79848,"parent_record_offset":79824,"ancestor_record_offset":73516,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":79928,"parent_record_offset":79872,"ancestor_record_offset":73516,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":186,"record_offset":73912,"parent_record_offset":73884,"ancestor_record_offset":73516,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":186,"record_offset":73944,"parent_record_offset":73912,"ancestor_record_offset":73516,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":186,"record_offset":92744,"parent_record_offset":91968,"ancestor_record_offset":73516,"inlinee":10944,"source_line":285,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":186,"record_offset":81716,"parent_record_offset":81692,"ancestor_record_offset":73516,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":186,"record_offset":80184,"parent_record_offset":79488,"ancestor_record_offset":73516,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":186,"record_offset":79488,"parent_record_offset":79444,"ancestor_record_offset":73516,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":186,"record_offset":79536,"parent_record_offset":79512,"ancestor_record_offset":73516,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":186,"record_offset":79784,"parent_record_offset":79760,"ancestor_record_offset":73516,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":186,"record_offset":79648,"parent_record_offset":79620,"ancestor_record_offset":73516,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":186,"record_offset":79984,"parent_record_offset":79584,"ancestor_record_offset":73516,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":186,"record_offset":79512,"parent_record_offset":79488,"ancestor_record_offset":73516,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":186,"record_offset":79676,"parent_record_offset":79648,"ancestor_record_offset":73516,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":186,"record_offset":79708,"parent_record_offset":79676,"ancestor_record_offset":73516,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":186,"record_offset":79584,"parent_record_offset":79560,"ancestor_record_offset":73516,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":186,"record_offset":79560,"parent_record_offset":79536,"ancestor_record_offset":73516,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::walls_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":26088,"rva":2825392,"va":5371534512}
// 编译实例 RVAs: 0x2b1cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ec10
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2506946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e83ebfd9ff","size":5,"soft_fail":false,"target_va":5369031696,"va":5371538637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140051fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2490455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8a9ffd9ff","size":5,"soft_fail":false,"target_va":5369044896,"va":5371535346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400520b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2493375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e841f4d9ff","size":5,"soft_fail":false,"target_va":5369045168,"va":5371538538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400523e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2492383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e821f8d9ff","size":5,"soft_fail":false,"target_va":5369045984,"va":5371538362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1934800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8307ae2ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5371535595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1478559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e86170e9ff","size":5,"soft_fail":false,"target_va":5370059888,"va":5371538442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ca870
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3245431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e877853100","size":5,"soft_fail":false,"target_va":5374781552,"va":5371536116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405faf50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3444198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8e68d3400","size":5,"soft_fail":false,"target_va":5374979920,"va":5371535717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8a7033500","size":5,"soft_fail":false,"target_va":5375009280,"va":5371534932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060cfb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3517783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e857ad3500","size":5,"soft_fail":false,"target_va":5375053744,"va":5371535956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3545220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e884183600","size":5,"soft_fail":false,"target_va":5375081488,"va":5371536263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3546299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8bb1c3600","size":5,"soft_fail":false,"target_va":5375081504,"va":5371535200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7166270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e83e596d00","size":5,"soft_fail":false,"target_va":5378702496,"va":5371536221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ca50
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7251686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8e6a66e00","size":5,"soft_fail":false,"target_va":5378787920,"va":5371536229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaae80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8358985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8498c7f00","size":5,"soft_fail":false,"target_va":5379894912,"va":5371535922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b19890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8812002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8e2758600","size":5,"soft_fail":false,"target_va":5380348048,"va":5371536041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b19900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8811599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e84f748600","size":5,"soft_fail":false,"target_va":5380348160,"va":5371536556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b19970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8811216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8d0728600","size":5,"soft_fail":false,"target_va":5380348272,"va":5371537051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9685789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e81dcb9300","size":5,"soft_fail":false,"target_va":5381222304,"va":5371536510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40310967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8b7186702","size":5,"soft_fail":false,"target_va":5411847152,"va":5371536180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40003226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e89a666202","size":5,"soft_fail":false,"target_va":5411540963,"va":5371537732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t40004114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8126a6202","size":5,"soft_fail":false,"target_va":5411541776,"va":5371537657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39944825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e879826102","size":5,"soft_fail":false,"target_va":5411482433,"va":5371537603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e821e1ffff","size":5,"soft_fail":false,"target_va":5371531776,"va":5371539674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40004984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8786d6202","size":5,"soft_fail":false,"target_va":5411542880,"va":5371537891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5932311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e817855a00","size":5,"soft_fail":false,"target_va":5377468208,"va":5371535892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::reset_max_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5931819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e82b835a00","size":5,"soft_fail":false,"target_va":5377467728,"va":5371535904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8335120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8102f7f00","size":5,"soft_fail":false,"target_va":5379871056,"va":5371535931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t11123275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e84bbaa900","size":5,"soft_fail":false,"target_va":5382657856,"va":5371534576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t11219181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8ed30ab00","size":5,"soft_fail":false,"target_va":5382753792,"va":5371534606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39999406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371534512,"raw_hex":"e8ae576202","size":5,"soft_fail":false,"target_va":5411537104,"va":5371537693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371534512,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5371536200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":33860,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":102,"record_offset":28544,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":102,"record_offset":33788,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":29392,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":102,"record_offset":37280,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":102,"record_offset":33560,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":102,"record_offset":34300,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":102,"record_offset":26260,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":11634,"source_line":10,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":102,"record_offset":29808,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":11638,"source_line":19,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":102,"record_offset":28604,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":11636,"source_line":25,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":102,"record_offset":30800,"parent_record_offset":26088,"ancestor_record_offset":26088,"inlinee":11640,"source_line":105,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":34068,"parent_record_offset":34044,"ancestor_record_offset":26088,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":34092,"parent_record_offset":34068,"ancestor_record_offset":26088,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":29748,"parent_record_offset":29720,"ancestor_record_offset":26088,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":34044,"parent_record_offset":34016,"ancestor_record_offset":26088,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":29720,"parent_record_offset":29568,"ancestor_record_offset":26088,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":33956,"parent_record_offset":33928,"ancestor_record_offset":26088,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":33420,"parent_record_offset":33392,"ancestor_record_offset":26088,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":33392,"parent_record_offset":33368,"ancestor_record_offset":26088,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":34016,"parent_record_offset":33984,"ancestor_record_offset":26088,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":33984,"parent_record_offset":33956,"ancestor_record_offset":26088,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":33368,"parent_record_offset":33344,"ancestor_record_offset":26088,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":34144,"parent_record_offset":33892,"ancestor_record_offset":26088,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":33928,"parent_record_offset":33892,"ancestor_record_offset":26088,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":102,"record_offset":33892,"parent_record_offset":33860,"ancestor_record_offset":26088,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":102,"record_offset":28572,"parent_record_offset":28544,"ancestor_record_offset":26088,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":26800,"parent_record_offset":26772,"ancestor_record_offset":26088,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":26860,"parent_record_offset":26828,"ancestor_record_offset":26088,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":33616,"parent_record_offset":33584,"ancestor_record_offset":26088,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":34200,"parent_record_offset":34176,"ancestor_record_offset":26088,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":34176,"parent_record_offset":34144,"ancestor_record_offset":26088,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":26664,"parent_record_offset":26640,"ancestor_record_offset":26088,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":29656,"parent_record_offset":29632,"ancestor_record_offset":26088,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":33452,"parent_record_offset":33420,"ancestor_record_offset":26088,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":26828,"parent_record_offset":26772,"ancestor_record_offset":26088,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":102,"record_offset":26428,"parent_record_offset":26396,"ancestor_record_offset":26088,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":102,"record_offset":26532,"parent_record_offset":26508,"ancestor_record_offset":26088,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":32480,"parent_record_offset":32452,"ancestor_record_offset":26088,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":28428,"parent_record_offset":26260,"ancestor_record_offset":26088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":26616,"parent_record_offset":26508,"ancestor_record_offset":26088,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":29688,"parent_record_offset":29600,"ancestor_record_offset":26088,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":26688,"parent_record_offset":26664,"ancestor_record_offset":26088,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":26640,"parent_record_offset":26616,"ancestor_record_offset":26088,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":26716,"parent_record_offset":26664,"ancestor_record_offset":26088,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":102,"record_offset":34368,"parent_record_offset":34328,"ancestor_record_offset":26088,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":33700,"parent_record_offset":33672,"ancestor_record_offset":26088,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":33728,"parent_record_offset":33700,"ancestor_record_offset":26088,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":33644,"parent_record_offset":33616,"ancestor_record_offset":26088,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":102,"record_offset":33584,"parent_record_offset":33560,"ancestor_record_offset":26088,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":102,"record_offset":34328,"parent_record_offset":34300,"ancestor_record_offset":26088,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":102,"record_offset":32308,"parent_record_offset":30800,"ancestor_record_offset":26088,"inlinee":10944,"source_line":285,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":29600,"parent_record_offset":29568,"ancestor_record_offset":26088,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":29512,"parent_record_offset":29488,"ancestor_record_offset":26088,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":29568,"parent_record_offset":29512,"ancestor_record_offset":26088,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":29540,"parent_record_offset":29512,"ancestor_record_offset":26088,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":29632,"parent_record_offset":29600,"ancestor_record_offset":26088,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":102,"record_offset":28500,"parent_record_offset":28476,"ancestor_record_offset":26088,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":102,"record_offset":26968,"parent_record_offset":26300,"ancestor_record_offset":26088,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":102,"record_offset":26300,"parent_record_offset":26260,"ancestor_record_offset":26088,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":102,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":26088,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":102,"record_offset":26576,"parent_record_offset":26556,"ancestor_record_offset":26088,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":102,"record_offset":26452,"parent_record_offset":26428,"ancestor_record_offset":26088,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":102,"record_offset":26772,"parent_record_offset":26396,"ancestor_record_offset":26088,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":102,"record_offset":26324,"parent_record_offset":26300,"ancestor_record_offset":26088,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":102,"record_offset":26480,"parent_record_offset":26452,"ancestor_record_offset":26088,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":102,"record_offset":26508,"parent_record_offset":26480,"ancestor_record_offset":26088,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":26088,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":26372,"parent_record_offset":26348,"ancestor_record_offset":26088,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00102.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 207; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion> >", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd> >", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::HighlightRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::system::commands::Commands", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_events
// 信息 ID: e32e3b5b9c79da4ad14bcc1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f514 -> 0x4936b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems
// 信息 ID: 55f67855b9985e274901f288
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f55b -> 0x493120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 信息 ID: 8f072ee466e719e9e78e87e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2244e3aa9ca98f1554fc8390: void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// [泛型宿主/闭包/直接引用方参数候选] 7b25cffe78a0ca24b17d209c: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::app_systems::impl$0::add_wall_systems", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 32 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; module=161, inline_record=6340; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> > >; module=178, inline_record=3156692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::walls_on_glade_unload
// 信息 ID: f5c3ef2c41e5acbe963f5233
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ad1569bdd3f3f49074b4416c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)> > @ 0x3c0051 -> 0x2b1cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>)> > > @ 0x477755 -> 0x2b1cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
