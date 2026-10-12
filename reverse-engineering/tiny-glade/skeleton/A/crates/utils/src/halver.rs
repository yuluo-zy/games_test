// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/halver.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 32db71b288197ef9612e59193a348fd99eb644c55153c08603b2b26c39c66c6d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: halver.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 616, 1237, 1704, 1923, 2796
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use half; // namespace candidate
//   package identities: [{"name":"half","source":"PDB_registry_path_only","version":"2.4.1"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$0::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1704,"record_offset":2272,"rva":18774432,"va":5387483552}
// 编译实例 RVAs: 0x11e79a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15711520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387483552,"raw_hex":"e820bdef00","size":5,"soft_fail":false,"target_va":5403195376,"va":5387483851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1405fe3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-12490509","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387483552,"raw_hex":"e9f36841ff","size":5,"soft_fail":false,"target_va":5374993376,"va":5387483880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] half::binary16::f16::from_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16.rs
//     evidence={"module":1704,"record_offset":2388,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":25677,"source_line":65,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1704,"record_offset":2664,"parent_record_offset":2548,"ancestor_record_offset":2272,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1704,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2272,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1704,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2272,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":1704,"record_offset":2424,"parent_record_offset":2388,"ancestor_record_offset":2272,"inlinee":25679,"source_line":64,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16_fallback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":1704,"record_offset":2732,"parent_record_offset":2424,"ancestor_record_offset":2272,"inlinee":25688,"source_line":482,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::f16c
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\macros.rs
//     evidence={"module":1704,"record_offset":2492,"parent_record_offset":2424,"ancestor_record_offset":2272,"inlinee":25681,"source_line":170,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":1704,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2272,"inlinee":25687,"source_line":100,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":1704,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":2272,"inlinee":25685,"source_line":180,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":1704,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":2272,"inlinee":25690,"source_line":188,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\mod.rs
//     evidence={"module":1704,"record_offset":2520,"parent_record_offset":2492,"ancestor_record_offset":2272,"inlinee":25683,"source_line":81,"archive":"_pdb/A/modules/01704.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$1::to_f16
// 记录形式: inlinee_declaration; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D0A
// 首条位置: {"module":616,"inlinee":36106}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$2::to_f16
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE972
// 首条位置: {"module":1237,"inlinee":59762}
// 编译实例 RVAs: 0x11e7af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387483888,"raw_hex":"e8b5fbffff","size":5,"soft_fail":false,"target_va":5387482832,"va":5387483926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$3::to_f16
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11D52
// 首条位置: {"module":1704,"record_offset":2936,"rva":18774832,"va":5387483952}
// 编译实例 RVAs: 0x11e7b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387483952,"raw_hex":"e87bfbffff","size":5,"soft_fail":false,"target_va":5387482832,"va":5387483984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$5::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1237,"record_offset":368,"rva":13360208,"va":5382069328}
// 编译实例 RVAs: 0xcbdc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5413438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382069328,"raw_hex":"e83e9a5200","size":5,"soft_fail":false,"target_va":5387482832,"va":5382069389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1237,"record_offset":488,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":23355,"source_line":112,"archive":"_pdb/A/modules/01237.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$2::to_f16
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":1237,"record_offset":512,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":59762,"source_line":28,"archive":"_pdb/A/modules/01237.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$6::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2796,"record_offset":244,"rva":25066544,"va":5393775664}
// 编译实例 RVAs: 0x17e7c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6292879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393775664,"raw_hex":"e871fa9fff","size":5,"soft_fail":false,"target_va":5387482832,"va":5393775706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":2796,"record_offset":364,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":90025,"source_line":129,"archive":"_pdb/A/modules/02796.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$3::to_f16
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":2796,"record_offset":388,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":73042,"source_line":37,"archive":"_pdb/A/modules/02796.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$7::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1923,"record_offset":244,"rva":19697120,"va":5388406240}
// 编译实例 RVAs: 0x12c8de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-923455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388406240,"raw_hex":"e8c1e8f1ff","size":5,"soft_fail":false,"target_va":5387482832,"va":5388406282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1923,"record_offset":364,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":77155,"source_line":404,"archive":"_pdb/A/modules/01923.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$3::to_f16
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":1923,"record_offset":388,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":73042,"source_line":37,"archive":"_pdb/A/modules/01923.jsonl.gz"}
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
// 完整结构及原始类型记录: halver.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: halver.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: halver.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::halver::impl$0::to_f16
// 信息 ID: 2ece54a704bca696d76e4450
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcb41 -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dea30 -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8decf5 -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd258 -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd45c -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0df9 -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e106e -> 0x11e79a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$1::to_f16
// 信息 ID: d4b2888f53a34ff745e20892
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$2::to_f16
// 信息 ID: b177acfcc9bfff1697c8ae9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcb36 -> 0x11e7af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd231 -> 0x11e7af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd43a -> 0x11e7af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41493 -> 0x11e7af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41eb2 -> 0x11e7af0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::halver::impl$5::to_f16; module=1237, inline_record=512; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$3::to_f16
// 信息 ID: 8c5b37c9355ca2e3d3b6a4dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8de9a8 -> 0x11e7b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8decdb -> 0x11e7b30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::halver::impl$7::to_f16; module=1923, inline_record=388; 不是独立调用边。
// [PDB 内联宿主] utils::halver::impl$6::to_f16; module=2796, inline_record=388; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$5::to_f16
// 信息 ID: e1474422d78c78f0f53e14d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8de9e1 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8decea -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0dc2 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e1063 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd188d0 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18a91 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18d27 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd183f4 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd19be9 -> 0xcbdc50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$6::to_f16
// 信息 ID: 8b86b64e2e9c8fa996842549
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd19bdc -> 0x17e7c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$7::to_f16
// 信息 ID: 5a952ea0c43bc15d4513697e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcaea -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8de980 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8deccf -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd168 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dd406 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e0d74 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e1054 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd188c1 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18a62 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18d00 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd183e5 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd19bb9 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41458 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41e44 -> 0x12c8de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
