// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/child_wall.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8835f64b1f573e8166e7c4d6d83254809cbdce0d7bfe3343804fb0203ecfd05f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: child_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 423, 461, 1426
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::child_wall::ChildWall::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FE3
// 首条位置: {"module":423,"inlinee":24547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":24492,"parent_record_offset":24464,"ancestor_record_offset":24464,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":24464,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":24464,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::child_wall::ChildWall::slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1426,"record_offset":113308,"rva":17298576,"va":5386007696}
// 编译实例 RVAs: 0x107f490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16971112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386007696,"raw_hex":"e868f50201","size":5,"soft_fail":false,"target_va":5402979152,"va":5386008035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16970928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386007696,"raw_hex":"e8b0f40201","size":5,"soft_fail":false,"target_va":5402978944,"va":5386008011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9067816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386007696,"raw_hex":"e8d8a275ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5386007763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1426,"record_offset":113504,"parent_record_offset":113308,"ancestor_record_offset":113308,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1426,"record_offset":113448,"parent_record_offset":113308,"ancestor_record_offset":113308,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1426,"record_offset":113588,"parent_record_offset":113564,"ancestor_record_offset":113308,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1426,"record_offset":113612,"parent_record_offset":113588,"ancestor_record_offset":113308,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1426,"record_offset":113648,"parent_record_offset":113504,"ancestor_record_offset":113308,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1426,"record_offset":113676,"parent_record_offset":113504,"ancestor_record_offset":113308,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1426,"record_offset":113564,"parent_record_offset":113540,"ancestor_record_offset":113308,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1426,"record_offset":113540,"parent_record_offset":113504,"ancestor_record_offset":113308,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01426.jsonl.gz"}
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
// 完整结构及原始类型记录: child_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::child_wall::ChildWall
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::utils::child_wall::ChildWall", "country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>,alloc::slice::stable_sort::closure_env$0<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>", "alloc::slice::stable_sort::closure_env$0<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "alloc::slice::stable_sort::closure_env$1<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "alloc::slice::stable_sort::closure_env$2<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "alloc::slice::stable_sort::closure_env$3<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<country_core::utils::child_wall::ChildWall,alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["country_core::utils::child_wall::ChildWall", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::utils::child_wall::ChildWall,country_core::systems::wall::eraser::archivist::impl$2::finish_shrink_or_split_event::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: child_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: child_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 24; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::child_wall::ChildWall::new
// 信息 ID: bc7714f1dd108dc64a22bd08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=24464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=70220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::child_wall::ChildWall::slice
// 信息 ID: d2f7bc8df039ae84006fc3a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall @ 0x5d580b -> 0x107f490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event @ 0x5d5b50 -> 0x107f490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
