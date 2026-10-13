// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00600.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00600.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00600.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00600.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_wooden_plank::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: a4d843c11f368499ccf9eec5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_wooden_plank::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_grass2_lod0::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_roof_tile::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_wooden_gable_plank::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::cull_and_bucket_wooden_plank::main<rhapsody::rg::binding::RgProtocol> > @ 0x177ff16 -> 0x9d69d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::cull_and_bucket_wooden_plank::impl$3::to_shader_permutation_key<rhapsody::rg::binding::RgProtocol>
// 信息 ID: 9b1b7862f2c049af19769f77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::cull_and_bucket_wooden_plank::impl$3::to_shader_permutation_key -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc930 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0xc309c3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0xc30cf3 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo,country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenGablePlank> @ 0xc31023 -> 0x9d7390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::nani::histogram_prefix_sum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 1bc4c3386ae5d0445f5796c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::nani::histogram_prefix_sum::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::water::calc_deferred_rt_args::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::histogram_prefix_sum::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::clear_histogram::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_editing::read_terrain_rock_mask::main<rhapsody::rg::binding::RgProtocol> > | 其余 1 个同地址别名见共享索引 @ 0x177dafa -> 0x9d8e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::shader::terrain_derivative::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: 7c291f03c9bcce0614630230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::terrain_derivative::impl$2::bind_entry_point -> ["rhapsody::rg::binding::RgProtocol", "rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_derivative::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::lut::water_noise::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::terrain_lighting_texture::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::rasterize_buildings_animate::main<rhapsody::rg::binding::RgProtocol> > @ 0x177a34d -> 0x903ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
