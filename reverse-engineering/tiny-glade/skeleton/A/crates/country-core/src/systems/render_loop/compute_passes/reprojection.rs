// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b454ec8b5181094a8dfe26be94e9552f435448c52209a938f236e3de3c423c91
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reprojection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 447
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
// 函数符号: country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":447,"record_offset":15436,"rva":6300416,"va":5375009536}
// 编译实例 RVAs: 0x602300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-493220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e85c79f8ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375009695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e80e090400","size":5,"soft_fail":false,"target_va":5375274368,"va":5375009901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t265735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e8070e0400","size":5,"soft_fail":false,"target_va":5375275808,"va":5375010068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t304446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e83ea50400","size":5,"soft_fail":false,"target_va":5375314864,"va":5375010413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t967464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e828c30e00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375011155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1229474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e8a2c21200","size":5,"soft_fail":false,"target_va":5376239920,"va":5375010441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8174716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e87cbc7c00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375009823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8218496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e880677d00","size":5,"soft_fail":false,"target_va":5383228352,"va":5375009851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: reprojection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28583426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e80226b401","size":5,"soft_fail":false,"target_va":5403593360,"va":5375009929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1332123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e89b531400","size":5,"soft_fail":false,"target_va":5376341760,"va":5375009632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1330854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e8a64e1400","size":5,"soft_fail":false,"target_va":5376340512,"va":5375009653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1209812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375009536,"raw_hex":"e8d4751200","size":5,"soft_fail":false,"target_va":5376219552,"va":5375009735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":447,"record_offset":15892,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":447,"record_offset":15844,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":447,"record_offset":15748,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":447,"record_offset":15656,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":447,"record_offset":15628,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":447,"record_offset":15716,"parent_record_offset":15656,"ancestor_record_offset":15436,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":447,"record_offset":15992,"parent_record_offset":15924,"ancestor_record_offset":15436,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":447,"record_offset":15924,"parent_record_offset":15892,"ancestor_record_offset":15436,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":447,"record_offset":15692,"parent_record_offset":15656,"ancestor_record_offset":15436,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00447.jsonl.gz"}
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
// 完整结构及原始类型记录: reprojection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reprojection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: reprojection.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":447,"record_offset":15748,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00447.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":447,"record_offset":15844,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00447.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139468,"position_among_mapped_render_targets":39,"mapped_render_target_count":70,"previous_callsite":3139336,"next_callsite":3139539,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/half_res.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reprojection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map
// 信息 ID: d590a0d0f0f5c1cd433545dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe78c -> 0x602300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
