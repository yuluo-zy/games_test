// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 35aea732299c200d9c0e95c56f510100f86e5a934fcb6283dec7abd3a5e9dd33
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: grass_shader.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 472
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68C4
// 首条位置: {"module":472,"inlinee":26820}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16276,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":472,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":16168,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":472,"record_offset":15984,"rva":6684928,"va":5375394048}
// 编译实例 RVAs: 0x660100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1175722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e8560feeff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375394133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t573185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e801bf0800","size":5,"soft_fail":false,"target_va":5375967552,"va":5375394362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7955121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e8b1627900","size":5,"soft_fail":false,"target_va":5383349424,"va":5375394298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7966556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e85c8f7900","size":5,"soft_fail":false,"target_va":5383360832,"va":5375394271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfa2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7971089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e811a17900","size":5,"soft_fail":false,"target_va":5383365344,"va":5375394250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28199194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e81a49ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375394161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t313262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394048,"raw_hex":"e8aec70400","size":5,"soft_fail":false,"target_va":5375707584,"va":5375394317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16328,"parent_record_offset":15984,"ancestor_record_offset":15984,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
//     evidence={"module":472,"record_offset":16168,"parent_record_offset":15984,"ancestor_record_offset":15984,"inlinee":26820,"source_line":18,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":15984,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":472,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":15984,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":472,"record_offset":16360,"rva":6685312,"va":5375394432}
// 编译实例 RVAs: 0x660280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1176106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e8d60deeff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375394517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t572801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e881bd0800","size":5,"soft_fail":false,"target_va":5375967552,"va":5375394746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7954737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e831617900","size":5,"soft_fail":false,"target_va":5383349424,"va":5375394682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7966172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e8dc8d7900","size":5,"soft_fail":false,"target_va":5383360832,"va":5375394655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfa2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7970705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e8919f7900","size":5,"soft_fail":false,"target_va":5383365344,"va":5375394634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28198810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e89a47ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375394545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t312142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375394432,"raw_hex":"e84ec30400","size":5,"soft_fail":false,"target_va":5375706848,"va":5375394701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16708,"parent_record_offset":16360,"ancestor_record_offset":16360,"inlinee":26821,"source_line":292,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
//     evidence={"module":472,"record_offset":16548,"parent_record_offset":16360,"ancestor_record_offset":16360,"inlinee":26820,"source_line":18,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":472,"record_offset":16604,"parent_record_offset":16576,"ancestor_record_offset":16360,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":472,"record_offset":16576,"parent_record_offset":16548,"ancestor_record_offset":16360,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"}
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
// 完整结构及原始类型记录: grass_shader.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: grass_shader.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: grass_shader.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":472,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":472,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":16168,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":472,"record_offset":16276,"parent_record_offset":16168,"ancestor_record_offset":16168,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_transitive","evidence":{"module":472,"record_offset":16196,"parent_record_offset":16168,"ancestor_record_offset":15984,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":472,"record_offset":16224,"parent_record_offset":16196,"ancestor_record_offset":15984,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_transitive","evidence":{"module":472,"record_offset":16576,"parent_record_offset":16548,"ancestor_record_offset":16360,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":472,"record_offset":16604,"parent_record_offset":16576,"ancestor_record_offset":16360,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00472.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: grass_shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
// 信息 ID: a6627d5b6b368017c09c562f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader; module=472, inline_record=16168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader; module=472, inline_record=16548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// 信息 ID: 74edae4e8388f15708143c99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// 信息 ID: a93e487d8627a142654409bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
