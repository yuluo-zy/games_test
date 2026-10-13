// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/path_pebbles.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f737f062b63828421b4704f9c54b290570eec966242df6d05aa2052acd6bb0dc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: path_pebbles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 414
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":414,"record_offset":42928,"rva":5772160,"va":5374481280}
// 编译实例 RVAs: 0x581380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-265874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e86ef1fbff","size":5,"soft_fail":false,"target_va":5374215680,"va":5374481549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t832765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e8fdb40c00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374482094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1496385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e841d51600","size":5,"soft_fail":false,"target_va":5375978624,"va":5374482234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1757800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e868d21a00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374482115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8703181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e8cdcc8400","size":5,"soft_fail":false,"target_va":5383184544,"va":5374481358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd4dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8731089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e8d1398500","size":5,"soft_fail":false,"target_va":5383212480,"va":5374481386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd5540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8732982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e836418500","size":5,"soft_fail":false,"target_va":5383214400,"va":5374481413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8746903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e897778500","size":5,"soft_fail":false,"target_va":5383228352,"va":5374481444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: path_pebbles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29111789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374481280,"raw_hex":"e8ed35bc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374481566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":414,"record_offset":43364,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_draw_indirect_cmd_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":414,"record_offset":43112,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":22663,"source_line":386,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":414,"record_offset":43140,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":414,"record_offset":43188,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":414,"record_offset":43464,"parent_record_offset":43392,"ancestor_record_offset":42928,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":414,"record_offset":43392,"parent_record_offset":43364,"ancestor_record_offset":42928,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00414.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":414,"record_offset":43216,"parent_record_offset":43188,"ancestor_record_offset":42928,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00414.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: path_pebbles.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":414,"record_offset":43140,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00414.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":414,"record_offset":43188,"parent_record_offset":42928,"ancestor_record_offset":42928,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00414.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":414,"record_offset":43216,"parent_record_offset":43188,"ancestor_record_offset":42928,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00414.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"93fc1c271dd7a884d151a972","relative_path":"_path_pebbles.cs.hlsl_cs_main_0.bin","sha256":"72883dc9cf276fc53325b790840513e929eb89e81507e88828bcad4ba98368fd","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"path_pebbles","prefix_candidates":["path_pebbles"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _path_pebbles.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"draw_commands","storage_class":"Uniform","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,8]},"decorations":[],"pointee":{"id":8,"name":"type.RWStructuredBuffer.DrawIndexedIndirectCommand","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"$Globals","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[23,58,58,58,58,29,23]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":23,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"terrain_heightmap_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"path_mask_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"rocky_terrain_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"water_mask_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"pebble_buffer_size","type_id":29,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"scatter_grid_dim","type_id":23,"decorations":[{"decoration":"Offset","operands":[44]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"terrain_heightmap","storage_class":"UniformConstant","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"path_mask","storage_class":"UniformConstant","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"rocky_terrain","storage_class":"UniformConstant","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"water_mask","storage_class":"UniformConstant","type":{"id":62,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"pebble_placement_buffer","storage_class":"Uniform","type":{"id":63,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.RWStructuredBuffer.PebblePlacement","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":65,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[29,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138304,"position_among_mapped_render_targets":19,"mapped_render_target_count":70,"previous_callsite":3138282,"next_callsite":3138326,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: path_pebbles.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
// 信息 ID: b983f145e0e9f1d47d464118
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe300 -> 0x581380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
