// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/reset_hints.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b9c5f04d7a33a5522cc08a69eb35518f98609b68674458ec15e25624d890e3a0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reset_hints.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3953
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
// 首条位置: {"module":3953,"record_offset":272,"rva":42408544,"va":5411117664}
// 编译实例 RVAs: 0x2871a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reset_hints.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5232181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411117664,"raw_hex":"e835d64f00","size":5,"soft_fail":false,"target_va":5416350720,"va":5411118534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3953,"record_offset":1788,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3953,"record_offset":1508,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3953,"record_offset":412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3953,"record_offset":2856,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3953,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3953,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3953,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":3052,"parent_record_offset":3012,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":3012,"parent_record_offset":2980,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3953,"record_offset":708,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3953,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3953,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3953,"record_offset":572,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3953,"record_offset":1552,"parent_record_offset":1508,"ancestor_record_offset":272,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3953,"record_offset":524,"parent_record_offset":460,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3953,"record_offset":2944,"parent_record_offset":2880,"ancestor_record_offset":272,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3953,"record_offset":460,"parent_record_offset":412,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3953,"record_offset":1600,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3953,"record_offset":1156,"parent_record_offset":1124,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3953,"record_offset":652,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3953,"record_offset":1648,"parent_record_offset":1600,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3953,"record_offset":1720,"parent_record_offset":1600,"ancestor_record_offset":272,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3953,"record_offset":1752,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3953,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3953,"record_offset":3164,"parent_record_offset":3012,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3953,"record_offset":1060,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3953,"record_offset":1348,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3953,"record_offset":1124,"parent_record_offset":1092,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3953,"record_offset":1092,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3953,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03953.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>", "tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>", "bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>", "bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >

// ------------------------------------------------------------------
// 函数: system_onboarding::reset_hints::reset_hints
// 信息 ID: 2947a4cc2a48f98ba91e5862
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e6bc8498b31b35da3fa57f93: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> @ 0x20e42b2 -> 0x2871a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::onboarding::TutorialOp>,tuple$<bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain::HintTerrainConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions> >,bevy_ecs::change_detection::ResMut<system_onboarding::hint_focused_edit::FocusedEditHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_undo::UndoHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_quick_tool_palette::QuickToolPaletteHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_roof_shape::HintRoofShapeConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_stairs::StairsHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_wall_group_selection::WallGroupSelectionHintConditions>,bevy_ecs::change_detection::ResMut<system_onboarding::hint_camera_progress::CameraHintProgress>,bevy_ecs::change_detection::ResMut<enum2$<system_onboarding::hint_camera_controls::CameraHintsState> >,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >)> > @ 0x20f9507 -> 0x2871a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
