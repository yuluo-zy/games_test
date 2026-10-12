// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/half_res.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 8cea0001ac2629095d44ecfb619bf2e332926e0040a389ec987457a5d15db608
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_res.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 501
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":501,"record_offset":35336,"rva":9396608,"va":5378105728}
// 编译实例 RVAs: 0x8f6180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-391087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e85108faff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378107226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e88f78ffff","size":5,"soft_fail":false,"target_va":5378071808,"va":5378106476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8c07effff","size":5,"soft_fail":false,"target_va":5378073152,"va":5378106235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t945783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8776e0e00","size":5,"soft_fail":false,"target_va":5379052752,"va":5378106964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3017367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8970a2e00","size":5,"soft_fail":false,"target_va":5381123296,"va":5378105924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3063362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e842be2e00","size":5,"soft_fail":false,"target_va":5381170352,"va":5378106985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13684559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e84fcfd000","size":5,"soft_fail":false,"target_va":5391790752,"va":5378106188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: half_res.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33740906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e86ad80202","size":5,"soft_fail":false,"target_va":5411847168,"va":5378106257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::div_up_extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3011549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8ddf32d00","size":5,"soft_fail":false,"target_va":5381117408,"va":5378105854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3011973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e885f52d00","size":5,"soft_fail":false,"target_va":5381117840,"va":5378105862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3010685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e87df02d00","size":5,"soft_fail":false,"target_va":5381116576,"va":5378105886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::aspect_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3011803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8dbf42d00","size":5,"soft_fail":false,"target_va":5381118112,"va":5378106304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3011575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e8f7f32d00","size":5,"soft_fail":false,"target_va":5381117856,"va":5378106276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3093087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378105728,"raw_hex":"e85f322f00","size":5,"soft_fail":false,"target_va":5381199056,"va":5378105964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":501,"record_offset":36020,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":501,"record_offset":35876,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":501,"record_offset":35784,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":501,"record_offset":35832,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":28228,"source_line":718,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":501,"record_offset":35528,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":501,"record_offset":35596,"parent_record_offset":35528,"ancestor_record_offset":35336,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":501,"record_offset":36120,"parent_record_offset":36048,"ancestor_record_offset":35336,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":501,"record_offset":36048,"parent_record_offset":36020,"ancestor_record_offset":35336,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":501,"record_offset":35568,"parent_record_offset":35528,"ancestor_record_offset":35336,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00501.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":501,"record_offset":35624,"parent_record_offset":35528,"ancestor_record_offset":35336,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00501.jsonl.gz"}
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
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":501,"record_offset":35784,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00501.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":501,"record_offset":35876,"parent_record_offset":35336,"ancestor_record_offset":35336,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00501.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162592,"position_among_mapped_render_targets":40,"mapped_render_target_count":59,"previous_callsite":5162571,"next_callsite":5162713,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/reprojection.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: half_res.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
// 信息 ID: 5e2bdfaf6e3da25d88ad3dc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec660 -> 0x8f6180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
