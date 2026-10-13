// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/b5bc5f4651942f15/api.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: api.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: api.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: api.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] u32 (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::add_recorded_node
// 信息 ID: 6d5d580b4b349bd7441c61b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> | core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> @ 0xe23611 -> 0xe25350 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0xe238f8 -> 0xe25350 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::impl$0::drop @ 0xe32123 -> 0xe25350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::add_resource_access_impl<1>
// 信息 ID: f04104b24ce107032a7c6a91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::api::GraphApi::add_resource_access_impl -> ["1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 102 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e4c6f -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e4da7 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e4ed6 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e50ca -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e52be -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e54af -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e55c4 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e5c70 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e5da5 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e5f9a -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e618c -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e637b -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::taa::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db25b -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::taa::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db38e -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::taa::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db4b7 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr_blur_lum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db8d8 -> 0x10e1820 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::add_resource_access_impl<2>
// 信息 ID: 7936edd42006381d4f9349de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::api::GraphApi::add_resource_access_impl -> ["2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 41 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e4fe4 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e51d8 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e53c9 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e5eb5 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e60a7 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10e6299 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::taa::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db5c9 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::taa::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db6a3 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr_blur_lum::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21db7fb -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr_reproject::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21dbaee -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr_reproject::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21dbce1 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr_reproject::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21dbec5 -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::shader::tgsr2_decide::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21dc76a -> 0x10e14f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_shadow::shader::setup_shadows::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x265ffe3 -> 0x1cb0450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_shadow::shader::setup_shadows::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x26600cc -> 0x1cb0450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_post::shader::blur::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x1cb8b19 -> 0x1cb9e60 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::clear_buffer
// 信息 ID: 10707cc8bb75c32f1170ba20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9b2d4a -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc672 -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9bee1f -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9bf281 -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x9bfdce -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0xa8d417 -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat @ 0xa8e55e -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat @ 0xa8e56d -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_compute_main @ 0xa8ee7a -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa91442 -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa91452 -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0xaea89a -> 0xe25190 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_core::reduction::generate_mips @ 0xe06ca0 -> 0xe25190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::clear_color_texture
// 信息 ID: 3f82d83dc352f98f7bf49cc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask @ 0x9b153a -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat @ 0xa8e2f0 -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa911a4 -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0xae9f29 -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0xb1f540 -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0xb1f54f -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0xb1f623 -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics @ 0xb1f62f -> 0xe26ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::clear_color_texture_f32
// 信息 ID: 3ea9c7c136fc9ed5be9a3513
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3999e5 -> 0xe26e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass @ 0xae845f -> 0xe26e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_and_push_binding_set_from_stack
// 信息 ID: 0cf0bba3f674afeb0b740da1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c03e -> 0xe289c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_binding_set
// 信息 ID: 3ea48a1015d2c9a8c4b127ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x454e1c -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x4552fc -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x4557ec -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x455cbd -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455d8c -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x4562ac -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x45680c -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::frame::FrameCounter,rhapsody_core::shader::frame::FrameCounter> | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::current_view::CurrentView,country_core::shader::inc::current_view::CurrentView> @ 0x456cec -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x45718c -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x45766c -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::frame::FrameConstants,country_core::shader::inc::frame::FrameConstants> @ 0x457b65 -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<rhapsody::rg::binding::RgProtocol> > @ 0x4580ec -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::flag::FlagTextures<rhapsody::rg::binding::RgProtocol> > @ 0x4585cc -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::roof_color_data::RoofColors<rhapsody::rg::binding::RgProtocol> > @ 0x458abc -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56d42f -> 0xe25f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56d5d6 -> 0xe25f50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_binding_set_from_stack
// 信息 ID: 56bbb8594655a2a7a4cd3f8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56adcf -> 0xe27b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b300 -> 0xe27b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b75e -> 0xe27b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody::raw::binding_set::BindAsUniformBuffer<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 6908271d12d79fd45e9294ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::binding_set::BindAsUniformBuffer -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::api::GraphApi::create_instant_binding_set -> ["rhapsody_core::shader::view::ViewConstants", "rhapsody::raw::binding_set::BindAsUniformBuffer<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_shadow::setup_directional_light_cascades @ 0xca9899 -> 0x1cb0a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol>,rhapsody_shadow::shader::shadow::CascadedShadowSetup<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: d0d575d812973b74822bdc83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_shadow::shader::shadow::CascadedShadowSetup -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody_shadow::shader::shadow::CascadedShadowSetup -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::api::GraphApi::create_instant_binding_set -> ["rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol>", "rhapsody_shadow::shader::shadow::CascadedShadowSetup<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_shadow::setup_directional_light_cascades @ 0xca9900 -> 0x1cb0f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_node
// 信息 ID: ff486c08a14a04b0613d78cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x451692 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1> > @ 0x451fb7 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9be127 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x9f4370 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1> > @ 0x1751507 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1751b2e -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > @ 0x1752452 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1752d88 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175c495 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1767b65 -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::buffer_copy_from_slice<u32> @ 0x176aafa -> 0xe25110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> @ 0x26ea8b9 -> 0xe25110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::create_raw_resource
// 信息 ID: 109225b39639042f15af7f28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 123 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x395c2f -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39954b -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::impl$6::create<rhapsody::rg::resource::TextureDesc> @ 0x4515af -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bc8db -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b074 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b141 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56ba6e -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_taa @ 0x6f1610 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1955 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1ad7 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1ba7 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1c94 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1e17 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f239d -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2522 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f25f4 -> 0xe26cf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::ensure_texture_mutable_format
// 信息 ID: 7f8644adb41ca3cdc25d33c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 233 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::screenshot::capture_screenshot_texture::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x78ea4 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::capture_screenshot_texture::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x790ba -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::tgsr_capture_hires::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x10338d -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::capture_screenshot_buffer::impl$6::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x1ef7cf -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::capture_screenshot_buffer::impl$6::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x1ef8e9 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::blur_downsample::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21f30b -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::blur_downsample::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x21f4f9 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x234308 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2357c8 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x236c48 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::tgsr_capture_ref::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x50dd0d -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::capture_panic_screenshot::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x5369b6 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::tgsr_capture_native::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x5dc55d -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::screenshot::tgsr_capture_native::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x5dc676 -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x902b1c -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::compute_froxel_lighting::impl$2::bind_entry_point<rhapsody::rg::binding::RgProtocol,rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder> @ 0x902c2d -> 0xe27bb0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::find_binding_set_stack_item
// 信息 ID: 2504a564021b2aed03623d5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae3250 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae326c -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae328b -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf66b0 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175f99b -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175f9bc -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175f9e0 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x1763282 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x17632a3 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x17632c7 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x176853c -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768561 -> 0xe27610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768589 -> 0xe27610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::import_binding_set
// 信息 ID: 5bd0c36ef8ce1b0119db35f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x39584e -> 0xe26210 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3991fd -> 0xe26210 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3994b8 -> 0xe26210 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x1772456 -> 0xe26210 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e6797 -> 0xe26210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::init_binding_set_from_stack
// 信息 ID: dbd8ea385deda5a6d73e8b9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56d630 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data @ 0x9b4ae0 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data @ 0x9b4b06 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0xae8f26 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_binding_set_from_stack @ 0xe27b95 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_and_push_binding_set_from_stack @ 0xe289d6 -> 0xe276d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage
// 信息 ID: 1528e123466d1189ce24d3dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource::impl$1::init_as_debug_garbage @ 0xe14689 -> 0xe27d50 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe2559c -> 0xe27d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::mark_binding_set_fully_initialized
// 信息 ID: 6245246f55b34988096fd9de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data @ 0x9b4aeb -> 0xe288c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::new
// 信息 ID: d95f344472249951e7bf2cf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x3957cd -> 0xe28a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x398259 -> 0xe28a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,rhapsody::rg::executor::impl$12::present::closure$0::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > > @ 0x39cfeb -> 0xe28a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x17723d7 -> 0xe28a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e66ea -> 0xe28a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::push_binding_set
// 信息 ID: bfe2a7202c153d6a87c2c05a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x395864 -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3994ce -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x45509b -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x45559a -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x455a6b -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x455cf1 -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x456056 -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x45659e -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x456a8b -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::frame::FrameCounter,rhapsody_core::shader::frame::FrameCounter> | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::current_view::CurrentView,country_core::shader::inc::current_view::CurrentView> @ 0x456f35 -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x45740b -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x45790a -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::frame::FrameConstants,country_core::shader::inc::frame::FrameConstants> @ 0x457e77 -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<rhapsody::rg::binding::RgProtocol> > @ 0x45836b -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::flag::FlagTextures<rhapsody::rg::binding::RgProtocol> > @ 0x45886a -> 0xe252e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::roof_color_data::RoofColors<rhapsody::rg::binding::RgProtocol> > @ 0x458d5a -> 0xe252e0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::uniform_buffer_from_slice<tuple$<i32,i32,i32,i32> >
// 信息 ID: 85813aed6797cf2310a22ec4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["i32", "i32", "i32", "i32"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::api::GraphApi::uniform_buffer_from_slice -> ["tuple$<i32,i32,i32,i32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccec12 -> 0x1cfdb00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::GraphApi::uniform_buffer_from_slice<u32>
// 信息 ID: 4db5afbec6f5ad8665533706
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::api::GraphApi::uniform_buffer_from_slice -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2a26 -> 0x10e1ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce366 -> 0x1cfdd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::impl$4::init_binding_set::closure$0<rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol>,rhapsody_shadow::shader::shadow::CascadedShadowSetup<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 402661b835c06930b239cb3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_shadow::shader::shadow::CascadedShadowSetup -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody_shadow::shader::shadow::CascadedShadowSetup -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::api::impl$4::init_binding_set::closure$0 -> ["rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol>", "rhapsody_shadow::shader::shadow::CascadedShadowSetup<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::api::impl$4::init_binding_set", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody::raw::binding_set::BindAsUniformBuffer<rhapsody::rg::binding::RgProtocol> > @ 0x1cb0dc1 -> 0x2ddeb30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol>,rhapsody_shadow::shader::shadow::CascadedShadowSetup<rhapsody::rg::binding::RgProtocol> > @ 0x1cb1301 -> 0x2ddeb30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::impl$5::read_buf
// 信息 ID: a9409c9fb8c39fbd5ac4e8da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x454f03 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x4553e6 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x455409 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x4558d3 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455e83 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x4563cb -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x4563eb -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x45640b -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x457273 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x457756 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<rhapsody::rg::binding::RgProtocol> > @ 0x4581d3 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::roof_color_data::RoofColors<rhapsody::rg::binding::RgProtocol> > @ 0x458bc9 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::water::inc::waves::WaterWaves<shabriri_rt::api::EmptyProtocol>,country_core::shader::water::inc::waves::WaterWaves<rhapsody::rg::binding::RgProtocol> > @ 0x176d1f6 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::rocky_terrain::inc::mesh_data::RockyTerrainMesh<shabriri_rt::api::EmptyProtocol>,country_core::shader::rocky_terrain::inc::mesh_data::RockyTerrainMesh<rhapsody::rg::binding::RgProtocol> > @ 0x176d6e6 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::rocky_terrain::inc::mesh_data::RockyTerrainMesh<shabriri_rt::api::EmptyProtocol>,country_core::shader::rocky_terrain::inc::mesh_data::RockyTerrainMesh<rhapsody::rg::binding::RgProtocol> > @ 0x176d709 -> 0xe22140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParamsBlock<shabriri_rt::api::EmptyProtocol>,country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParamsBlock<rhapsody::rg::binding::RgProtocol> > @ 0x176dbd3 -> 0xe22140 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::impl$5::read_tex
// 信息 ID: 521c6994b570833694caded4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455ea3 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455ec3 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x4568f3 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x457779 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::flag::FlagTextures<rhapsody::rg::binding::RgProtocol> > @ 0x4586b6 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::flag::FlagTextures<rhapsody::rg::binding::RgProtocol> > @ 0x4586d9 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::roof_color_data::RoofColors<rhapsody::rg::binding::RgProtocol> > @ 0x458ba6 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::buildings::inc::building_mask::BuildingMaskTex<shabriri_rt::api::EmptyProtocol>,country_core::shader::buildings::inc::building_mask::BuildingMaskTex<rhapsody::rg::binding::RgProtocol> > @ 0x176b8a6 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::buildings::inc::building_mask::BuildingMaskTex<shabriri_rt::api::EmptyProtocol>,country_core::shader::buildings::inc::building_mask::BuildingMaskTex<rhapsody::rg::binding::RgProtocol> > @ 0x176b8c9 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::terrain_height::TerrainHeightmap<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::terrain_height::TerrainHeightmap<rhapsody::rg::binding::RgProtocol> > @ 0x176bde6 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::terrain_height::VisualTerrainHeightmap<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::terrain_height::VisualTerrainHeightmap<rhapsody::rg::binding::RgProtocol> > @ 0x176c33f -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::foliage_exclusion::FoliageExclusionMask<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::foliage_exclusion::FoliageExclusionMask<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::world::GrassRemovalMask<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::world::GrassRemovalMask<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::world::PathsVisual<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::world::PathsVisual<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::world::PlantSampleMask<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::world::PlantSampleMask<rhapsody::rg::binding::RgProtocol> > | 其余 5 个同地址别名见共享索引 @ 0x176c833 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::object_push::ObjectPush<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::object_push::ObjectPush<rhapsody::rg::binding::RgProtocol> > @ 0x176cd13 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::water::inc::waves::WaterWaves<shabriri_rt::api::EmptyProtocol>,country_core::shader::water::inc::waves::WaterWaves<rhapsody::rg::binding::RgProtocol> > @ 0x176d219 -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::paths::inc::path_mask_sampler::PathExclusionMask<shabriri_rt::api::EmptyProtocol>,country_core::shader::paths::inc::path_mask_sampler::PathExclusionMask<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::world::GardenMask<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::world::GardenMask<rhapsody::rg::binding::RgProtocol> > @ 0x176e0ef -> 0xe221c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_shadow::shader::shadow::CascadedShadows<shabriri_rt::api::EmptyProtocol>,rhapsody_shadow::shader::shadow::CascadedShadows<rhapsody::rg::binding::RgProtocol> > @ 0x176eb0b -> 0xe221c0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::api::impl$5::uniform_buf
// 信息 ID: 1a3cdd1edda8d010ce46e241
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_instant_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody::raw::binding_set::BindAsUniformBuffer<rhapsody::rg::binding::RgProtocol> > @ 0x1cb0b79 -> 0xe220c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
