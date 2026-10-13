// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 190d7183b5d9023a6abac0f7d2f1700791773fce0e39f24e41409cdd5f11cf82
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: prefix_scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 518
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
// 函数符号: country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":518,"record_offset":22724,"rva":10206224,"va":5378915344}
// 编译实例 RVAs: 0x9bbc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: prefix_scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14373303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8b751db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378915460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417792d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: prefix_scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14406701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e82dd4db00","size":5,"soft_fail":false,"target_va":5393322704,"va":5378915998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177a840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: prefix_scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14412364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e84ceadb00","size":5,"soft_fail":false,"target_va":5393328192,"va":5378915823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177da70
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: prefix_scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14425520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8b01ddc00","size":5,"soft_fail":false,"target_va":5393341040,"va":5378915515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37705114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e89a553f02","size":5,"soft_fail":false,"target_va":5416621188,"va":5378916069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4632486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8a6af4600","size":5,"soft_fail":false,"target_va":5383548144,"va":5378915653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4620787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8f3814600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378915880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_indirect
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4623234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8828b4600","size":5,"soft_fail":false,"target_va":5383538768,"va":5378915529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4623654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e8268d4600","size":5,"soft_fail":false,"target_va":5383539152,"va":5378915493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::BufferDesc::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4608579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378915344,"raw_hex":"e843524600","size":5,"soft_fail":false,"target_va":5383524160,"va":5378915576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":518,"record_offset":23000,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":518,"record_offset":23156,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::next_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":518,"record_offset":23028,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":30261,"source_line":3691,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":22932,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":518,"record_offset":23056,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":23188,"parent_record_offset":22724,"ancestor_record_offset":22724,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::prefix_scan::inclusive_prefix_scan::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":22968,"parent_record_offset":22932,"ancestor_record_offset":22724,"inlinee":30339,"source_line":31142,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::prefix_scan::inclusive_prefix_scan_merge::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":23252,"parent_record_offset":23216,"ancestor_record_offset":22724,"inlinee":30343,"source_line":31177,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":23124,"parent_record_offset":23088,"ancestor_record_offset":22724,"inlinee":30341,"source_line":31222,"archive":"_pdb/C/modules/00518.jsonl.gz"}
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
// 完整结构及原始类型记录: prefix_scan.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: prefix_scan.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: prefix_scan.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect
// [PDB Shader 类型/入口] {"shader_family":"country_core/prefix_scan/inclusive_prefix_scan","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":22968,"parent_record_offset":22932,"ancestor_record_offset":22724,"inlinee":30339,"source_line":31142,"archive":"_pdb/C/modules/00518.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/prefix_scan/inclusive_prefix_scan_subtotals","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":23124,"parent_record_offset":23088,"ancestor_record_offset":22724,"inlinee":30341,"source_line":31222,"archive":"_pdb/C/modules/00518.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/prefix_scan/inclusive_prefix_scan_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":23252,"parent_record_offset":23216,"ancestor_record_offset":22724,"inlinee":30343,"source_line":31177,"archive":"_pdb/C/modules/00518.jsonl.gz"}}
// [Shader 映射] {"shader_id":"4932e88cae08d47f7ce5fb43","relative_path":"country-core/compiled/prefix_scan/inclusive_prefix_scan.precomp","sha256":"1056aee87f481f63953779e31e09f94efda98def4f4fcf299a64dfd41de7b099","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect","hint":{"shader_family":"country_core/prefix_scan/inclusive_prefix_scan","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":22968,"parent_record_offset":22932,"ancestor_record_offset":22724,"inlinee":30339,"source_line":31142,"archive":"_pdb/C/modules/00518.jsonl.gz"}},"shader_uri":"country_core/prefix_scan/inclusive_prefix_scan:main"}
// [Shader 映射] {"shader_id":"b828534e348e219025dfff0c","relative_path":"country-core/compiled/prefix_scan/inclusive_prefix_scan_subtotals.precomp","sha256":"00b8dd8ee1e51b22d4d70b92aa392b88a3c9c98bfc154a232cb7531ebef5694e","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect","hint":{"shader_family":"country_core/prefix_scan/inclusive_prefix_scan_subtotals","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan_subtotals::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":23124,"parent_record_offset":23088,"ancestor_record_offset":22724,"inlinee":30341,"source_line":31222,"archive":"_pdb/C/modules/00518.jsonl.gz"}},"shader_uri":"country_core/prefix_scan/inclusive_prefix_scan_subtotals:main"}
// [Shader 映射] {"shader_id":"e67af39e8fd352217b6f16ca","relative_path":"country-core/compiled/prefix_scan/inclusive_prefix_scan_merge.precomp","sha256":"b81dd9bf00b4bac9f6a11ecd4695ddf310e1a22c321f7eda314e642cfdddcca7","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect","hint":{"shader_family":"country_core/prefix_scan/inclusive_prefix_scan_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::prefix_scan::inclusive_prefix_scan_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":23252,"parent_record_offset":23216,"ancestor_record_offset":22724,"inlinee":30343,"source_line":31177,"archive":"_pdb/C/modules/00518.jsonl.gz"}},"shader_uri":"country_core/prefix_scan/inclusive_prefix_scan_merge:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/prefix_scan/inclusive_prefix_scan.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[9,14]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[512,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"inout_buf","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,33]},"decorations":[],"pointee":{"id":33,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[34]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/prefix_scan/inclusive_prefix_scan_subtotals.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[9]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[512,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"entryPointParams","storage_class":"PushConstant","type":{"id":24,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,23]},"decorations":[],"pointee":{"id":23,"name":"EntryPointParams_natural","record":{"opcode":"OpTypeStruct","operands":[5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"segment_count","type_id":5,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"input_buf","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,55]},"decorations":[],"pointee":{"id":55,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[56]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":56,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":286,"name":"output_buf","storage_class":"Uniform","type":{"id":285,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,284]},"decorations":[],"pointee":{"id":284,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[56]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":56,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/prefix_scan/inclusive_prefix_scan_merge.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[9,17]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[512,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"inout_buf","storage_class":"Uniform","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[37]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":37,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"subtotal_sum_buf","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,56]},"decorations":[],"pointee":{"id":56,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[37]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":37,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":11082480,"caller_aliases":{"alias_group_rva":11082480,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::grass2::grass2_splat_scan_and_header"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":11082943,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: prefix_scan.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect
// 信息 ID: 08be2ceaeb5cbd6106f111d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_scan_and_header @ 0xa91cbf -> 0x9bbc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
