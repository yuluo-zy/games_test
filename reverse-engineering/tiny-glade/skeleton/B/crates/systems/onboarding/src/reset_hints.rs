// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/onboarding/src/reset_hints.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 50b9ce1109b93cf875cf9a2c481ba32b21dc5122774f29d32774e04ee58c2c9e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_hints.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2515
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::reset_hints::reset_hints
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2515,"record_offset":264,"rva":29477072,"va":5398186192}
// 编译实例 RVAs: 0x1c1c8d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23831426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398186192,"raw_hex":"e87e5c94fe","size":5,"soft_fail":false,"target_va":5374355568,"va":5398186989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2515,"record_offset":1784,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2515,"record_offset":1504,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":2515,"record_offset":404,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":2515,"record_offset":2732,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2515,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2515,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2515,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2515,"record_offset":2896,"parent_record_offset":2856,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2515,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2515,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2515,"record_offset":2856,"parent_record_offset":2820,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2515,"record_offset":704,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2515,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2515,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2515,"record_offset":2820,"parent_record_offset":2756,"ancestor_record_offset":264,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":2515,"record_offset":516,"parent_record_offset":452,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2515,"record_offset":564,"parent_record_offset":516,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":2515,"record_offset":1548,"parent_record_offset":1504,"ancestor_record_offset":264,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":2515,"record_offset":452,"parent_record_offset":404,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2515,"record_offset":1596,"parent_record_offset":1548,"ancestor_record_offset":264,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2515,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2515,"record_offset":648,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2515,"record_offset":1644,"parent_record_offset":1596,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2515,"record_offset":1716,"parent_record_offset":1596,"ancestor_record_offset":264,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2515,"record_offset":1748,"parent_record_offset":1548,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2515,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2515,"record_offset":3008,"parent_record_offset":2856,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2515,"record_offset":1056,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2515,"record_offset":1344,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2515,"record_offset":1120,"parent_record_offset":1088,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2515,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/02515.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2515,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02515.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>", "tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>", "bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >

// ------------------------------------------------------------------
// 函数: system_onboarding::reset_hints::reset_hints
// 信息 ID: b454431e445c14583236f72d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 92475591082ef7dece8f2f0f: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> > @ 0xeae864 -> 0x1c1c8d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> > > @ 0x1c2b22c -> 0x1c1c8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
