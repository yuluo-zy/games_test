// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/photomode/photomode_buttons.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5d0fbd27e7a10eefd874582ce258fca51861b291071298488407823110e88a98
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: photomode_buttons.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4208
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::photomode_buttons::photomode_buttons
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4208,"record_offset":264,"rva":43450768,"va":5412159888}
// 编译实例 RVAs: 0x2970190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27506710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8ea475cfe","size":5,"soft_fail":false,"target_va":5384656240,"va":5412162945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27506583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e869485cfe","size":5,"soft_fail":false,"target_va":5384656400,"va":5412162978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-36484505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8674ad3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412163204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42824221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8e38d72fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5412163064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38681563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e825c4b1fd","size":5,"soft_fail":false,"target_va":5373479040,"va":5412160598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38675871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e861dab1fd","size":5,"soft_fail":false,"target_va":5373484384,"va":5412160250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38675497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8d7dbb1fd","size":5,"soft_fail":false,"target_va":5373484784,"va":5412160276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140545280
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37927647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e82145bdfd","size":5,"soft_fail":false,"target_va":5374235264,"va":5412162906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35923681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e81fd9dbfd","size":5,"soft_fail":false,"target_va":5376236864,"va":5412160540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140731780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35911263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8a109dcfd","size":5,"soft_fail":false,"target_va":5376251776,"va":5412163034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409af150
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33300375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e869e003fe","size":5,"soft_fail":false,"target_va":5378863440,"va":5412163810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a202c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32835202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e87ef90afe","size":5,"soft_fail":false,"target_va":5379326656,"va":5412161853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4d240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31602764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8b4c71dfe","size":5,"soft_fail":false,"target_va":5380559424,"va":5412162183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31095942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e87a8325fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412160113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30554378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8f6c62dfe","size":5,"soft_fail":false,"target_va":5381607664,"va":5412162037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29071757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8736644fe","size":5,"soft_fail":false,"target_va":5383091344,"va":5412163096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25114492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e884c880fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412160471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25040312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e848ea81fe","size":5,"soft_fail":false,"target_va":5387120192,"va":5412160499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4188141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8ede73f00","size":5,"soft_fail":false,"target_va":5416350384,"va":5412162238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4188940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e80ceb3f00","size":5,"soft_fail":false,"target_va":5416351056,"va":5412162111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4191217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8f1f33f00","size":5,"soft_fail":false,"target_va":5416351392,"va":5412160170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4189689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8f9ed3f00","size":5,"soft_fail":false,"target_va":5416353040,"va":5412163346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4456793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e859014400","size":5,"soft_fail":false,"target_va":5416621536,"va":5412164738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4457441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8e1034400","size":5,"soft_fail":false,"target_va":5416622208,"va":5412164762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::iter_non_default_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33298462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8e2e703fe","size":5,"soft_fail":false,"target_va":5378863472,"va":5412161929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30452799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8c1532ffe","size":5,"soft_fail":false,"target_va":5381709840,"va":5412162634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next_demo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30451973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8fb562ffe","size":5,"soft_fail":false,"target_va":5381709856,"va":5412161824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::activate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33750696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e85801fdfd","size":5,"soft_fail":false,"target_va":5378412080,"va":5412162771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::deactivate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33750759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e81901fdfd","size":5,"soft_fail":false,"target_va":5378411856,"va":5412162610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::is_active
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33750363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8a502fdfd","size":5,"soft_fail":false,"target_va":5378412224,"va":5412162582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29025801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8f71945fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5412160516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29023348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e88c2345fe","size":5,"soft_fail":false,"target_va":5383139568,"va":5412162911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::create_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1853250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e842471c00","size":5,"soft_fail":false,"target_va":5414016384,"va":5412163129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35953774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e89263dbfd","size":5,"soft_fail":false,"target_va":5376206368,"va":5412160137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35951929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e8c76adbfd","size":5,"soft_fail":false,"target_va":5376208368,"va":5412160292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35979667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412159888,"raw_hex":"e86dfedafd","size":5,"soft_fail":false,"target_va":5376180896,"va":5412160558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4208,"record_offset":476,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4208,"record_offset":12228,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4208,"record_offset":3040,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4208,"record_offset":2340,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4208,"record_offset":5996,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4208,"record_offset":9104,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4208,"record_offset":7872,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4208,"record_offset":3324,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11799,"source_line":1524,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4208,"record_offset":8760,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":4208,"record_offset":5652,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4208,"record_offset":8196,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::photo_mode::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/photo_mode.rs
//     evidence={"module":4208,"record_offset":12196,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":136685,"source_line":19,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":4208,"record_offset":8168,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":4208,"record_offset":8224,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4208,"record_offset":1076,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":536,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":2200,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":852,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4208,"record_offset":1104,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4208,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4208,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4208,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4208,"record_offset":8588,"parent_record_offset":8532,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8016,"parent_record_offset":7988,"ancestor_record_offset":264,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":264,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8560,"parent_record_offset":8532,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8076,"parent_record_offset":8044,"ancestor_record_offset":264,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":264,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4208,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":688,"parent_record_offset":652,"ancestor_record_offset":264,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":264,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":264,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":264,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":264,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":628,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":264,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":7988,"parent_record_offset":7956,"ancestor_record_offset":264,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4208,"record_offset":2660,"parent_record_offset":2428,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":4208,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":264,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":4900,"parent_record_offset":4872,"ancestor_record_offset":264,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":11740,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":10620,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":10704,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":11992,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":3728,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4208,"record_offset":3916,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4208,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4208,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4208,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":264,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4208,"record_offset":3404,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4208,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":264,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4208,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":264,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4208,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":264,"inlinee":8623,"source_line":959,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":4208,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":264,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4208,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":264,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4208,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":264,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4208,"record_offset":2404,"parent_record_offset":2364,"ancestor_record_offset":264,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4208,"record_offset":5396,"parent_record_offset":5324,"ancestor_record_offset":264,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4208,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":264,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":264,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":264,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":264,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":4288,"parent_record_offset":4260,"ancestor_record_offset":264,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":264,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":264,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":11380,"parent_record_offset":10796,"ancestor_record_offset":264,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":10008,"parent_record_offset":9456,"ancestor_record_offset":264,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":4812,"parent_record_offset":4236,"ancestor_record_offset":264,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":4872,"parent_record_offset":4840,"ancestor_record_offset":264,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4208,"record_offset":5148,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4208,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":264,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4208,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":264,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4208,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":264,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4208,"record_offset":4236,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4208,"record_offset":9456,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4208,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":264,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4208,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":264,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4208,"record_offset":4404,"parent_record_offset":4376,"ancestor_record_offset":264,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4208,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":264,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4208,"record_offset":4376,"parent_record_offset":4348,"ancestor_record_offset":264,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4208,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":264,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":4348,"parent_record_offset":4316,"ancestor_record_offset":264,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":264,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":9372,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":264,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":4004,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4208,"record_offset":10408,"parent_record_offset":10008,"ancestor_record_offset":264,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4208,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":264,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4208,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":264,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4208,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":264,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4208,"record_offset":7928,"parent_record_offset":7900,"ancestor_record_offset":264,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4208,"record_offset":7956,"parent_record_offset":7928,"ancestor_record_offset":264,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4208,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":264,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4208,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":264,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4208,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":264,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4208,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":264,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4208,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":264,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4208,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4208,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":264,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4208,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":264,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4208,"record_offset":2016,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4208,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":264,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4208,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":264,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4208,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":264,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4208,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":264,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4208,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":264,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4208,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":264,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4208,"record_offset":5780,"parent_record_offset":5752,"ancestor_record_offset":264,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4208,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":264,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4208,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":264,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4208,"record_offset":2476,"parent_record_offset":2428,"ancestor_record_offset":264,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4208,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":264,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4208,"record_offset":1720,"parent_record_offset":1692,"ancestor_record_offset":264,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4208,"record_offset":4496,"parent_record_offset":4432,"ancestor_record_offset":264,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4208,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":264,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":264,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":264,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":264,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":3516,"parent_record_offset":3428,"ancestor_record_offset":264,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":264,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":3608,"parent_record_offset":3428,"ancestor_record_offset":264,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":3672,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4208,"record_offset":3172,"parent_record_offset":3092,"ancestor_record_offset":264,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4208,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4208,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4208,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4208,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":264,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4208,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":264,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4208,"record_offset":1168,"parent_record_offset":1136,"ancestor_record_offset":264,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4208,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":264,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4208,"record_offset":808,"parent_record_offset":652,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4208,"record_offset":3700,"parent_record_offset":3348,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4208,"record_offset":2532,"parent_record_offset":2428,"ancestor_record_offset":264,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4208,"record_offset":11936,"parent_record_offset":9136,"ancestor_record_offset":264,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4208,"record_offset":2884,"parent_record_offset":2428,"ancestor_record_offset":264,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4208,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":264,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4208,"record_offset":4640,"parent_record_offset":4608,"ancestor_record_offset":264,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4208,"record_offset":2588,"parent_record_offset":2560,"ancestor_record_offset":264,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4208,"record_offset":2560,"parent_record_offset":2532,"ancestor_record_offset":264,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4208,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":264,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4208,"record_offset":3248,"parent_record_offset":3196,"ancestor_record_offset":264,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4208,"record_offset":5868,"parent_record_offset":5724,"ancestor_record_offset":264,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$1::iter_non_default_mut::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":4208,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":264,"inlinee":70317,"source_line":110,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4208,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4208,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":264,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4208,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":264,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":4208,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":264,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4208,"record_offset":1660,"parent_record_offset":1632,"ancestor_record_offset":264,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4208,"record_offset":1692,"parent_record_offset":1660,"ancestor_record_offset":264,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4208,"record_offset":1632,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4208,"record_offset":1492,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4208,"record_offset":2044,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4208,"record_offset":1576,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4208,"record_offset":1436,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4208,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":264,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4208,"record_offset":1272,"parent_record_offset":1232,"ancestor_record_offset":264,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4208,"record_offset":1900,"parent_record_offset":1792,"ancestor_record_offset":264,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4208,"record_offset":1304,"parent_record_offset":1272,"ancestor_record_offset":264,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":264,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":2136,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":264,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":264,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1332,"parent_record_offset":1272,"ancestor_record_offset":264,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":264,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1464,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4208,"record_offset":1792,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4208,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":264,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4208,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":264,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4208,"record_offset":1232,"parent_record_offset":1196,"ancestor_record_offset":264,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4208,"record_offset":1196,"parent_record_offset":1136,"ancestor_record_offset":264,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":4208,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":264,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":4208,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":264,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":264,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":1136,"parent_record_offset":1104,"ancestor_record_offset":264,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4208,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":264,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4208,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":264,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":600,"parent_record_offset":560,"ancestor_record_offset":264,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":264,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":928,"parent_record_offset":876,"ancestor_record_offset":264,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/04208.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4208,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":264,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/04208.jsonl.gz"}
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
// 完整结构及原始类型记录: photomode_buttons.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: photomode_buttons.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: photomode_buttons.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::photomode::photomode_buttons::photomode_buttons
// 信息 ID: c6a02fdf720895df4e9c30a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ed61dcea0891ddbc69cf8595: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)> > @ 0x2314b67 -> 0x2970190 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)> @ 0x29b5afd -> 0x2970190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
