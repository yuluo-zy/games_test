// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/roofs/roof_noise.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 666a4fecc675aa72119135f67d55e80a956881f33a8bd102c9adb09b673b9b8a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_noise.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 477, 493, 496
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::apply_roof_noise
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":477,"record_offset":3200,"rva":6742336,"va":5375451456}
// 编译实例 RVAs: 0x66e140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_noise::gpunoise::noise
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t356251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375451456,"raw_hex":"e89b6f0500","size":5,"soft_fail":false,"target_va":5375807792,"va":5375451536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":477,"record_offset":3408,"parent_record_offset":3200,"ancestor_record_offset":3200,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":477,"record_offset":3348,"parent_record_offset":3200,"ancestor_record_offset":3200,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":477,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":3200,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":477,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":3200,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::revert_roof_noise
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69A9
// 首条位置: {"module":493,"inlinee":27049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":493,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":20540,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":493,"record_offset":20592,"parent_record_offset":20568,"ancestor_record_offset":20540,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CCB
// 首条位置: {"module":496,"inlinee":27851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4000,"parent_record_offset":3968,"ancestor_record_offset":3968,"inlinee":7169,"source_line":1639,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CCC
// 首条位置: {"module":496,"inlinee":27852}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4140,"parent_record_offset":4088,"ancestor_record_offset":4088,"inlinee":7169,"source_line":1639,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":4088,"inlinee":27853,"source_line":1703,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CCA
// 首条位置: {"module":496,"inlinee":27850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3944,"inlinee":27851,"source_line":24,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":4088,"parent_record_offset":3944,"ancestor_record_offset":3944,"inlinee":27852,"source_line":34,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4000,"parent_record_offset":3968,"ancestor_record_offset":3944,"inlinee":7169,"source_line":1639,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3944,"inlinee":27853,"source_line":1703,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CD2
// 首条位置: {"module":496,"inlinee":27858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::noise
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":496,"record_offset":3604,"rva":7098672,"va":5375807792}
// 编译实例 RVAs: 0x6c5130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: roof_noise.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27786264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375807792,"raw_hex":"e818fca701","size":5,"soft_fail":false,"target_va":5403594096,"va":5375807827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::hash2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":3944,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27850,"source_line":50,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":5184,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27859,"source_line":79,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":5120,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27858,"source_line":54,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::fract_gl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":496,"record_offset":3832,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27848,"source_line":638,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":496,"record_offset":4852,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27855,"source_line":991,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$21::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":496,"record_offset":4940,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":27857,"source_line":1089,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":496,"record_offset":4764,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":496,"record_offset":3776,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4000,"parent_record_offset":3968,"ancestor_record_offset":3604,"inlinee":7169,"source_line":1639,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":496,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3604,"inlinee":27853,"source_line":1703,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":496,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":3604,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":496,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3604,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3604,"inlinee":27851,"source_line":24,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_noise.rs
//     evidence={"module":496,"record_offset":4088,"parent_record_offset":3944,"ancestor_record_offset":3604,"inlinee":27852,"source_line":34,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":496,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3604,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00496.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CD3
// 首条位置: {"module":496,"inlinee":27859}
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
// 完整结构及原始类型记录: roof_noise.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_noise.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_noise.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::apply_roof_noise
// 信息 ID: 1cf43d98da87b88d1c484892
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1986406 -> 0x66e140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3cc1d -> 0x66e140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash1
// 信息 ID: 4871a8636d2cf7ca2d2b5412
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=3968; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash2
// 信息 ID: 38a0454e9361af5208d41705
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=3944; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4204; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4328; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::hash_combine2
// 信息 ID: 3b54a3b0ebd8eb94f4c71923
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4088; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4232; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4368; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=4648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::lerp
// 信息 ID: 105c4d8212e342fbb13f0bde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=5184; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=5212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::noise
// 信息 ID: 8a8bd987c38c2ce049ec81bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::roofs::roof_noise::apply_roof_noise @ 0x66e190 -> 0x6c5130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped @ 0x6bb1bf -> 0x6c5130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::gpunoise::uint_to_u01_float
// 信息 ID: 253ddef3707b5d423f2da36b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=5120; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::gpunoise::noise; module=496, inline_record=5152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_noise::revert_roof_noise
// 信息 ID: 769e835352fa13a550c954f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped; module=493, inline_record=20540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
