// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/easing_functions.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 307709dc30d2a416b2fe5aed7090005ef69e4b9f44371a57105fa81b3f96299b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: easing_functions.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 763
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::EasingFunction::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":252,"rva":13197040,"va":5381906160}
// 编译实例 RVAs: 0xc95ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29941401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381906160,"raw_hex":"e899dec801","size":5,"soft_fail":false,"target_va":5411847808,"va":5381906402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29941485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381906160,"raw_hex":"e8eddec801","size":5,"soft_fail":false,"target_va":5411847856,"va":5381906366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29941810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381906160,"raw_hex":"e832e0c801","size":5,"soft_fail":false,"target_va":5411848144,"va":5381906329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29635192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381906160,"raw_hex":"e87832c401","size":5,"soft_fail":false,"target_va":5411542064,"va":5381906867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::easing_functions::circular_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":408,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44898,"source_line":71,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":760,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44902,"source_line":80,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":928,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44904,"source_line":39,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_in_out_elastic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":504,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44899,"source_line":56,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_out_back
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":808,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44903,"source_line":51,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::shoulder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":704,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44901,"source_line":90,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::sine_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":763,"record_offset":648,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":44900,"source_line":43,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":252,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":588,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":868,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":560,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":444,"parent_record_offset":408,"ancestor_record_offset":252,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF68
// 首条位置: {"module":763,"inlinee":44904}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":928,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::sine_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF64
// 首条位置: {"module":763,"inlinee":44900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":648,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_out_back
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF67
// 首条位置: {"module":763,"inlinee":44903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":868,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":896,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_in_out_elastic
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF63
// 首条位置: {"module":763,"inlinee":44899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":588,"parent_record_offset":504,"ancestor_record_offset":504,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":560,"parent_record_offset":504,"ancestor_record_offset":504,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::circular_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF62
// 首条位置: {"module":763,"inlinee":44898}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":763,"record_offset":444,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_in_out
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF66
// 首条位置: {"module":763,"inlinee":44902}
// 编译实例 RVAs: 0xc961e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::shoulder
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAF65
// 首条位置: {"module":763,"inlinee":44901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: easing_functions.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::easing_functions::EasingFunction
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: easing_functions.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::easing_functions::EasingFunction", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: easing_functions.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::easing_functions::EasingFunction::apply
// 信息 ID: 4fcf7d7ca5c196334ee31e1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 39 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x5064d4 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x5067d6 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x506bf2 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x77bda6 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x77c268 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x7a01f0 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a4efe -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::revert @ 0x858edb -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::reapply @ 0x859464 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x85994a -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x859bf3 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x859a90 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x859af8 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x86bbd3 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape @ 0x8e3cf3 -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x97ec0e -> 0xc95ef0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::circular_in_out
// 信息 ID: b1a7ca3c294cf8f1faf387c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_in_out
// 信息 ID: bb6f17f3f5975f04886d61f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f2536 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0x9f258f -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f4e8c -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0x9f4ee4 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f782d -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0x9f7886 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fa176 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0x9fa1cf -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fcad6 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0x9fcb2f -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0x9ff42d -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0x9ff493 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0xabfc03 -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze @ 0xac295a -> 0xc961e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=760; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_out
// 信息 ID: f324f0fdfe9a472bfd3e00ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_in_out_elastic
// 信息 ID: 95e46bdfcf04ce3fa343e9c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_out_back
// 信息 ID: 6fc63db7acb47bee2566dff4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::shoulder
// 信息 ID: a7ac9b23162fddf61bf073fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::sine_in_out
// 信息 ID: de4c18adfaf313e83483cbb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=763, inline_record=648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
