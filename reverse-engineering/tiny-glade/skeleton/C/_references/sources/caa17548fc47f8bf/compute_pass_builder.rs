// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/caa17548fc47f8bf/compute_pass_builder.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: compute_pass_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compute_pass_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compute_pass_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] u32 (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi::dispatch
// 信息 ID: 4fb85955f9ff58607999f710
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x1775e6f -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_grass2_lod0::closure$0::closure_env$0,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x1776312 -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x177657f -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_instance_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x1776bb2 -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x1776e59 -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x1777309 -> 0xe24f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x177750f -> 0xe24f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<clutter_icon_pipe::shader::clutter_icon_compose::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: cb03e1944824ab1ccff269c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] clutter_icon_pipe::shader::clutter_icon_compose::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["clutter_icon_pipe::shader::clutter_icon_compose::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e703d -> 0x2ab1a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<clutter_icon_pipe::shader::clutter_icon_light::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 241920422aef839551759682
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] clutter_icon_pipe::shader::clutter_icon_light::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["clutter_icon_pipe::shader::clutter_icon_light::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e6ee5 -> 0x2ab1420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 56c693e00473c7fb67ee1b1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blur_downsample::cs_main_multisampled -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bca25 -> 0x477610 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcacf -> 0x477610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 88838e32211d085888253ddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blur_downsample::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bca25 -> 0x477610 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcacf -> 0x477610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 1c7914c13a6f5f9d4d24e548
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::capture_panic_screenshot::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x3f7a48 -> 0x476fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 9a3bd8bd449c1d45d0fe91ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::capture_screenshot_buffer::sample4 -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcdf6 -> 0x477c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 1161867f29945f66c43e6b69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::capture_screenshot_texture::sample1 -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcc48 -> 0x4787d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcf6a -> 0x4787d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: d787c73af3ada2e3faeabed9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::capture_screenshot_texture::sample4 -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcc48 -> 0x4787d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcf6a -> 0x4787d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 129fb20936e7582899be30be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_hires::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b194c -> 0x4781f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 67dcac4cc015f14c70d93515
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_native::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1a4c -> 0x479980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 940ceefc0efdac535bf89052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::screenshot::tgsr_capture_ref::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1859 -> 0x479390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: cfed2ad0dc14114c6cfc6a49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::upload_cpu_raster::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x69f7d7 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x69fd67 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x6a02f7 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster> @ 0x6a0887 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster> @ 0x6a0e17 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster> @ 0x6a13a7 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster> @ 0x6a1937 -> 0x478dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody::rg::binding::BindDynamic>
// 信息 ID: 06b8e4fb73070f324a0d7efa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody::rg::binding::BindDynamic"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe257c9 -> 0xe240d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage @ 0xe28296 -> 0xe240d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_core::shader::calculate_reprojection_map::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 4ae133aada9226762268bdc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_core::shader::calculate_reprojection_map::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_core::shader::calculate_reprojection_map::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reprojection::render_reprojection_map @ 0xe04db5 -> 0x1e4e150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_core::shader::copy_depth_to_r::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 21b2140411e5dce245dd48be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_core::shader::copy_depth_to_r::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_core::shader::copy_depth_to_r::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reprojection::render_reprojection_map @ 0xe04e60 -> 0x1e4db90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_core::shader::depth_min_max::main_head<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 8dee861ac5e7a2927e7fb980
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_core::shader::depth_min_max::main_head -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_core::shader::depth_min_max::main_head<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reduction::depth_min_max @ 0xe06a6a -> 0x1e4e730 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_core::reduction::depth_min_max_tail @ 0xe07559 -> 0x1e4e730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_core::shader::depth_min_max::main_tail<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: d6ca41bcbe188276371324b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_core::shader::depth_min_max::main_tail -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_core::shader::depth_min_max::main_tail<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reduction::depth_min_max @ 0xe06a6a -> 0x1e4e730 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_core::reduction::depth_min_max_tail @ 0xe07559 -> 0x1e4e730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_core::shader::generate_mips::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: af1a0f28f9af0d2eeaedfc03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_core::shader::generate_mips::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_core::shader::generate_mips::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reduction::generate_mips @ 0xe07178 -> 0x1e4ecf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::analyze::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 011d9552c7227709278185e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::analyze::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::analyze::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce4fa -> 0x1d00520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::clear_halfres::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 05035e40e91c9ec57df956c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::clear_halfres::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::clear_halfres::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce779 -> 0x1cfff60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::fullres_all_lod0::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: ad1e2f336b6aa1cbbdea8b98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::fullres_all_lod0::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::fullres_all_lod0::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce89a -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce9b3 -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcceacc -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccebde -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::fullres_all_lod1::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 84a57debeeb45f1064d284d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::fullres_all_lod1::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::fullres_all_lod1::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce89a -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce9b3 -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcceacc -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccebde -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::fullres_upsample::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 0c27095f927698ed3504fa0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::fullres_upsample::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::fullres_upsample::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccedea -> 0x1cfee00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::halfres::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: ed1872066cab9152015e1664
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::halfres::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::halfres::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce89a -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce9b3 -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcceacc -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccebde -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::halfres_all::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 94bef7b7dc37aa581e1793e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::halfres_all::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::halfres_all::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce89a -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce9b3 -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcceacc -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xccebde -> 0x1cfe830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_dof::shader::init_buffers::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 8c21a8562cb1198f5788fea1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_dof::shader::init_buffers::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_dof::shader::init_buffers::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce40f -> 0x1cff3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::blur_screen_radiance::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 725277853e0bf78f8fe33f26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::blur_screen_radiance::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::blur_screen_radiance::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xcccb68 -> 0x1cf9550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::eval_screen_radiance::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 15150fb2a1badaba68ff013b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::eval_screen_radiance::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::eval_screen_radiance::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xccc7e8 -> 0x1cfa120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::gtao::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 4f12e25d2f3f28106e05c2f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::gtao::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::gtao::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccbfe2 -> 0x1cf8f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::gtao_temporal::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: e9daf539a0977038947517a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::gtao_temporal::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::gtao_temporal::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccc440 -> 0x1cf83d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::gtao_upsample::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 59b46ff07bce5394dcdf4613
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::gtao_upsample::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::gtao_upsample::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccc1b5 -> 0x1cf89b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_gi::shader::reproject_screen_radiance::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 8fcdaf5e77768fb123e2829f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_gi::shader::reproject_screen_radiance::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_gi::shader::reproject_screen_radiance::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xccca70 -> 0x1cf9b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_post::shader::blur::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 48a1bad7e950c9707a4f2197
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_post::shader::blur::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_post::shader::blur::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_post::blur_pyramid::render_blur_pyramid @ 0xcae9d4 -> 0x1cbad20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource::Handle::concurrent_mut<rhapsody::raw::texture::Texture,2,tuple$<>,rhapsody_post::blur_pyramid::render_blur_pyramid::closure$0::closure_env$0> @ 0x1cbb74f -> 0x1cbad20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_post::shader::rev_blur::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 3e162174b219d61c10d430f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_post::shader::rev_blur::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_post::shader::rev_blur::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_post::blur_pyramid::render_bidirectional_blur_pyramid @ 0xcaef02 -> 0x1cba740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_shadow::shader::setup_shadows::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 2e92ebccc0ce04942d93af82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_shadow::shader::setup_shadows::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_shadow::shader::setup_shadows::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_shadow::setup_directional_light_cascades @ 0xca97c7 -> 0x1cb1d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::taa::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: dfb9f111e86d72a3110721a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::taa::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::taa::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_taa @ 0x6f174a -> 0x10e4450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: b1d4456ad6dabe0091251e29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::tgsr2::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::tgsr2::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2ded -> 0x10e3240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2_decide::decide<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 2704ea330b3067509e5e7f39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::tgsr2_decide::decide -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::tgsr2_decide::decide<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2c74 -> 0x10e3850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 8c24be6f99177ddc244b218d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::tgsr::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::tgsr::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f2185 -> 0x10e2600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_blur_lum::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: e583d35df2581373d567db43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::tgsr_blur_lum::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::tgsr_blur_lum::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1d97 -> 0x10e3e90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2894 -> 0x10e3e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_reproject::main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 9284829018eb75bc1b6db77d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody_taa::shader::tgsr_reproject::main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point -> ["rhapsody_taa::shader::tgsr_reproject::main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1f7f -> 0x10e2c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f29ba -> 0x10e2c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
// 信息 ID: 1bb2cceb2db854fe3e5abdaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 162 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x3f7a9c -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcb24 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcc9d -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcfbf -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bce3f -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1a80 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1891 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1985 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x69f822 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x69fdb2 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x6a0342 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster> @ 0x6a08d2 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster> @ 0x6a0e62 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster> @ 0x6a13f2 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster> @ 0x6a1982 -> 0xe24020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_taa @ 0x6f177a -> 0xe24020 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_indirect
// 信息 ID: d835d7e37545ad91c7ab45e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect @ 0x9bbcc9 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect @ 0x9bbeae -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat @ 0xa8e83c -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat @ 0xa8e951 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa91722 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa9183b -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_scan_and_header @ 0xa91cab -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_scan_and_header @ 0xa91d63 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_scan_and_header @ 0xa91dfc -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0xaea0be -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0xaea1f2 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0xaeae6f -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce790 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce8b1 -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcce9ca -> 0xe24850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_dof::render_dof_with_coc @ 0xcceae3 -> 0xe24850 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
// 信息 ID: e9c7808143b557f99293fc32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 192 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x3f7a18 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bc9ee -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcbf8 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bca98 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcf17 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcdac -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1a01 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1806 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::capture_tgsr_frame @ 0x5b1900 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x69f7a4 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x69fd34 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x6a02c4 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster> @ 0x6a0854 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster> @ 0x6a0de4 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster> @ 0x6a1374 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster> @ 0x6a1904 -> 0xe249d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure$0::closure$0
// 信息 ID: 77241407c91a4851d8ba28b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0> @ 0xe741e4 -> 0xe71350 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0> @ 0xe742a6 -> 0xe71350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
