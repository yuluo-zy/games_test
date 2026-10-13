// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/photomode/photomode_buttons.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1c03067083010219d629c467c27a91a3c484850cb48e4fe6f887cf1797e7a3a8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: photomode_buttons.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3647
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::photomode_buttons::photomode_buttons
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3647,"record_offset":244,"rva":28693120,"va":5397402240}
// 编译实例 RVAs: 0x1b5d280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400693c0
//     owner=unresolved_namespace; count=1
//     target_aliases=33 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28263420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e804bc50fe","size":5,"soft_fail":false,"target_va":5369140160,"va":5397403575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25089060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8dc2b81fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5397403263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24654260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e84cce87fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397403951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24654781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e843cc87fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397404488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140517ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23354395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8e5a39bfe","size":5,"soft_fail":false,"target_va":5374048960,"va":5397403350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23004538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e886faa0fe","size":5,"soft_fail":false,"target_va":5374398944,"va":5397403477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22232110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8d2c3acfe","size":5,"soft_fail":false,"target_va":5375171056,"va":5397403161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140648fa0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22103939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e87db8aefe","size":5,"soft_fail":false,"target_va":5375299488,"va":5397403422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6188978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8b26f5e00","size":5,"soft_fail":false,"target_va":5403593360,"va":5397404377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5572803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8c3085500","size":5,"soft_fail":false,"target_va":5402978144,"va":5397405336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5514738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8f2255400","size":5,"soft_fail":false,"target_va":5402919072,"va":5397404329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-26098029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e893c671fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5397403080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19878199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8c9aed0fe","size":5,"soft_fail":false,"target_va":5377527120,"va":5397405314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20158826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e89666ccfe","size":5,"soft_fail":false,"target_va":5377245296,"va":5397404117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5310599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e879f7aeff","size":5,"soft_fail":false,"target_va":5392092384,"va":5397402978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5566773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e835f15400","size":5,"soft_fail":false,"target_va":5402972080,"va":5397405302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20157706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8f66accfe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397404229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21928568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e88865b1fe","size":5,"soft_fail":false,"target_va":5375475424,"va":5397403987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next_demo
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21928014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8b267b1fe","size":5,"soft_fail":false,"target_va":5375475312,"va":5397403321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::activate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22155607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8a9eeadfe","size":5,"soft_fail":false,"target_va":5375248288,"va":5397403890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::deactivate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22155368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e898efadfe","size":5,"soft_fail":false,"target_va":5375248512,"va":5397403875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::is_active
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22155056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8d0f0adfe","size":5,"soft_fail":false,"target_va":5375248800,"va":5397403851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2778543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8af652a00","size":5,"soft_fail":false,"target_va":5400181776,"va":5397403228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21249213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e843c3bbfe","size":5,"soft_fail":false,"target_va":5376153904,"va":5397403112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19995701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e8cbe3cefe","size":5,"soft_fail":false,"target_va":5377408752,"va":5397404448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19963375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e81162cffe","size":5,"soft_fail":false,"target_va":5377439776,"va":5397403146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19962061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397402240,"raw_hex":"e83367cffe","size":5,"soft_fail":false,"target_va":5377441120,"va":5397403176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3647,"record_offset":1804,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3647,"record_offset":7252,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":448,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":1356,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":988,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3647,"record_offset":6476,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4012,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4104,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3647,"record_offset":1300,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3647,"record_offset":6280,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3647,"record_offset":1328,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3647,"record_offset":1776,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::photo_mode::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/photo_mode.rs
//     evidence={"module":3647,"record_offset":7224,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":108753,"source_line":18,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3647,"record_offset":5056,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":28174,"source_line":2262,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":1608,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":3096,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":1636,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3647,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3647,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3647,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3647,"record_offset":3484,"parent_record_offset":3448,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3647,"record_offset":5536,"parent_record_offset":5480,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":4360,"parent_record_offset":4332,"ancestor_record_offset":244,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":4332,"parent_record_offset":4296,"ancestor_record_offset":244,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":4296,"parent_record_offset":4264,"ancestor_record_offset":244,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":244,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":244,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":3604,"parent_record_offset":3420,"ancestor_record_offset":244,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3647,"record_offset":5480,"parent_record_offset":5452,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3647,"record_offset":3352,"parent_record_offset":3316,"ancestor_record_offset":244,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3647,"record_offset":3316,"parent_record_offset":3280,"ancestor_record_offset":244,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3647,"record_offset":3280,"parent_record_offset":3244,"ancestor_record_offset":244,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":1512,"parent_record_offset":1472,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":4412,"parent_record_offset":4216,"ancestor_record_offset":244,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":1472,"parent_record_offset":1436,"ancestor_record_offset":244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":4264,"parent_record_offset":4216,"ancestor_record_offset":244,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":4580,"parent_record_offset":4216,"ancestor_record_offset":244,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":3392,"parent_record_offset":3352,"ancestor_record_offset":244,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3647,"record_offset":3244,"parent_record_offset":3204,"ancestor_record_offset":244,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3647,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3647,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3647,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":244,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":244,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":1436,"parent_record_offset":1356,"ancestor_record_offset":244,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":244,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3647,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":244,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3647,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":244,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3647,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":244,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3647,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":244,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3647,"record_offset":6832,"parent_record_offset":6724,"ancestor_record_offset":244,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3647,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":244,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3647,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":244,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3647,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":244,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3647,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":244,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3647,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":244,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3647,"record_offset":7132,"parent_record_offset":6500,"ancestor_record_offset":244,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3647,"record_offset":6724,"parent_record_offset":6500,"ancestor_record_offset":244,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3647,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":244,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3647,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":244,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3647,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":244,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3647,"record_offset":7044,"parent_record_offset":6980,"ancestor_record_offset":244,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3647,"record_offset":6548,"parent_record_offset":6500,"ancestor_record_offset":244,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3647,"record_offset":6772,"parent_record_offset":6748,"ancestor_record_offset":244,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3647,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":244,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3647,"record_offset":7180,"parent_record_offset":7156,"ancestor_record_offset":244,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3647,"record_offset":6640,"parent_record_offset":6500,"ancestor_record_offset":244,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":244,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4044,"parent_record_offset":4012,"ancestor_record_offset":244,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":244,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3647,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":244,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3647,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":244,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3647,"record_offset":4612,"parent_record_offset":4176,"ancestor_record_offset":244,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3647,"record_offset":4216,"parent_record_offset":4176,"ancestor_record_offset":244,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3647,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":244,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3647,"record_offset":3716,"parent_record_offset":3204,"ancestor_record_offset":244,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3647,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3647,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":244,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3647,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":244,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3647,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":244,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3647,"record_offset":580,"parent_record_offset":536,"ancestor_record_offset":244,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3647,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":244,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3647,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":244,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3647,"record_offset":1120,"parent_record_offset":1040,"ancestor_record_offset":244,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3647,"record_offset":3644,"parent_record_offset":3352,"ancestor_record_offset":244,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3647,"record_offset":5320,"parent_record_offset":5056,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3647,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3647,"record_offset":4496,"parent_record_offset":4216,"ancestor_record_offset":244,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3647,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":244,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3647,"record_offset":6608,"parent_record_offset":6576,"ancestor_record_offset":244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3647,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3647,"record_offset":1568,"parent_record_offset":1472,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3647,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":244,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3647,"record_offset":644,"parent_record_offset":536,"ancestor_record_offset":244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3647,"record_offset":832,"parent_record_offset":536,"ancestor_record_offset":244,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3647,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3647,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":244,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3647,"record_offset":680,"parent_record_offset":644,"ancestor_record_offset":244,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3647,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":244,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3647,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":244,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3647,"record_offset":1196,"parent_record_offset":1144,"ancestor_record_offset":244,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3647,"record_offset":3132,"parent_record_offset":3096,"ancestor_record_offset":244,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3647,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":244,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3647,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":244,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":3647,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":244,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3647,"record_offset":5288,"parent_record_offset":5056,"ancestor_record_offset":244,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":3647,"record_offset":5092,"parent_record_offset":5056,"ancestor_record_offset":244,"inlinee":28175,"source_line":2518,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":244,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":244,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":3647,"record_offset":5120,"parent_record_offset":5056,"ancestor_record_offset":244,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3647,"record_offset":3168,"parent_record_offset":3132,"ancestor_record_offset":244,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/03647.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":3647,"record_offset":3204,"parent_record_offset":3168,"ancestor_record_offset":244,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/03647.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >", "bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>", "bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >", "bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::photomode::photomode_buttons::photomode_buttons
// 信息 ID: 23ce1fd5074a9810aeff4d7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 71acf78f0e9d62017a748118: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands)> > @ 0x15df6d1 -> 0x1b5d280 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >,bevy_ecs::system::commands::Commands)> @ 0x1b9d8ee -> 0x1b5d280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
