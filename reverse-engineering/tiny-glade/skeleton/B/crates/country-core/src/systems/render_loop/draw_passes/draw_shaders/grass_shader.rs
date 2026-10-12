// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d88ae5e225bc1594db00646dc2ae74ddaa220bff6d159fdbf11db23eff80fe1b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: grass_shader.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 589
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x830A
// 首条位置: {"module":589,"inlinee":33546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119304,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119352,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":589,"record_offset":119176,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":589,"record_offset":119396,"parent_record_offset":119352,"ancestor_record_offset":119148,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119204,"parent_record_offset":119176,"ancestor_record_offset":119148,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":118964,"rva":11841392,"va":5380550512}
// 编译实例 RVAs: 0xb4af70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2849647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e89184d4ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5380550954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2368378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e886dcdbff","size":5,"soft_fail":false,"target_va":5378182224,"va":5380550597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1964900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e89c04e2ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5380550863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11425542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e80657ae00","size":5,"soft_fail":false,"target_va":5391976336,"va":5380550789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11428589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e8ed62ae00","size":5,"soft_fail":false,"target_va":5391979408,"va":5380550814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31296538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e81a8cdd01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380550625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2373488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380550512,"raw_hex":"e890c8dbff","size":5,"soft_fail":false,"target_va":5378177424,"va":5380550907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119432,"parent_record_offset":118964,"ancestor_record_offset":118964,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
//     evidence={"module":589,"record_offset":119148,"parent_record_offset":118964,"ancestor_record_offset":118964,"inlinee":33546,"source_line":18,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":589,"record_offset":119396,"parent_record_offset":119352,"ancestor_record_offset":118964,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119204,"parent_record_offset":119176,"ancestor_record_offset":118964,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119352,"parent_record_offset":119148,"ancestor_record_offset":118964,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":589,"record_offset":119176,"parent_record_offset":119148,"ancestor_record_offset":118964,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":119464,"rva":11842000,"va":5380551120}
// 编译实例 RVAs: 0xb4b1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2850255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e83182d4ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5380551562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2368986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e826dadbff","size":5,"soft_fail":false,"target_va":5378182224,"va":5380551205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1965508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e83c02e2ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5380551471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11424934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e8a654ae00","size":5,"soft_fail":false,"target_va":5391976336,"va":5380551397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11427981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e88d60ae00","size":5,"soft_fail":false,"target_va":5391979408,"va":5380551422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: grass_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31295930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e8ba89dd01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380551233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2374830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380551120,"raw_hex":"e852c3dbff","size":5,"soft_fail":false,"target_va":5378176688,"va":5380551513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119936,"parent_record_offset":119464,"ancestor_record_offset":119464,"inlinee":28429,"source_line":332,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/grass_shader.rs
//     evidence={"module":589,"record_offset":119652,"parent_record_offset":119464,"ancestor_record_offset":119464,"inlinee":33546,"source_line":18,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":589,"record_offset":119900,"parent_record_offset":119856,"ancestor_record_offset":119464,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119708,"parent_record_offset":119680,"ancestor_record_offset":119464,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":589,"record_offset":119856,"parent_record_offset":119652,"ancestor_record_offset":119464,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":589,"record_offset":119680,"parent_record_offset":119652,"ancestor_record_offset":119464,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":589,"record_offset":119176,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119204,"parent_record_offset":119176,"ancestor_record_offset":119148,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":589,"record_offset":119304,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":589,"record_offset":119352,"parent_record_offset":119148,"ancestor_record_offset":119148,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119176,"parent_record_offset":119148,"ancestor_record_offset":118964,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119204,"parent_record_offset":119176,"ancestor_record_offset":118964,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119352,"parent_record_offset":119148,"ancestor_record_offset":118964,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119680,"parent_record_offset":119652,"ancestor_record_offset":119464,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119708,"parent_record_offset":119680,"ancestor_record_offset":119464,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_transitive","evidence":{"module":589,"record_offset":119856,"parent_record_offset":119652,"ancestor_record_offset":119464,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00589.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: grass_shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd> >,country_core::systems::paths::draw_path::PathRastersResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd> >", "country_core::systems::paths::draw_path::PathRastersResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_common
// 信息 ID: 718fbb87ef1ef72a1246c323
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader; module=589, inline_record=119148; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader; module=589, inline_record=119652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_lod1_shader
// 信息 ID: da94758645134583e8d2ff37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::grass_shader::draw_grass_shader
// 信息 ID: e0db7912e37d42129172364e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
