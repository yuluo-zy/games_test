// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e7395dc5b46d23ee52b267e58f6e8d6067657ca2ae76f5e93b62a784ebf41718
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ssgi.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 462
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
// 函数符号: country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":29160,"rva":6530240,"va":5375239360}
// 编译实例 RVAs: 0x63a4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-723014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8baf7f4ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375239489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ac090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-585985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8ff0ef7ff","size":5,"soft_fail":false,"target_va":5374656656,"va":5375242636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405acbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-583459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8dd18f7ff","size":5,"soft_fail":false,"target_va":5374659536,"va":5375242990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e888870000","size":5,"soft_fail":false,"target_va":5375274368,"va":5375239667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8868c0000","size":5,"soft_fail":false,"target_va":5375275808,"va":5375239829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t74631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e887230100","size":5,"soft_fail":false,"target_va":5375314864,"va":5375240228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=22
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t734696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8e8350b00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375243923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t999659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8eb400f00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375240256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7944922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8da3a7900","size":5,"soft_fail":false,"target_va":5383184544,"va":5375239617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7985478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e846d97900","size":5,"soft_fail":false,"target_va":5383228352,"va":5375242869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28353664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e880a4b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375239691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2119608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8b8572000","size":5,"soft_fail":false,"target_va":5377364336,"va":5375244723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27736337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e81139a701","size":5,"soft_fail":false,"target_va":5402980128,"va":5375243786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t1102341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e805d21000","size":5,"soft_fail":false,"target_va":5376341760,"va":5375239414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1101072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e810cd1000","size":5,"soft_fail":false,"target_va":5376340512,"va":5375239435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1098112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e880c11000","size":5,"soft_fail":false,"target_va":5376340720,"va":5375242603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1098155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e8abc11000","size":5,"soft_fail":false,"target_va":5376340752,"va":5375242592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t980018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375239360,"raw_hex":"e832f40e00","size":5,"soft_fail":false,"target_va":5376219552,"va":5375239529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":462,"record_offset":31460,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":462,"record_offset":29600,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":462,"record_offset":29552,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":462,"record_offset":29456,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":462,"record_offset":29364,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":462,"record_offset":29336,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":462,"record_offset":29424,"parent_record_offset":29364,"ancestor_record_offset":29160,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":29700,"parent_record_offset":29632,"ancestor_record_offset":29160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":462,"record_offset":29632,"parent_record_offset":29600,"ancestor_record_offset":29160,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":462,"record_offset":29400,"parent_record_offset":29364,"ancestor_record_offset":29160,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00462.jsonl.gz"}
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
// 完整结构及原始类型记录: ssgi.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ssgi.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: ssgi.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":29456,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":29552,"parent_record_offset":29160,"ancestor_record_offset":29160,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"46a0c5e485c7e379ce0ea6c2","relative_path":"_ssgi_spatial_filter.cs.hlsl_cs_main_0.bin","sha256":"4018b222e8eb7ffe74971e8c8bfa497c6054e3e80f9e17b4e8222d953435d6b8","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"55424fa62878e4ac1c03a1d7","relative_path":"_ssgi_ssgi.cs.hlsl_cs_main_0.bin","sha256":"23a79c4da915d16f930bba6fbef62205ed63b6aff72dfb8b8ac1be7efcf2c40c","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c87d183ad492785a5526f258","relative_path":"_ssgi_temporal_filter.cs.hlsl_cs_main_0.bin","sha256":"2f63abefad7e64739dba72bdf0531dac6fc1083ddf7aa4ea7d35f1fcb2621286","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"818ff868942524c061c399c9","relative_path":"_ssgi_upsample.cs.hlsl_cs_main_0.bin","sha256":"cf0857256ea886aa0e8cf3e428a7d642896d7b392e7f786f38b807668372a0aa","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_spatial_filter.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"ssgi_tex","storage_class":"UniformConstant","type":{"id":13,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"output_tex","storage_class":"UniformConstant","type":{"id":14,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":16,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,15]},"decorations":[],"pointee":{"id":15,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[9,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_ssgi.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"frame_constants","storage_class":"Uniform","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[61,61,61,61,61,61,61,61,22,21,21,22,67,22,21,45,21,45,22,45,22,61,17]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":61,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":61,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":61,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":22,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":21,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":21,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":22,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":67,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":22,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":21,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":45,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":21,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":45,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":22,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":45,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":22,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":61,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":17,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"$Globals","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[67,67,67]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":67,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"half_view_normal_tex_dims","type_id":67,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"output_tex_dims","type_id":67,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"half_view_normal_tex","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"output_tex","storage_class":"UniformConstant","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,14]},"decorations":[],"pointee":{"id":14,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,74]},"decorations":[],"pointee":{"id":74,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[22,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_upsample.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"frame_constants","storage_class":"Uniform","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4]},"decorations":[],"pointee":{"id":4,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[26,26,26,26,26,26,26,26,21,20,20,21,32,21,20,39,20,39,21,39,21,26,17]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":26,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":26,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":26,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":26,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":26,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":26,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":26,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":21,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":20,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":20,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":21,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":32,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":21,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":20,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":20,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":39,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":21,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":39,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":21,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":26,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":17,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"ssgi_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"half_view_normal_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"gbuffer_tex","storage_class":"UniformConstant","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[21,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"output_tex","storage_class":"UniformConstant","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,14]},"decorations":[],"pointee":{"id":14,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":50,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[21,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_temporal_filter.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"frame_constants","storage_class":"Uniform","type":{"id":44,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[30,30,30,30,30,30,30,30,24,23,23,24,40,24,23,41,23,41,24,41,24,30,20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":30,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":30,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":30,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":24,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":23,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":23,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":24,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":40,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":24,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":23,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":41,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":23,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":41,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":24,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":41,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":24,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":30,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":20,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"$Globals","storage_class":"Uniform","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[24,40,40,40,40,40]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":24,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"input_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"history_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"reprojection_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"final_output_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"history_output_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"input_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"history_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"final_output_tex","storage_class":"UniformConstant","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"history_output_tex","storage_class":"UniformConstant","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,17]},"decorations":[],"pointee":{"id":17,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":50,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139539,"position_among_mapped_render_targets":40,"mapped_render_target_count":70,"previous_callsite":3139468,"next_callsite":3139966,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ssgi.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
// 信息 ID: d568255ab1c7b2687d88d91e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe7d3 -> 0x63a4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
