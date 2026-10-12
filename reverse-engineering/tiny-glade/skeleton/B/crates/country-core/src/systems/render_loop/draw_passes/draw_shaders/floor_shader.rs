// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 0c5912765c5addb99fc52ec12e01b84b51af867fb1862ebe088fcad72e08b922
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: floor_shader.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 530
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":78764,"rva":10267824,"va":5378976944}
// 编译实例 RVAs: 0x9cacb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1277040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e89083ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378978347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-796771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e89dd7f3ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378977390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-794854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e81adff3ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5378977073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-783164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e8c40cf4ff","size":5,"soft_fail":false,"target_va":5378194288,"va":5378977447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-781217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e85f14f4ff","size":5,"soft_fail":false,"target_va":5378196224,"va":5378977436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-391557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e87b06faff","size":5,"soft_fail":false,"target_va":5378586704,"va":5378978256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: floor_shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32870150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e8068ff501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378977013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-803510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378976944,"raw_hex":"e84abdf3ff","size":5,"soft_fail":false,"target_va":5378174784,"va":5378978289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":530,"record_offset":79164,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":530,"record_offset":79740,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":530,"record_offset":79036,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":78956,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":27055,"source_line":624,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":79784,"parent_record_offset":79740,"ancestor_record_offset":78764,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":530,"record_offset":79064,"parent_record_offset":79036,"ancestor_record_offset":78764,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":78996,"parent_record_offset":78976,"ancestor_record_offset":78764,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":78976,"parent_record_offset":78956,"ancestor_record_offset":78764,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00530.jsonl.gz"}
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
// 完整结构及原始类型记录: floor_shader.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: floor_shader.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/floor_shader.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: floor_shader.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":530,"record_offset":79036,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":530,"record_offset":79064,"parent_record_offset":79036,"ancestor_record_offset":78764,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":530,"record_offset":79164,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":530,"record_offset":79740,"parent_record_offset":78764,"ancestor_record_offset":78764,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":10012288,"caller_aliases":{"alias_group_rva":10012288,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10013671,"position_among_mapped_render_targets":1,"mapped_render_target_count":2,"previous_callsite":null,"next_callsite":10014615,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10192240,"caller_aliases":{"alias_group_rva":10192240,"count":1,"display_examples":["core::ops::function::FnOnce::call_once"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10192306,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: floor_shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader
// 信息 ID: 1dfc96aedc4933e4d6d2d202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data @ 0x98cbe7 -> 0x9cacb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$46,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b85b2 -> 0x9cacb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
