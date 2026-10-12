// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/easing_functions.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 307709dc30d2a416b2fe5aed7090005ef69e4b9f44371a57105fa81b3f96299b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: easing_functions.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 758
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
// 首条位置: {"module":758,"record_offset":376,"rva":14588080,"va":5383297200}
// 编译实例 RVAs: 0xde98b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33760252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297200,"raw_hex":"e8fc230302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383297439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bdd0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33760333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297200,"raw_hex":"e84d240302","size":5,"soft_fail":false,"target_va":5417057744,"va":5383297406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: easing_functions.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33760658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297200,"raw_hex":"e892250302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383297369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33324299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297200,"raw_hex":"e80b7dfc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383297904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::easing_functions::circular_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":532,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49048,"source_line":71,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":944,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49052,"source_line":80,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::cubic_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":1140,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49054,"source_line":39,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_in_out_elastic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":684,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49049,"source_line":56,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::ease_out_back
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":992,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49053,"source_line":51,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::shoulder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":888,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49051,"source_line":90,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] utils::easing_functions::sine_in_out
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/easing_functions.rs
//     evidence={"module":758,"record_offset":832,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49050,"source_line":43,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":376,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":376,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":376,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":376,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":772,"parent_record_offset":684,"ancestor_record_offset":376,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":1052,"parent_record_offset":992,"ancestor_record_offset":376,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":744,"parent_record_offset":684,"ancestor_record_offset":376,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":568,"parent_record_offset":532,"ancestor_record_offset":376,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF9E
// 首条位置: {"module":758,"inlinee":49054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":1140,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::sine_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF9A
// 首条位置: {"module":758,"inlinee":49050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":832,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_out_back
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF9D
// 首条位置: {"module":758,"inlinee":49053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":1052,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":1108,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":992,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::ease_in_out_elastic
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF99
// 首条位置: {"module":758,"inlinee":49049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":772,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":744,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::circular_in_out
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF98
// 首条位置: {"module":758,"inlinee":49048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":758,"record_offset":568,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":758,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":532,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00758.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::easing_functions::cubic_in_out
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBF9C
// 首条位置: {"module":758,"inlinee":49052}
// 编译实例 RVAs: 0xde9850
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
// IPI IDs: 0xBF9B
// 首条位置: {"module":758,"inlinee":49051}
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
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: easing_functions.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::easing_functions::EasingFunction::apply
// 信息 ID: e5ee2eacf5e78f24ea1089b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311ce3 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x311fd3 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x3123ef -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x81bb0c -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks @ 0x81c00a -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c2d4 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860df0 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x920fea -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x9221a5 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x9222c3 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x922323 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve @ 0x92249b -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::revert @ 0x9229be -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::terrain_edit::TerrainEdit::reapply @ 0x923014 -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x9286df -> 0xde98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0xa3ebab -> 0xde98b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::circular_in_out
// 信息 ID: 8ee4cdd5845e5c13eca6ce22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_in_out
// 信息 ID: e243a76329fac5a201730a04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64e8b -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64ee8 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67651 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb676a6 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69e13 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69e68 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c6db -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c738 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eeb4 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ef0a -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7172b -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb71788 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze @ 0xbe779b -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_plant_paint::replay_plant_paint @ 0x17c4292 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1> @ 0x19e0161 -> 0xde9850 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=944; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::cubic_out
// 信息 ID: ba49e26588cb3b66d8cc7bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=1140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_in_out_elastic
// 信息 ID: 19f806fa135e7135c52c6dcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::ease_out_back
// 信息 ID: 540dcf151219388ec3e4f607
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::shoulder
// 信息 ID: b1b8fa36cb7b239b371c6850
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::easing_functions::sine_in_out
// 信息 ID: a808f95be6a8d2f942af0ab9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::easing_functions::EasingFunction::apply; module=758, inline_record=832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
