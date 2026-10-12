// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/photomode/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5ce4be8a9593bf22c29ac2fc1e7053e12baca103a636aa7b9c93cd0ed5700f0c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4206
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::do_photomode
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4206,"record_offset":264,"rva":43440896,"va":5412150016}
// 编译实例 RVAs: 0x296db00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34033165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8f3b1f8fd","size":5,"soft_fail":false,"target_va":5378120480,"va":5412153640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9c50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34032032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e860b6f8fd","size":5,"soft_fail":false,"target_va":5378120784,"va":5412152811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34027343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8b1c8f8fd","size":5,"soft_fail":false,"target_va":5378125088,"va":5412152426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a3e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33336583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8f95203fe","size":5,"soft_fail":false,"target_va":5378817680,"va":5412154258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1fff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32826299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8451c0bfe","size":5,"soft_fail":false,"target_va":5379325936,"va":5412152230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a202c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32825906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8ce1d0bfe","size":5,"soft_fail":false,"target_va":5379326656,"va":5412152557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a22a10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32814062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8124c0bfe","size":5,"soft_fail":false,"target_va":5379336720,"va":5412150777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31089116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8249e25fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412153287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25107553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e89fe380fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412153532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ac1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24913695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8e1d883fe","size":5,"soft_fail":false,"target_va":5387239872,"va":5412153562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429b3a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t284122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8da550400","size":5,"soft_fail":false,"target_va":5412436608,"va":5412152481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4198443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e82b104000","size":5,"soft_fail":false,"target_va":5416350384,"va":5412151936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4198595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8c3104000","size":5,"soft_fail":false,"target_va":5416350720,"va":5412152120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4199576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e898144000","size":5,"soft_fail":false,"target_va":5416351056,"va":5412151475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4197998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e86e0e4000","size":5,"soft_fail":false,"target_va":5416351392,"va":5412153389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bdd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4905469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8fdd94a00","size":5,"soft_fail":false,"target_va":5417057744,"va":5412152270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4467575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8772b4400","size":5,"soft_fail":false,"target_va":5416621600,"va":5412154020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4467629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8ad2b4400","size":5,"soft_fail":false,"target_va":5416621762,"va":5412154128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4468135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8a72d4400","size":5,"soft_fail":false,"target_va":5416622240,"va":5412154100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33745786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e88614fdfd","size":5,"soft_fail":false,"target_va":5378404688,"va":5412150469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32463885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8f3a310fe","size":5,"soft_fail":false,"target_va":5379687184,"va":5412151064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30545561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e867e92dfe","size":5,"soft_fail":false,"target_va":5381607616,"va":5412153172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30542847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e801f42dfe","size":5,"soft_fail":false,"target_va":5381607568,"va":5412150410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::switch_kind
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33287371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8351304fe","size":5,"soft_fail":false,"target_va":5378863456,"va":5412150822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30396606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8422f30fe","size":5,"soft_fail":false,"target_va":5381756080,"va":5412152681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::aspect_ratio
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30396733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8c32e30fe","size":5,"soft_fail":false,"target_va":5381755968,"va":5412152696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34032533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e86bb4f8fd","size":5,"soft_fail":false,"target_va":5378118240,"va":5412150768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::init_walk_camera
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32833614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8b2ff0afe","size":5,"soft_fail":false,"target_va":5379318016,"va":5412151625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::match_free_camera_to_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32831273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8d7080bfe","size":5,"soft_fail":false,"target_va":5379319584,"va":5412150852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32826213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e89b1c0bfe","size":5,"soft_fail":false,"target_va":5379325968,"va":5412152176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::deactivate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33739108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e89c2efdfd","size":5,"soft_fail":false,"target_va":5378411856,"va":5412150959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32127030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8cac715fe","size":5,"soft_fail":false,"target_va":5380025344,"va":5412152369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32127559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8b9c515fe","size":5,"soft_fail":false,"target_va":5380024784,"va":5412152338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32127782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8dac415fe","size":5,"soft_fail":false,"target_va":5380024576,"va":5412152353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32127506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e8eec515fe","size":5,"soft_fail":false,"target_va":5380024896,"va":5412152397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35946948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412150016,"raw_hex":"e83c7edbfd","size":5,"soft_fail":false,"target_va":5376206368,"va":5412153311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5412150016,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5412152502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4206,"record_offset":456,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4206,"record_offset":1296,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4206,"record_offset":3204,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4206,"record_offset":516,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4206,"record_offset":2736,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4206,"record_offset":6016,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4206,"record_offset":1600,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4206,"record_offset":6476,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":6772,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":6680,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4206,"record_offset":6620,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::project_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4206,"record_offset":7396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":32183,"source_line":1258,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":4206,"record_offset":6888,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4206,"record_offset":8044,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4206,"record_offset":6592,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":17937,"source_line":436,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4206,"record_offset":8508,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":22173,"source_line":250,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4206,"record_offset":8480,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8160,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":21728,"source_line":488,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8860,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8628,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4206,"record_offset":2956,"parent_record_offset":2924,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4206,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4206,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":2924,"parent_record_offset":2884,"ancestor_record_offset":264,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":4216,"parent_record_offset":4188,"ancestor_record_offset":264,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":264,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":2852,"parent_record_offset":2808,"ancestor_record_offset":264,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":2884,"parent_record_offset":2852,"ancestor_record_offset":264,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4206,"record_offset":864,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4206,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4206,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":264,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4206,"record_offset":624,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4206,"record_offset":1336,"parent_record_offset":1296,"ancestor_record_offset":264,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4206,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":264,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4206,"record_offset":552,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":9341,"source_line":79,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4206,"record_offset":588,"parent_record_offset":552,"ancestor_record_offset":264,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4206,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":264,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4206,"record_offset":6100,"parent_record_offset":6016,"ancestor_record_offset":264,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4206,"record_offset":2808,"parent_record_offset":2764,"ancestor_record_offset":264,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":4206,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":264,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4206,"record_offset":7156,"parent_record_offset":7040,"ancestor_record_offset":264,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4206,"record_offset":7980,"parent_record_offset":7900,"ancestor_record_offset":264,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4206,"record_offset":7008,"parent_record_offset":6928,"ancestor_record_offset":264,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4206,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":264,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":4206,"record_offset":6524,"parent_record_offset":6476,"ancestor_record_offset":264,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4206,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":264,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4206,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":264,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4206,"record_offset":1376,"parent_record_offset":1336,"ancestor_record_offset":264,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4206,"record_offset":3256,"parent_record_offset":3228,"ancestor_record_offset":264,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4206,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":264,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4206,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4206,"record_offset":692,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4206,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":264,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":1408,"parent_record_offset":1376,"ancestor_record_offset":264,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":264,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":264,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":1464,"parent_record_offset":1376,"ancestor_record_offset":264,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":3336,"parent_record_offset":3256,"ancestor_record_offset":264,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":8592,"parent_record_offset":8532,"ancestor_record_offset":264,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4206,"record_offset":1496,"parent_record_offset":1336,"ancestor_record_offset":264,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4206,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4206,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":264,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4206,"record_offset":3044,"parent_record_offset":2884,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4206,"record_offset":720,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4206,"record_offset":920,"parent_record_offset":624,"ancestor_record_offset":264,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4206,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4206,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":264,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4206,"record_offset":948,"parent_record_offset":920,"ancestor_record_offset":264,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4206,"record_offset":3412,"parent_record_offset":3360,"ancestor_record_offset":264,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4206,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":264,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4206,"record_offset":6928,"parent_record_offset":6888,"ancestor_record_offset":264,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4206,"record_offset":7336,"parent_record_offset":6888,"ancestor_record_offset":264,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$9::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4206,"record_offset":7900,"parent_record_offset":7396,"ancestor_record_offset":264,"inlinee":32185,"source_line":1202,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":4206,"record_offset":8012,"parent_record_offset":7396,"ancestor_record_offset":264,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4206,"record_offset":8564,"parent_record_offset":8532,"ancestor_record_offset":264,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":264,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":264,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4206,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":264,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/04206.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4206,"record_offset":8224,"parent_record_offset":8184,"ancestor_record_offset":264,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04206.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>

// ------------------------------------------------------------------
// 函数: toolbar::photomode::do_photomode
// 信息 ID: bd0fa13de0f65a008cfdb6ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5710f6aef908b638eaee4f51: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>)> > @ 0x230619c -> 0x296db00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>)> @ 0x23bd213 -> 0x296db00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
