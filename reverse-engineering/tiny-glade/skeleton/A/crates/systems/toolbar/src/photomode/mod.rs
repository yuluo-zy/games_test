// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/photomode/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9bf69bb60405e84e0a8003ab39b2b880e957d0ed30b40b917e78f13969079bee
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3646
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::do_photomode
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3646,"record_offset":244,"rva":28688784,"va":5397397904}
// 编译实例 RVAs: 0x1b5c190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27783679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8010e58fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5397401322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24247171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e87d048efe","size":5,"soft_fail":false,"target_va":5373154400,"va":5397401566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140516250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23355945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8d79d9bfe","size":5,"soft_fail":false,"target_va":5374042704,"va":5397398644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140517ac0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23351606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8caae9bfe","size":5,"soft_fail":false,"target_va":5374048960,"va":5397400561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140517ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23351167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e881b09bfe","size":5,"soft_fail":false,"target_va":5374048992,"va":5397400154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22830844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e804a1a3fe","size":5,"soft_fail":false,"target_va":5374570832,"va":5397401671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140685e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21851333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e83b93b2fe","size":5,"soft_fail":false,"target_va":5375549040,"va":5397400368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21427959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e80909b9fe","size":5,"soft_fail":false,"target_va":5375974272,"va":5397402226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a700b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17747792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8b030f1fe","size":5,"soft_fail":false,"target_va":5379653808,"va":5397401595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9b310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t255908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8a4e70300","size":5,"soft_fail":false,"target_va":5397656336,"va":5397400423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6192852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8d47e5e00","size":5,"soft_fail":false,"target_va":5403593344,"va":5397400487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6193829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8a5825e00","size":5,"soft_fail":false,"target_va":5403594048,"va":5397400214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-26094496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e860d471fe","size":5,"soft_fail":false,"target_va":5371304880,"va":5397399371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5576889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8b9185500","size":5,"soft_fail":false,"target_va":5402978880,"va":5397401986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6054933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e815645c00","size":5,"soft_fail":false,"target_va":5403456912,"va":5397401974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5577407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8bf1a5500","size":5,"soft_fail":false,"target_va":5402979344,"va":5397401932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5576836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e884185500","size":5,"soft_fail":false,"target_va":5402978944,"va":5397402103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22615298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8feeaa6fe","size":5,"soft_fail":false,"target_va":5374783040,"va":5397398333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22716964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8dc5da5fe","size":5,"soft_fail":false,"target_va":5374681984,"va":5397398943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23002018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e85e04a1fe","size":5,"soft_fail":false,"target_va":5374399200,"va":5397401213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22999143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8990fa1fe","size":5,"soft_fail":false,"target_va":5374399152,"va":5397398290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::switch_kind
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22099174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e81acbaefe","size":5,"soft_fail":false,"target_va":5375299520,"va":5397398689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21988468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e88c7bb0fe","size":5,"soft_fail":false,"target_va":5375412208,"va":5397400671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::aspect_ratio
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21988467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e88d7bb0fe","size":5,"soft_fail":false,"target_va":5375412224,"va":5397400686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21850719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8a195b2fe","size":5,"soft_fail":false,"target_va":5375547920,"va":5397398634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::init_walk_camera
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23356816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8709a9bfe","size":5,"soft_fail":false,"target_va":5374042720,"va":5397399531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::match_free_camera_to_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23355237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e89ba09bfe","size":5,"soft_fail":false,"target_va":5374043488,"va":5397398720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-23351170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e87eb09bfe","size":5,"soft_fail":false,"target_va":5374048944,"va":5397400109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::deactivate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22150323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e84d03aefe","size":5,"soft_fail":false,"target_va":5375248512,"va":5397398830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21850461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8a396b2fe","size":5,"soft_fail":false,"target_va":5375550352,"va":5397400808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21628772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e89cf8b5fe","size":5,"soft_fail":false,"target_va":5375771552,"va":5397400319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21629125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e83bf7b5fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5397400288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21628964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8dcf7b5fe","size":5,"soft_fail":false,"target_va":5375771344,"va":5397400303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21629068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e874f7b5fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5397400343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24241959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397397904,"raw_hex":"e8d9188efe","size":5,"soft_fail":false,"target_va":5373159392,"va":5397401346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397397904,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5397400507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=19
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3646,"record_offset":428,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":488,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":2368,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":4732,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":1092,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":2728,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3646,"record_offset":1396,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3646,"record_offset":5052,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":5560,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":5468,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3646,"record_offset":5228,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::project_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3646,"record_offset":6096,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":21864,"source_line":1075,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3646,"record_offset":5672,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3646,"record_offset":5140,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6462,"source_line":491,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":3646,"record_offset":6884,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":51025,"source_line":243,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":6656,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":50927,"source_line":360,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":7256,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":7028,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3646,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3646,"record_offset":2484,"parent_record_offset":2440,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3646,"record_offset":2440,"parent_record_offset":2396,"ancestor_record_offset":244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3646,"record_offset":456,"parent_record_offset":428,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3646,"record_offset":2584,"parent_record_offset":2368,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":596,"parent_record_offset":560,"ancestor_record_offset":244,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":244,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":524,"parent_record_offset":488,"ancestor_record_offset":244,"inlinee":6368,"source_line":717,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":560,"parent_record_offset":524,"ancestor_record_offset":244,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":1132,"parent_record_offset":1092,"ancestor_record_offset":244,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3646,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":244,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3646,"record_offset":5296,"parent_record_offset":5256,"ancestor_record_offset":244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3646,"record_offset":5884,"parent_record_offset":5796,"ancestor_record_offset":244,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3646,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":244,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3646,"record_offset":5764,"parent_record_offset":5712,"ancestor_record_offset":244,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3646,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":244,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3646,"record_offset":5108,"parent_record_offset":5052,"ancestor_record_offset":244,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3646,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":244,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3646,"record_offset":1172,"parent_record_offset":1132,"ancestor_record_offset":244,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3646,"record_offset":2780,"parent_record_offset":2752,"ancestor_record_offset":244,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3646,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":244,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3646,"record_offset":664,"parent_record_offset":596,"ancestor_record_offset":244,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":244,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":244,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":244,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":1260,"parent_record_offset":1172,"ancestor_record_offset":244,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":2860,"parent_record_offset":2780,"ancestor_record_offset":244,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":6992,"parent_record_offset":6908,"ancestor_record_offset":244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":3572,"parent_record_offset":3436,"ancestor_record_offset":244,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3646,"record_offset":1292,"parent_record_offset":1132,"ancestor_record_offset":244,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3646,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3646,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":244,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3646,"record_offset":6936,"parent_record_offset":6908,"ancestor_record_offset":244,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3646,"record_offset":2548,"parent_record_offset":2440,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3646,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":244,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3646,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":244,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3646,"record_offset":728,"parent_record_offset":596,"ancestor_record_offset":244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3646,"record_offset":932,"parent_record_offset":596,"ancestor_record_offset":244,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3646,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":244,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3646,"record_offset":768,"parent_record_offset":728,"ancestor_record_offset":244,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3646,"record_offset":960,"parent_record_offset":932,"ancestor_record_offset":244,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3646,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":244,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3646,"record_offset":2936,"parent_record_offset":2884,"ancestor_record_offset":244,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3646,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":244,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3646,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":244,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3646,"record_offset":6504,"parent_record_offset":6096,"ancestor_record_offset":244,"inlinee":21869,"source_line":733,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3646,"record_offset":5712,"parent_record_offset":5672,"ancestor_record_offset":244,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3646,"record_offset":6036,"parent_record_offset":5672,"ancestor_record_offset":244,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3646,"record_offset":6964,"parent_record_offset":6908,"ancestor_record_offset":244,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":244,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":244,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3646,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":244,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/03646.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3646,"record_offset":6716,"parent_record_offset":6680,"ancestor_record_offset":244,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/03646.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>

// ------------------------------------------------------------------
// 函数: toolbar::photomode::do_photomode
// 信息 ID: 6da768826ac3c8f284e82183
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0b79338fca0490e5a6e156b5: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >)> > @ 0x15db90e -> 0x1b5c190 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights> >)> @ 0x1661142 -> 0x1b5c190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
