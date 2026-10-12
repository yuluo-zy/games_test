// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/remember_style.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 85e1ebf312dc54dd35e6788a1e97ed5bb76a2a629ccbd2ce1cad9beba7ad5e9d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: remember_style.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4004
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::style_sample::StyleSample::remember
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4004,"record_offset":892,"rva":42526256,"va":5411235376}
// 编译实例 RVAs: 0x288e630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: remember_style.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33114953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411235376,"raw_hex":"e8b7b406fe","size":5,"soft_fail":false,"target_va":5378120480,"va":5411235428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05d10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: remember_style.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30968447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411235376,"raw_hex":"e8817527fe","size":5,"soft_fail":false,"target_va":5380267280,"va":5411235722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_color::style_sample::StructureSample::remember
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/remember_style.rs
//     evidence={"module":4004,"record_offset":1028,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":134789,"source_line":17,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4004,"record_offset":1212,"parent_record_offset":1028,"ancestor_record_offset":892,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   [pdb_inline_transitive] system_color::material_color_mapping::stone_to_wood
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":4004,"record_offset":1136,"parent_record_offset":1028,"ancestor_record_offset":892,"inlinee":108233,"source_line":3,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   [pdb_inline_transitive] system_color::material_color_mapping::wood_to_stair
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":4004,"record_offset":1184,"parent_record_offset":1028,"ancestor_record_offset":892,"inlinee":108234,"source_line":40,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_sample::StructureSample::remember
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E85
// 首条位置: {"module":4004,"inlinee":134789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4004,"record_offset":1212,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   [pdb_inline_direct] system_color::material_color_mapping::stone_to_wood
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":4004,"record_offset":1136,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":108233,"source_line":3,"archive":"_pdb/C/modules/04004.jsonl.gz"}
//   [pdb_inline_direct] system_color::material_color_mapping::wood_to_stair
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/material_color_mapping.rs
//     evidence={"module":4004,"record_offset":1184,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":108234,"source_line":40,"archive":"_pdb/C/modules/04004.jsonl.gz"}
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
// 完整结构及原始类型记录: remember_style.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: remember_style.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: remember_style.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_color::style_sample::StructureSample::remember
// 信息 ID: e1bbdb7e55f584ed8acc500c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::style_sample::StyleSample::remember; module=4004, inline_record=1028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_sample::StyleSample::remember
// 信息 ID: 6afa249caf512ee46a8102a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_picker::style_picker @ 0x2148216 -> 0x288e630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
