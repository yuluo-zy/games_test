// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/angle_range.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d07b056d2f8d20824aa1c1c9bdf80dc214dd29c5043609422f2651a05127252b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: angle_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4247
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::angle_range::AngleRange::clamp
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4247,"record_offset":436,"rva":31225680,"va":5399934800}
// 编译实例 RVAs: 0x1dc7750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: angle_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3659298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399934800,"raw_hex":"e822d63700","size":5,"soft_fail":false,"target_va":5403594144,"va":5399934841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::angle_range::wrap_angle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/angle_range.rs
//     evidence={"module":4247,"record_offset":560,"parent_record_offset":436,"ancestor_record_offset":436,"inlinee":119591,"source_line":83,"archive":"_pdb/A/modules/04247.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4247,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":436,"inlinee":6485,"source_line":363,"archive":"_pdb/A/modules/04247.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::angle_range::AngleRange::from_min_and_len
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4247,"record_offset":244,"rva":31225568,"va":5399934688}
// 编译实例 RVAs: 0x1dc76e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: angle_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3659418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399934688,"raw_hex":"e89ad63700","size":5,"soft_fail":false,"target_va":5403594144,"va":5399934721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3044357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399934688,"raw_hex":"e805742e00","size":5,"soft_fail":false,"target_va":5402979152,"va":5399934790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::angle_range::wrap_angle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/angle_range.rs
//     evidence={"module":4247,"record_offset":376,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":119591,"source_line":83,"archive":"_pdb/A/modules/04247.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4247,"record_offset":400,"parent_record_offset":376,"ancestor_record_offset":244,"inlinee":6485,"source_line":363,"archive":"_pdb/A/modules/04247.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::angle_range::wrap_angle
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D327
// 首条位置: {"module":4247,"inlinee":119591}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4247,"record_offset":400,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":6485,"source_line":363,"archive":"_pdb/A/modules/04247.jsonl.gz"}
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
// 完整结构及原始类型记录: angle_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::angle_range::AngleRange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: angle_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: angle_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::angle_range::AngleRange::clamp
// 信息 ID: 52fee1b0aa3c1079d498627e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::color_menu::color_menu @ 0x1aa97f7 -> 0x1dc7750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::angle_range::AngleRange::from_min_and_len
// 信息 ID: d1a8f30a24620f5a7a07d03d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::radial_menu::ColorRadialMenu::create @ 0x1dc91b0 -> 0x1dc76e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::angle_range::wrap_angle
// 信息 ID: 4a00de6c7c1de6bdd5a3e2b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::angle_range::AngleRange::from_min_and_len; module=4247, inline_record=376; 不是独立调用边。
// [PDB 内联宿主] utils::angle_range::AngleRange::clamp; module=4247, inline_record=560; 不是独立调用边。
// [PDB 内联宿主] utils::angle_range::AngleRange::clamp; module=4247, inline_record=616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
