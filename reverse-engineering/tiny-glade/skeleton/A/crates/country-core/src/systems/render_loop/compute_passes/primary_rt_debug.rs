// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 22265adffb74f8e1225731de8771de4b59c6294f2c7b1367c2c59ea4b419a2df
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: primary_rt_debug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 412
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":12940,"rva":5735920,"va":5374445040}
// 编译实例 RVAs: 0x5785f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-231028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e88c79fcff","size":5,"soft_fail":false,"target_va":5374214416,"va":5374445439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-229601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e81f7ffcff","size":5,"soft_fail":false,"target_va":5374215680,"va":5374445276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t830608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e890ac0c00","size":5,"soft_fail":false,"target_va":5375275808,"va":5374445195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t869011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e893420d00","size":5,"soft_fail":false,"target_va":5375314864,"va":5374445848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1532620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8cc621700","size":5,"soft_fail":false,"target_va":5375978624,"va":5374445999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1794046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8fe5f1b00","size":5,"soft_fail":false,"target_va":5376239920,"va":5374445869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcab40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8725397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e895238500","size":5,"soft_fail":false,"target_va":5383170880,"va":5374445478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcaf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8726375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e867278500","size":5,"soft_fail":false,"target_va":5383171872,"va":5374445492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8739395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8435a8500","size":5,"soft_fail":false,"target_va":5383184544,"va":5374445144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8782827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8eb038600","size":5,"soft_fail":false,"target_va":5383228352,"va":5374445520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: primary_rt_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29148129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8e1c3bc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374445226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1896668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374445040,"raw_hex":"e8dcf01c00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374445087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":13416,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":21933,"source_line":452,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":13156,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":412,"record_offset":13204,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":412,"record_offset":13284,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":23612,"source_line":26,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":412,"record_offset":13128,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":13516,"parent_record_offset":13444,"ancestor_record_offset":12940,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":13444,"parent_record_offset":13416,"ancestor_record_offset":12940,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":412,"record_offset":13232,"parent_record_offset":13204,"ancestor_record_offset":12940,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00412.jsonl.gz"}
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
// 完整结构及原始类型记录: primary_rt_debug.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: primary_rt_debug.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: primary_rt_debug.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":13156,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":13204,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":412,"record_offset":13232,"parent_record_offset":13204,"ancestor_record_offset":12940,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_lighting_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":13284,"parent_record_offset":12940,"ancestor_record_offset":12940,"inlinee":23612,"source_line":26,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"f84343f3d3aac8cb243ca9a6","relative_path":"_primary_rt_debug.cs.hlsl_cs_main_0.bin","sha256":"2c49b1c537c3ae1569570957e2e1a88bd36514ef4d2af2c738c0282b53c330f4","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"primary_rt_debug","prefix_candidates":["primary_rt_debug"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _primary_rt_debug.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[5]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"frame_constants","storage_class":"Uniform","type":{"id":179,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,8]},"decorations":[],"pointee":{"id":8,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[75,75,75,75,75,75,75,75,43,42,42,43,59,43,42,51,42,51,43,51,43,75,32]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":75,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":75,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":75,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":75,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":43,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":42,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":42,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":43,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":59,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":43,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":42,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":42,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":51,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":43,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":51,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":43,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":75,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":32,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"$Globals","storage_class":"Uniform","type":{"id":180,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[43,59,59,59,43,43,59]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":43,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"terrain_heightmap_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"blue_noise_texture_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"terrain_lighting_texture_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"rt_bvh_node_count","type_id":43,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"rt_triangle_count","type_id":43,"decorations":[{"decoration":"Offset","operands":[36]}]},{"index":6,"name":"output_tex_dims","type_id":59,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"terrain_heightmap","storage_class":"UniformConstant","type":{"id":181,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":181,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"terrain_lighting_texture","storage_class":"UniformConstant","type":{"id":181,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,11]},"decorations":[],"pointee":{"id":11,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"color_library","storage_class":"Uniform","type":{"id":182,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75,75]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":75,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":75,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":75,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":75,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":75,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":75,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":75,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":75,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":75,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":75,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":75,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":75,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":75,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":75,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":75,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":75,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":75,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":75,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":75,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":20,"name":"wall_color_ids","storage_class":"Uniform","type":{"id":183,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.StructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[33]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":33,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"rt_triangle_meta","storage_class":"Uniform","type":{"id":183,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.StructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[33]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":33,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"rt_bl_bvh","storage_class":"Uniform","type":{"id":185,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,22]},"decorations":[],"pointee":{"id":22,"name":"type.StructuredBuffer.PackedBlBvhNode","record":{"opcode":"OpTypeStruct","operands":[35]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"rt_triangles","storage_class":"Uniform","type":{"id":186,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.StructuredBuffer.PackedTriangle","record":{"opcode":"OpTypeStruct","operands":[38]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"output_tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[42,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":191,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,190]},"decorations":[],"pointee":{"id":190,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[43,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":192,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3137068,"position_among_mapped_render_targets":2,"mapped_render_target_count":70,"previous_callsite":3135385,"next_callsite":3137946,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/reset_indirect_args.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/upload_world_raster.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: primary_rt_debug.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
// 信息 ID: c7d724339fafcd36d51064c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fde2c -> 0x5785f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
