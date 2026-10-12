// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 767596e384ee768f9630b4f0d8ac6a70b6a3bb5cd4f34945fb655acac15b97b0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: path_pebbles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 514
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":514,"record_offset":16500,"rva":9801584,"va":5378510704}
// 编译实例 RVAs: 0x958f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-795711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e8c1dbf3ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378511850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-343712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e860c1faff","size":5,"soft_fail":false,"target_va":5378167392,"va":5378511099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t75073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e841250100","size":5,"soft_fail":false,"target_va":5378585968,"va":5378510890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t541025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e861410800","size":5,"soft_fail":false,"target_va":5379052752,"va":5378511722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2658604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e82c912800","size":5,"soft_fail":false,"target_va":5381170352,"va":5378511743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13279965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e8dda2ca00","size":5,"soft_fail":false,"target_va":5391790752,"va":5378510782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416058b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13289681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e8d1c8ca00","size":5,"soft_fail":false,"target_va":5391800496,"va":5378510810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13302802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e812fcca00","size":5,"soft_fail":false,"target_va":5391813648,"va":5378510841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33336251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378510704,"raw_hex":"e8bbabfc01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378510912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":17224,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_draw_indirect_cmd_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":16688,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":28012,"source_line":394,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":16948,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":16796,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":514,"record_offset":16996,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":514,"record_offset":16760,"parent_record_offset":16716,"ancestor_record_offset":16500,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":514,"record_offset":17324,"parent_record_offset":17252,"ancestor_record_offset":16500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":17252,"parent_record_offset":17224,"ancestor_record_offset":16500,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":17028,"parent_record_offset":16996,"ancestor_record_offset":16500,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":514,"record_offset":16716,"parent_record_offset":16688,"ancestor_record_offset":16500,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00514.jsonl.gz"}
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
// 完整结构及原始类型记录: path_pebbles.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: path_pebbles.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: path_pebbles.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":514,"record_offset":16716,"parent_record_offset":16688,"ancestor_record_offset":16500,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00514.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":514,"record_offset":16796,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00514.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":514,"record_offset":16948,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00514.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":514,"record_offset":16996,"parent_record_offset":16500,"ancestor_record_offset":16500,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00514.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":514,"record_offset":17028,"parent_record_offset":16996,"ancestor_record_offset":16500,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00514.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"5fd63d43e59ed936f1f90d69","relative_path":"_path_pebbles.cs.hlsl_cs_main_0.bin","sha256":"a054ae8e00efbde175f43da7629b9a309bbfad760e4f8a9b44253a6ba034b45d","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"path_pebbles","prefix_candidates":["path_pebbles"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _path_pebbles.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,27]},"decorations":[],"pointee":{"id":27,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,27]},"decorations":[],"pointee":{"id":27,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"__alias_draw_commands","storage_class":"Uniform","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.RWStructuredBuffer.DrawIndexedIndirectCommand","record":{"opcode":"OpTypeStruct","operands":[44]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":44,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"$Globals","storage_class":"Uniform","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,34]},"decorations":[],"pointee":{"id":34,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[47,83,83,83,83,83,53,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":47,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"terrain_heightmap_dims","type_id":83,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"visual_terrain_heightmap_dims","type_id":83,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"path_mask_dims","type_id":83,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"rocky_terrain_dims","type_id":83,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"water_mask_dims","type_id":83,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"pebble_buffer_size","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"scatter_grid_dim","type_id":47,"decorations":[{"decoration":"Offset","operands":[52]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"terrain_heightmap","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"path_mask","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"rocky_terrain","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"water_mask","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"__alias_pebble_placement_buffer","storage_class":"Uniform","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,40]},"decorations":[],"pointee":{"id":40,"name":"type.RWStructuredBuffer.Affine3Packed","record":{"opcode":"OpTypeStruct","operands":[46]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":46,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,90]},"decorations":[],"pointee":{"id":90,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161883,"position_among_mapped_render_targets":22,"mapped_render_target_count":59,"previous_callsite":5161872,"next_callsite":5161894,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: path_pebbles.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
// 信息 ID: 55f749e1ebf4c0bd58cb5f69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec39b -> 0x958f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
