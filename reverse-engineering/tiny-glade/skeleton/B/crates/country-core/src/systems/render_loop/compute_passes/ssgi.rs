// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6f54bf1c4c8fa7c5d98d8ef55d90c801973fdcb4b0c63859074018d86687aa64
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ssgi.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 484
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":56352,"rva":8864688,"va":5377573808}
// 编译实例 RVAs: 0x8743b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t138808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8381e0200","size":5,"soft_fail":false,"target_va":5377716144,"va":5377577331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t497545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e889970700","size":5,"soft_fail":false,"target_va":5378071808,"va":5377574258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t499045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8659d0700","size":5,"soft_fail":false,"target_va":5378073152,"va":5377574102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1009685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e815680f00","size":5,"soft_fail":false,"target_va":5378585968,"va":5377576278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1017895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e827880f00","size":5,"soft_fail":false,"target_va":5378593936,"va":5377576036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1024664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e898a20f00","size":5,"soft_fail":false,"target_va":5378601104,"va":5377576435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1478096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8d08d1600","size":5,"soft_fail":false,"target_va":5379052752,"va":5377574651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3549364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8b4283600","size":5,"soft_fail":false,"target_va":5381123296,"va":5377573927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3595668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e894dd3600","size":5,"soft_fail":false,"target_va":5381170352,"va":5377574679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14216692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8f4edd800","size":5,"soft_fail":false,"target_va":5391790752,"va":5377574055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: ssgi.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34273037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e80df70a02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377574126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5077227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8eb784d00","size":5,"soft_fail":false,"target_va":5382655024,"va":5377577792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33965680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e870460602","size":5,"soft_fail":false,"target_va":5411542880,"va":5377577195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3543973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8a5133600","size":5,"soft_fail":false,"target_va":5381117840,"va":5377573862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3542688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e8a00e3600","size":5,"soft_fail":false,"target_va":5381116576,"va":5377573883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3540776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e828073600","size":5,"soft_fail":false,"target_va":5381116784,"va":5377576003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3540819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e853073600","size":5,"soft_fail":false,"target_va":5381116816,"va":5377575992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3625084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377573808,"raw_hex":"e87c503700","size":5,"soft_fail":false,"target_va":5381199056,"va":5377573967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":484,"record_offset":58092,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":484,"record_offset":56828,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":484,"record_offset":56780,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":484,"record_offset":56684,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":484,"record_offset":58168,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":484,"record_offset":56560,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":484,"record_offset":56532,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":484,"record_offset":56624,"parent_record_offset":56560,"ancestor_record_offset":56352,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":484,"record_offset":58212,"parent_record_offset":58168,"ancestor_record_offset":56352,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":56928,"parent_record_offset":56860,"ancestor_record_offset":56352,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":484,"record_offset":56860,"parent_record_offset":56828,"ancestor_record_offset":56352,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":484,"record_offset":56600,"parent_record_offset":56560,"ancestor_record_offset":56352,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":484,"record_offset":56652,"parent_record_offset":56560,"ancestor_record_offset":56352,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00484.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":484,"record_offset":56684,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00484.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":484,"record_offset":56780,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00484.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":484,"record_offset":58168,"parent_record_offset":56352,"ancestor_record_offset":56352,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00484.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"236ab5ebf3b77975582a238e","relative_path":"_ssgi_ssgi.cs.hlsl_cs_main_0.bin","sha256":"66e61114e7e782b18c01a857168ac52b587b1a063635a084e2af6b5caebef57d","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"1986fd2022c396ed32c14214","relative_path":"_ssgi_temporal_filter.cs.hlsl_cs_main_0.bin","sha256":"d31c87e724cf473fea5cc9cf65e66699177e57997eb8b8aaf9651482313920b1","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"53cbaddb37825f7c1bfc5289","relative_path":"_ssgi_upsample.cs.hlsl_cs_main_0.bin","sha256":"2a466f7f8b21c2cedf861db078351469a1dc11eebc14e29de06359d8ecc077d6","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"ssgi","prefix_candidates":["ssgi","gtao_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_temporal_filter.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":49,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"frame_constants","storage_class":"Uniform","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[40,40,40,40,40,40,40,40,34,33,33,34,50,34,33,51,33,51,34,51,34,33,34,34,34,40,30]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":40,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":40,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":40,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":40,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":40,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":40,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":40,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":34,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":33,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":33,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":34,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":50,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":34,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":33,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":33,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":51,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":34,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":51,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":34,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":33,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":34,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":34,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":34,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":40,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":30,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"$Globals","storage_class":"Uniform","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[34,50,50,50,50,50]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"history_valid","type_id":34,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"input_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"history_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"reprojection_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"final_output_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"history_output_tex_dims","type_id":50,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"input_tex","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[33,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"history_tex","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[33,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[33,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"final_output_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,26]},"decorations":[],"pointee":{"id":26,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[33,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"history_output_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,26]},"decorations":[],"pointee":{"id":26,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[33,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[34,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_ssgi.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":20,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,19]},"decorations":[],"pointee":{"id":19,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":23,"name":"frame_constants","storage_class":"Uniform","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,21]},"decorations":[],"pointee":{"id":21,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[76,76,76,76,76,76,76,76,37,36,36,37,82,37,36,60,36,60,37,60,37,36,37,37,37,76,32]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":76,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":76,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":76,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":37,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":36,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":36,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":37,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":82,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":37,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":36,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":60,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":36,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":60,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":37,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":60,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":37,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":36,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":37,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":37,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":37,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":76,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":32,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"$Globals","storage_class":"Uniform","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[82,82,82]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":82,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"half_view_normal_tex_dims","type_id":82,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"output_tex_dims","type_id":82,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[36,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"half_view_normal_tex","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,24]},"decorations":[],"pointee":{"id":24,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[36,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"output_tex","storage_class":"UniformConstant","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[36,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,89]},"decorations":[],"pointee":{"id":89,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _ssgi_upsample.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"frame_constants","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,15]},"decorations":[],"pointee":{"id":15,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[37,37,37,37,37,37,37,37,32,31,31,32,43,32,31,50,31,50,32,50,32,31,32,32,32,37,28]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":37,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":37,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":37,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":37,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":37,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":37,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":37,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":37,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":32,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":31,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":31,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":32,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":43,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":32,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":31,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":31,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":50,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":32,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":50,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":32,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":31,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":32,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":32,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":32,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":37,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":28,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"ssgi_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,18]},"decorations":[],"pointee":{"id":18,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[31,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":20,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,18]},"decorations":[],"pointee":{"id":18,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[31,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,18]},"decorations":[],"pointee":{"id":18,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[31,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"half_view_normal_tex","storage_class":"UniformConstant","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,18]},"decorations":[],"pointee":{"id":18,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[31,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"gbuffer_tex","storage_class":"UniformConstant","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,23]},"decorations":[],"pointee":{"id":23,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[32,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":26,"name":"output_tex","storage_class":"UniformConstant","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,25]},"decorations":[],"pointee":{"id":25,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[31,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[32,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162760,"position_among_mapped_render_targets":42,"mapped_render_target_count":59,"previous_callsite":5162713,"next_callsite":5163398,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/ray_trace_gi.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ssgi.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::world::deferred_world::DeferredWorld,bevy_ecs::observer::ObserverTrigger,bevy_ptr::PtrMut<bevy_ptr::Aligned>,ref_mut$<bool>)
// 候选参数类型: ["bevy_ecs::world::deferred_world::DeferredWorld", "bevy_ecs::observer::ObserverTrigger", "bevy_ptr::PtrMut<bevy_ptr::Aligned>", "ref_mut$<bool>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
// 信息 ID: 524e5ae8e4680ee4b712e0a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec708 -> 0x8743b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
