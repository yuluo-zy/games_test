// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/half_res.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5bc1cba1929314d8052077bee2c9498dc43fd5c1bb6f1e2624474574fc78694b
// 前部保存 PDB 元数据；末尾 recovered_c 保存 2026-10-12 原指令/SPIR-V 驱动的候选实现。
// 候选状态 implemented_unverified；原作者文本、完整工程接入和 GPU 等价性仍未恢复/验收。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_res.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 556
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":74840,"rva":11457344,"va":5380166464}
// 编译实例 RVAs: 0xaed340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3306108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e87c723200","size":5,"soft_fail":false,"target_va":5383472768,"va":5380166655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13121792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e80039c800","size":5,"soft_fail":false,"target_va":5393288768,"va":5380166971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14178f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13246749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e81d21ca00","size":5,"soft_fail":false,"target_va":5393413840,"va":5380167086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3381535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e81f993300","size":5,"soft_fail":false,"target_va":5383548144,"va":5380166604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3369526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e8366a3300","size":5,"soft_fail":false,"target_va":5383536672,"va":5380167141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3372143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e86f743300","size":5,"soft_fail":false,"target_va":5383539152,"va":5380167004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::TexView::with_stencil_aspect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3357824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380166464,"raw_hex":"e8803c3300","size":5,"soft_fail":false,"target_va":5383524688,"va":5380166859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":75264,"parent_record_offset":74840,"ancestor_record_offset":74840,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":75120,"parent_record_offset":74840,"ancestor_record_offset":74840,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":556,"record_offset":75332,"parent_record_offset":74840,"ancestor_record_offset":74840,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::with_view
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":75200,"parent_record_offset":74840,"ancestor_record_offset":74840,"inlinee":30194,"source_line":696,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::div_ceil_dims
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":75032,"parent_record_offset":74840,"ancestor_record_offset":74840,"inlinee":14911,"source_line":379,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":75088,"parent_record_offset":75032,"ancestor_record_offset":74840,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":556,"record_offset":75056,"parent_record_offset":75032,"ancestor_record_offset":74840,"inlinee":14912,"source_line":3657,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::extract_half_res_depth_normals::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":556,"record_offset":75300,"parent_record_offset":75264,"ancestor_record_offset":74840,"inlinee":34782,"source_line":6739,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$59::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":75232,"parent_record_offset":75200,"ancestor_record_offset":74840,"inlinee":15346,"source_line":182,"archive":"_pdb/C/modules/00556.jsonl.gz"}
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
// 完整结构及原始类型记录: half_res.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: half_res.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/half_res.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: half_res.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// [PDB Shader 类型/入口] {"shader_family":"country_core/extract_half_res_depth_normals","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::extract_half_res_depth_normals::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":75300,"parent_record_offset":75264,"ancestor_record_offset":74840,"inlinee":34782,"source_line":6739,"archive":"_pdb/C/modules/00556.jsonl.gz"}}
// [Shader 映射] {"shader_id":"0318005a4c0e8f7a980a103f","relative_path":"country-core/compiled/extract_half_res_depth_normals.precomp","sha256":"4e36b589039aeaaf9b7fa9761c004561a3e61b859956be51546fcb23471df455","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals","hint":{"shader_family":"country_core/extract_half_res_depth_normals","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::extract_half_res_depth_normals::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":556,"record_offset":75300,"parent_record_offset":75264,"ancestor_record_offset":74840,"inlinee":34782,"source_line":6739,"archive":"_pdb/C/modules/00556.jsonl.gz"}},"shader_uri":"country_core/extract_half_res_depth_normals:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/extract_half_res_depth_normals.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[28]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"frame_counter","storage_class":"Uniform","type":{"id":51,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,50]},"decorations":[],"pointee":{"id":50,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[24]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":24,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":78,"name":"depth_input_tex","storage_class":"UniformConstant","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,75]},"decorations":[],"pointee":{"id":75,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":85,"name":"stencil_tex","storage_class":"UniformConstant","type":{"id":84,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,82]},"decorations":[],"pointee":{"id":82,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[24,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":120,"name":"depth_output_tex","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,117]},"decorations":[],"pointee":{"id":117,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,3]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":131,"name":"gbuffer_input_tex","storage_class":"UniformConstant","type":{"id":130,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,128]},"decorations":[],"pointee":{"id":128,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[24,1,2,0,0,1,35]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":188,"name":"current_view","storage_class":"Uniform","type":{"id":187,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,184]},"decorations":[],"pointee":{"id":184,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[185,185,185,185,185,185,185,185,185,185,185,185,185,186,186,186,186,185,185,79,8,8,8,8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":185,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":185,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":185,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":185,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":185,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":185,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":185,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":185,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":185,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":185,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":185,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":185,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":185,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":186,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":186,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":186,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":186,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":185,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":185,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":79,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":8,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":8,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":8,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":8,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":222,"name":"normals_output_tex","storage_class":"UniformConstant","type":{"id":221,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,219]},"decorations":[],"pointee":{"id":219,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[8,1,2,0,0,2,5]},"decorations":[]}},"descriptor_set":0,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"unknown","storage_class":"Input","type":{"id":27,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5686412,"position_among_mapped_render_targets":52,"mapped_render_target_count":74,"previous_callsite":5686383,"next_callsite":5686501,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/grass_velocity_reconstruct.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: half_res.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 信息 ID: 66201204392cc02925029ba2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c48c -> 0xaed340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// === NATIVE RECOVERY C 2026-10-12 BEGIN ===
// 原函数 RVA 0xaed340..0xaed629；调用方 render_world 的 CALL RVA 0x56c48c。
// 证据与缺口见同目录 half_res.rs.recovery.json / half_res.recovery.md。
// 下面采用语义 Rust 接口，不宣称恢复了原作者签名或原生 repr(C) 布局。
// A/B 的指令和依赖不同，不能把此 C 实现直接标记为三版恢复完成。
pub mod recovered_c {
    use glam::{Mat4, Vec3, Vec4}; // C 原锁 glam 0.30.10；复用数学库。

    pub const SHADER_URI: &str = "country_core/extract_half_res_depth_normals:main";
    pub const LOCAL_SIZE: [u32; 3] = [8, 8, 1];

    /// get_compute 收到的 24 字节存储快照；内部字段名/容器 ABI 未裁定。
    /// 适配器必须使用匹配 rhapsody 的 ShaderPermutationKey，不能把这三个字当作三个宏。
    pub const OBSERVED_KEY_STORAGE_WORDS: [u64; 3] = [0, 8, 0];

    /// 只保存 create_raw_resource 调用前确实写入的 15 字节和标签。
    /// 未恢复的 byte15/padding、Rust 字段名和依赖默认构造器不在此结构中补零。
    /// 此结构是恢复接口，绝不是原生 TextureDesc 的内存镜像。
    #[derive(Clone, Copy, Debug)]
    pub struct ObservedTextureCreate {
        pub written_prefix: [u8; 15],
        pub debug_name: &'static str,
    }

    impl ObservedTextureCreate {
        fn new(dims: [u16; 2], constants: [u8; 8], debug_name: &'static str) -> Self {
            let mut written_prefix = [0u8; 15];
            written_prefix[2..4].copy_from_slice(&dims[0].to_le_bytes());
            written_prefix[4..6].copy_from_slice(&dims[1].to_le_bytes());
            written_prefix[6..14].copy_from_slice(&constants);
            Self {
                written_prefix,
                debug_name,
            }
        }

        pub fn dimensions(&self) -> [u16; 2] {
            [
                u16::from_le_bytes([self.written_prefix[2], self.written_prefix[3]]),
                u16::from_le_bytes([self.written_prefix[4], self.written_prefix[5]]),
            ]
        }

        /// byte11 = 已观察 Vulkan format 值：depth 100/R32_SFLOAT，normals 38/R8G8B8A8_SNORM。
        pub fn observed_vk_format(&self) -> u8 {
            self.written_prefix[11]
        }
    }

    pub fn half_dimensions(depth_dimensions: [u16; 2]) -> [u16; 2] {
        // 0xaed364..0xaed387；PDB div_ceil_dims / div_ceil / Ord::max。
        depth_dimensions.map(|n| n.div_ceil(2).max(1))
    }

    /// 字段顺序对应 native bind_entry_point 的五个指针；不是 GPU binding 顺序。
    pub struct EntryBindings<'a, Texture> {
        pub stencil: &'a Texture, // set0 binding1 read；与输入 depth 同 resource ID
        pub normals_output: &'a Texture, // set0 binding5 write
        pub depth_input: &'a Texture, // set0 binding2 read
        pub depth_output: &'a Texture, // set0 binding4 write
        pub gbuffer_input: &'a Texture, // set0 binding3 read
    }

    pub struct HalfResOutputs<Texture> {
        pub depth: Texture,   // native sret bytes0..23；R32_SFLOAT
        pub normals: Texture, // native sret bytes24..47；RGBA8_SNORM，alpha 保存采样子位置
    }

    /// 有意保留依赖接入边界：匹配私有 rhapsody commit 的真实适配器尚未恢复。
    /// 不提供内存假 Graph/假 handle 实现，也不把成功调用 trait 当作 GPU 运行证明。
    /// 错误、panic/资源失败策略由原依赖承担；此处不添加恢复/吞错分支。
    pub trait RhapsodyHalfResApi {
        type Texture;
        type Shader;
        type ComputeNode;
        type BoundComputeNode;

        fn texture_dimensions(&self, texture: &Self::Texture) -> [u16; 2];
        fn create_texture(&mut self, desc: ObservedTextureCreate) -> Self::Texture;
        /// GraphApi+0x24c 原 byte；每个 create 后分别检查 ==1。
        fn debug_garbage_mode_byte(&self) -> u8;
        fn init_as_debug_garbage(&mut self, texture: &Self::Texture);
        /// 必须保留 depth resource ID 和 descriptor；view 从 TexView(0) 加 stencil aspect。
        /// 原 with_stencil_aspect(0) 返回 0x800000，不是在原输入 view 上 OR。
        fn stencil_view_from_default(&mut self, depth: &Self::Texture) -> Self::Texture;
        fn get_compute_with_observed_key(
            &mut self,
            uri: &'static str,
            key_storage_words: [u64; 3],
        ) -> Self::Shader;
        fn new_compute(&mut self, name: &'static str, shader: Self::Shader) -> Self::ComputeNode;
        /// 保留真实依赖的 descriptor/graph read-write 注册及继承的全局 binding sets。
        /// 还需现有 set2/binding0 FrameCounter 与 set4/binding0 CurrentView；包装层不生产它们。
        fn bind_entry_point(
            &mut self,
            node: Self::ComputeNode,
            textures: EntryBindings<'_, Self::Texture>,
        ) -> Self::BoundComputeNode;
        /// 这里传线程数；8x8x1 工作组取整和 Vulkan dispatch 由原依赖实现。
        fn dispatch_threads(&mut self, node: Self::BoundComputeNode, threads: [u32; 3]);
    }

    /// 从 C 的完整 745 字节包装函数恢复资源创建、视图、绑定、dispatch 和返回顺序。
    /// graph 的真实 FrameCounter/View 输入必须由调用者既有 binding-set 链提供。
    pub fn extract_half_res_depth_normals<Api: RhapsodyHalfResApi>(
        graph: &mut Api,
        gbuffer: &Api::Texture,
        depth: &Api::Texture,
    ) -> HalfResOutputs<Api::Texture> {
        let dims = half_dimensions(graph.texture_dimensions(depth));
        let depth_output = graph.create_texture(ObservedTextureCreate::new(
            dims,
            [1, 0, 1, 0, 1, 100, 1, 2],
            "half-res depth",
        ));
        if graph.debug_garbage_mode_byte() == 1 {
            graph.init_as_debug_garbage(&depth_output);
        }
        let normals_output = graph.create_texture(ObservedTextureCreate::new(
            dims,
            [1, 0, 1, 0, 1, 38, 1, 2],
            "half-res normals",
        ));
        if graph.debug_garbage_mode_byte() == 1 {
            graph.init_as_debug_garbage(&normals_output);
        }
        let stencil = graph.stencil_view_from_default(depth);
        let shader = graph.get_compute_with_observed_key(SHADER_URI, OBSERVED_KEY_STORAGE_WORDS);
        let node = graph.new_compute(SHADER_URI, shader);
        let bound = graph.bind_entry_point(
            node,
            EntryBindings {
                stencil: &stencil,
                normals_output: &normals_output,
                depth_input: depth,
                depth_output: &depth_output,
                gbuffer_input: gbuffer,
            },
        );
        graph.dispatch_threads(bound, [u32::from(dims[0]), u32::from(dims[1]), 1]);
        HalfResOutputs {
            depth: depth_output,
            normals: normals_output,
        }
    }

    /// GPU 输入仅投影本 Pass 确实使用的字段，不补造整个 GameViews/FrameCounter owner。
    /// world_to_view 是逻辑矩阵；原 uniform member4 byte256 为 RowMajor/stride16。
    pub struct UsedShaderParameters {
        pub frame_index: u32,
        pub world_to_view: Mat4,
    }

    impl UsedShaderParameters {
        /// 参数来源投影：C RenderTemporalState+0x108 的 u32 与 GameViews 首1024字节。
        /// 原 lookup 的完整128-bit TypeId 与对应 get_res<T> 实例匹配；字段原名未知。
        /// 只读取已有资源数据，不重新实现 Bevy query/状态 owner 或递增规则。
        /// 短数据的 None 是恢复接口输入诊断，不是声称原作也有此错误分支。
        pub fn from_resource_payloads(
            render_temporal_state: &[u8],
            game_views: &[u8],
        ) -> Option<Self> {
            let frame: &[u8; 4] = render_temporal_state.get(0x108..0x10c)?.try_into().ok()?;
            let view: &[u8; 1024] = game_views.get(..1024)?.try_into().ok()?;
            Some(Self::from_uploaded_blocks(frame, view))
        }

        /// 读取真实上传的两个 uniform block；不产生/默认填充任何游戏状态。
        /// 原 Shader 的 frame member0 offset0，view member4 offset256/RowMajor/stride16。
        pub fn from_uploaded_blocks(frame: &[u8; 4], view: &[u8; 1024]) -> Self {
            let mut rows = [[0.0; 4]; 4];
            for (row, values) in rows.iter_mut().enumerate() {
                for (col, value) in values.iter_mut().enumerate() {
                    let at = 256 + row * 16 + col * 4;
                    *value = f32::from_le_bytes(view[at..at + 4].try_into().unwrap());
                }
            }
            Self {
                frame_index: u32::from_le_bytes(*frame),
                world_to_view: Mat4::from_cols_array_2d(&rows).transpose(),
            }
        }
    }

    pub fn source_pixel(output_pixel: [u32; 2], frame_index: u32) -> [u32; 2] {
        let offset = [[0, 0], [0, 1], [1, 1], [1, 0]][(frame_index & 3) as usize];
        [
            output_pixel[0].wrapping_mul(2).wrapping_add(offset[0]),
            output_pixel[1].wrapping_mul(2).wrapping_add(offset[1]),
        ]
    }

    /// SPIR-V 的纹理写入前值。GPU Rgba8Snorm 的量化不在该 CPU 投影中伪造。
    pub struct ShaderWriteValues {
        pub depth: f32,
        pub normals: Vec4,
    }

    /// 给定真实选中像素的三个输入，恢复 SPIR-V 的位操作与数学组合。
    /// 不是替代 GPU/texelFetch 的软件渲染器，未宣称 CPU/GPU 浮点逐位相同。
    pub fn shader_write_values(
        params: &UsedShaderParameters,
        sampled_depth: f32,
        sampled_stencil: u32,
        sampled_gbuffer_x: u32,
    ) -> ShaderWriteValues {
        let material_id = sampled_stencil & 7;
        let depth_bits = if material_id == 1 || material_id == 2 {
            sampled_depth.to_bits() | 1
        } else {
            sampled_depth.to_bits() & 0xffff_fffe
        };
        let scale = f32::from_bits(998_277_249); // 原 OpConstant，不替换为近似十进制。
        let normal01 = Vec3::new(
            (sampled_gbuffer_x & 255) as f32 * scale,
            ((sampled_gbuffer_x >> 8) & 255) as f32 * scale,
            ((sampled_gbuffer_x >> 16) & 255) as f32 * scale,
        );
        let normal = normal01 * 2.0 - Vec3::ONE;
        // OpVectorTimesMatrix：行向量 normal4 * world_to_view，不是 matrix * normal4。
        let view_normal = (params.world_to_view.transpose() * normal.extend(0.0))
            .truncate()
            .normalize();
        let offset = [[0, 0], [0, 1], [1, 1], [1, 0]][(params.frame_index & 3) as usize];
        let alpha =
            (if offset[0] == 1 { 1.0 } else { 0.5 }) * (if offset[1] == 1 { 1.0 } else { -1.0 });
        ShaderWriteValues {
            depth: f32::from_bits(depth_bits),
            normals: view_normal.extend(alpha),
        }
    }
}
// === NATIVE RECOVERY C 2026-10-12 END ===
