// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 05da2fcf7ba6cc4313da14548ab1cae35db0fa6af7c35d4b5423ae4aaa550a75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: blur_pyramid.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 469
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
// 函数符号: country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":25228,"rva":6640640,"va":5375349760}
// 编译实例 RVAs: 0x655400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-833550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8f247f3ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375350025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-75884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e894d7feff","size":5,"soft_fail":false,"target_va":5375274368,"va":5375350247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-74650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e866dcfeff","size":5,"soft_fail":false,"target_va":5375275808,"va":5375350453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e83873ffff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375350899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t626433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8018f0900","size":5,"soft_fail":false,"target_va":5375978624,"va":5375352186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t888988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e89c900d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375350927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7834336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8e08a7700","size":5,"soft_fail":false,"target_va":5383184544,"va":5375350203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8d50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7878201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e839367800","size":5,"soft_fail":false,"target_va":5383228752,"va":5375350546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: blur_pyramid.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28243083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e88bf4ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375350272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::all_mip_levels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t991097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8791f0f00","size":5,"soft_fail":false,"target_va":5376340992,"va":5375349890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_extent
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t989970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8121b0f00","size":5,"soft_fail":false,"target_va":5376341504,"va":5375351529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t991251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e813200f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375350504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t991000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8181f0f00","size":5,"soft_fail":false,"target_va":5376340880,"va":5375349875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t991808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e840220f00","size":5,"soft_fail":false,"target_va":5376341664,"va":5375349851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_mip_level
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t991774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e81e220f00","size":5,"soft_fail":false,"target_va":5376342096,"va":5375350317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t991800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e838220f00","size":5,"soft_fail":false,"target_va":5376342160,"va":5375350355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t991480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e8f8200f00","size":5,"soft_fail":false,"target_va":5376341776,"va":5375350291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t869482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375349760,"raw_hex":"e86a440d00","size":5,"soft_fail":false,"target_va":5376219552,"va":5375350065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":469,"record_offset":25964,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$6::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":25416,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":26694,"source_line":2074,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":25632,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":25592,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":25544,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":26048,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":25800,"source_line":702,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":469,"record_offset":25444,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":25228,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":469,"record_offset":25512,"parent_record_offset":25444,"ancestor_record_offset":25228,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":469,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":25228,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":25732,"parent_record_offset":25664,"ancestor_record_offset":25228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":469,"record_offset":25664,"parent_record_offset":25632,"ancestor_record_offset":25228,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":469,"record_offset":25484,"parent_record_offset":25444,"ancestor_record_offset":25228,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
// 记录形式: inlinee_declaration; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6843
// 首条位置: {"module":469,"inlinee":26691}
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
// 完整结构及原始类型记录: blur_pyramid.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: blur_pyramid.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: blur_pyramid.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":469,"record_offset":25544,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00469.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::read_write_texture","relationship":"pdb_inline_direct","evidence":{"module":469,"record_offset":25592,"parent_record_offset":25228,"ancestor_record_offset":25228,"inlinee":8419,"source_line":153,"archive":"_pdb/A/modules/00469.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"90266bbb527b84207456c9a5","relative_path":"_blur_pyramid_blur.cs.hlsl_cs_main_0.bin","sha256":"a675908090fc5633925cd10bb35fb7d931e4e60e12aa418d169e51ba26d1fec8","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"blur_pyramid","prefix_candidates":["blur_pyramid"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"9b4da555998a6a9b95aaa01a","relative_path":"_blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin","sha256":"2511305afd906cf6e0c05a1e7c9a5756d566ab44b24aea7a870fd0e153dc7a07","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"blur_pyramid","prefix_candidates":["blur_pyramid"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _blur_pyramid_blur.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"input_tex","storage_class":"UniformConstant","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[13,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"output_tex","storage_class":"UniformConstant","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[13,1,2,0,0,2,1]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"$Globals","storage_class":"Uniform","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[27]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_mip_dims","type_id":27,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[14,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[14,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,38]},"decorations":[],"pointee":{"id":38,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[14,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":28,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"input_tail_tex","storage_class":"UniformConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"input_tex","storage_class":"UniformConstant","type":{"id":29,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"output_tex","storage_class":"UniformConstant","type":{"id":30,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,9]},"decorations":[],"pointee":{"id":9,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,2,1]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"$Globals","storage_class":"Uniform","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,11]},"decorations":[],"pointee":{"id":11,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[31,14]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"output_extent","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"self_weight","type_id":14,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,33]},"decorations":[],"pointee":{"id":33,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[15,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":6542864,"caller_aliases":{"alias_group_rva":6542864,"count":1,"display_examples":["country_core::systems::render_loop::vermeer::vermeer_render_from_target"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6542983,"position_among_mapped_render_targets":1,"mapped_render_target_count":2,"previous_callsite":null,"next_callsite":6543347,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6542864,"caller_aliases":{"alias_group_rva":6542864,"count":1,"display_examples":["country_core::systems::render_loop::vermeer::vermeer_render_from_target"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6543347,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":6542983,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: blur_pyramid.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::blur_pyramid::blur_pyramid
// 信息 ID: bc87f6d2322b036cbfd9fcaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x63d687 -> 0x655400 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x63d7f3 -> 0x655400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::blur_pyramid::rev_blur_pyramid
// 信息 ID: 879f9866c2cbdffa11c64d56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
