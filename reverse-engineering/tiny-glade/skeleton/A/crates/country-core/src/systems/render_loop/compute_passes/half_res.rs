// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/half_res.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f279f3edf84c8dc518ca38e284b208e724c45ac3492dc1acc209b9a8fa0cb1b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_res.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 490
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
// 函数符号: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":49192,"rva":7001600,"va":5375710720}
// 编译实例 RVAs: 0x6ad600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1194449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e82fc6edff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375710924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-436882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e86e55f9ff","size":5,"soft_fail":false,"target_va":5375274368,"va":5375711245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-435683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e81d5af9ff","size":5,"soft_fail":false,"target_va":5375275808,"va":5375711486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-397115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8c5f0f9ff","size":5,"soft_fail":false,"target_va":5375314864,"va":5375711974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t266385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e891100400","size":5,"soft_fail":false,"target_va":5375978624,"va":5375712234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8300e0800","size":5,"soft_fail":false,"target_va":5376239920,"va":5375711995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7473341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8bd087200","size":5,"soft_fail":false,"target_va":5383184544,"va":5375711198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27882088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e86872a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375711267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_up_extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t630473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8c99e0900","size":5,"soft_fail":false,"target_va":5376341328,"va":5375710850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t630897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e871a00900","size":5,"soft_fail":false,"target_va":5376341760,"va":5375710858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t629625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8799b0900","size":5,"soft_fail":false,"target_va":5376340512,"va":5375710882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::aspect_mask
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t630713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8b99f0900","size":5,"soft_fail":false,"target_va":5376342032,"va":5375711314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t630485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8d59e0900","size":5,"soft_fail":false,"target_va":5376341776,"va":5375711286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t508583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375710720,"raw_hex":"e8a7c20700","size":5,"soft_fail":false,"target_va":5376219552,"va":5375710964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":490,"record_offset":49816,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":490,"record_offset":49672,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":490,"record_offset":49580,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":490,"record_offset":49628,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":25800,"source_line":702,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":490,"record_offset":49384,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":49452,"parent_record_offset":49384,"ancestor_record_offset":49192,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":490,"record_offset":49916,"parent_record_offset":49844,"ancestor_record_offset":49192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":490,"record_offset":49844,"parent_record_offset":49816,"ancestor_record_offset":49192,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":490,"record_offset":49424,"parent_record_offset":49384,"ancestor_record_offset":49192,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00490.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":490,"record_offset":49580,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":490,"record_offset":49672,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3139336,"position_among_mapped_render_targets":38,"mapped_render_target_count":70,"previous_callsite":3139297,"next_callsite":3139468,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: half_res.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 信息 ID: f8e4f9c7c412c7d931e602a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe708 -> 0x6ad600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
