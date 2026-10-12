// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/onboarding/src/reset_hints.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7da9daedbbf09761912c13f0c86ba5bb236beef2e45825569e42af3a4c467ef1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_hints.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2185
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::reset_hints::reset_hints
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2185,"record_offset":256,"rva":21414080,"va":5390123200}
// 编译实例 RVAs: 0x146c0c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18818646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390123200,"raw_hex":"e8aad9e0fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5390123697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2185,"record_offset":748,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":396,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":1468,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":664,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2185,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":256,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2185,"record_offset":1628,"parent_record_offset":1588,"ancestor_record_offset":256,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2185,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":256,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2185,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2185,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":1552,"parent_record_offset":1468,"ancestor_record_offset":256,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":452,"parent_record_offset":416,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":2185,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":256,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":2185,"record_offset":416,"parent_record_offset":396,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2185,"record_offset":524,"parent_record_offset":476,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2185,"record_offset":588,"parent_record_offset":476,"ancestor_record_offset":256,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2185,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02185.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2185,"record_offset":1684,"parent_record_offset":1588,"ancestor_record_offset":256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02185.jsonl.gz"}
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
// 完整结构及原始类型记录: reset_hints.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reset_hints.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reset_hints.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>", "bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >

// ------------------------------------------------------------------
// 函数: system_onboarding::reset_hints::reset_hints
// 信息 ID: cf7aec440b162d7fb24868c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8fda0cd802f7452dbdba50e: void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> @ 0x99abc1 -> 0x146c0c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::onboarding::ResetHints>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> > @ 0x1453cd9 -> 0x146c0c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
