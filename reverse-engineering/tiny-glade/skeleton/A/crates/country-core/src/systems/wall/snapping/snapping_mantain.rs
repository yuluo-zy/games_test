// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/snapping/snapping_mantain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 09f2875e495f73a9c0e2ebf109856679417517fa78d33f136799841c50ae7046
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_mantain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 453
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_mantain::check_for_disable_snapping
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":22804,"rva":6412976,"va":5375122096}
// 编译实例 RVAs: 0x61dab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3817299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122096,"raw_hex":"e8adc0c5ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5375122174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":23296,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":23036,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mod_key::Modifiers::disable_snapping
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mod_key.rs
//     evidence={"module":453,"record_offset":22980,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":22254,"source_line":19,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":453,"record_offset":23224,"parent_record_offset":23192,"ancestor_record_offset":22804,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":23192,"parent_record_offset":23156,"ancestor_record_offset":22804,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":453,"record_offset":23156,"parent_record_offset":23120,"ancestor_record_offset":22804,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":23088,"parent_record_offset":23060,"ancestor_record_offset":22804,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":22804,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":453,"record_offset":23120,"parent_record_offset":23036,"ancestor_record_offset":22804,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":453,"record_offset":23256,"parent_record_offset":23156,"ancestor_record_offset":22804,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_mantain::snapping_signifiers
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":23620,"rva":6413184,"va":5375122304}
// 编译实例 RVAs: 0x61db80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapping_mantain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1821502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122304,"raw_hex":"e83ecb1b00","size":5,"soft_fail":false,"target_va":5376944000,"va":5375122493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140edd820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_mantain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9174079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122304,"raw_hex":"e83ffc8b00","size":5,"soft_fail":false,"target_va":5384296480,"va":5375122396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140edd9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_mantain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9174563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375122304,"raw_hex":"e823fe8b00","size":5,"soft_fail":false,"target_va":5384296912,"va":5375122344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":23780,"parent_record_offset":23620,"ancestor_record_offset":23620,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":23620,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_mantain::snapping_mantain
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":453,"record_offset":24120,"rva":6413504,"va":5375122624}
// 编译实例 RVAs: 0x61dcc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":24336,"parent_record_offset":24120,"ancestor_record_offset":24120,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/mod.rs
//     evidence={"module":453,"record_offset":24284,"parent_record_offset":24120,"ancestor_record_offset":24120,"inlinee":25999,"source_line":30,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":453,"record_offset":24360,"parent_record_offset":24336,"ancestor_record_offset":24120,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00453.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_mantain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_mantain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_mantain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_mantain::check_for_disable_snapping
// 信息 ID: 3c97260e50f4cdbd36bd890a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] db92af87cf1005c257b86711: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)> > @ 0xad5f8 -> 0x61dab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>)> @ 0x2682cd -> 0x61dab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_mantain::snapping_mantain
// 信息 ID: 83abef8a0975473bd84c9637
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0dcc397b87b2a358b39e9cc0: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)> > @ 0xa3f07 -> 0x61dcc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>)> @ 0x268071 -> 0x61dcc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_mantain::snapping_signifiers
// 信息 ID: 1d706638f4bf184abed2275b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8f4a91511dee62637f06f30d: void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >)> > @ 0xb7049 -> 0x61db80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToSegmentPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToEndPreview> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::SnappingToEndPreview>,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::SnappingToSegmentPreview> >)> @ 0x267f01 -> 0x61db80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
