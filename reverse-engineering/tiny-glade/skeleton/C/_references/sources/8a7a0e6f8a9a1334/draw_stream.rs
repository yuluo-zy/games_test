// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/8a7a0e6f8a9a1334/draw_stream.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/draw_stream.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: draw_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline
// 信息 ID: 4a4895b94e52e1097c1c3abb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae3419 -> 0xe1ca20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf6718 -> 0xe1ca20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d115 -> 0xe1ca20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d519 -> 0xe1ca20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<clutter_icon_pipe::shader::clutter_icon_bake::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 651dbd52987158295b8768b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] clutter_icon_pipe::shader::clutter_icon_bake::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args -> ["clutter_icon_pipe::shader::clutter_icon_bake::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> @ 0x26ebd45 -> 0x2ab20f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::ice_plane::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: f73d60b22936ca3ff84e4bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::ice_plane::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args -> ["country_core::shader::ice_plane::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x17687d5 -> 0x19818e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: b0a06c6603f8b7f85f20aa76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water_plane::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args -> ["country_core::shader::water_plane::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fb6d -> 0x197ee20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane_refraction_copy::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 58c8396b054dc75143556291
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::water_plane_refraction_copy::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args -> ["country_core::shader::water_plane_refraction_copy::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x1763377 -> 0x19804b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::set_pipeline_impl
// 信息 ID: e860bc333ac2b3f5e2c9ee74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 35 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x233d3e -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2351ee -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x23669d -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline @ 0xe1ca23 -> 0xe1ca30 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader_with_compatible_args<country_core::shader::deferred::solid_light_sun_sky::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197d989 -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197ee68 -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane_refraction_copy::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19804ed -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::ice_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x198191d -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::card_splat_resolve::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1982e1a -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::logo_overlay::vs_main> @ 0x19842fc -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::resolve_msaa_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x198485e -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::lut_gallery::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1985cee -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::thumbnail_painterly::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19871ce -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::velocity_reconstruct::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19886dd -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch_decode_srgb::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1989b6d -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x198affe -> 0xe1ca30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 6ed13e84df7dbbb6e33cf329
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blit::blit_stretch::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> @ 0x45370a -> 0x236600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch_decode_srgb::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 02bc2b5526597505b2eac356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blit::blit_stretch_decode_srgb::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::blit::blit_stretch_decode_srgb::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::menu_background::render_menu_bg::closure_env$0> | rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg::closure_env$0> @ 0x17537f0 -> 0x1989ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::resolve_msaa_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 759a6ab4063345a62d1dcdd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blit::resolve_msaa_to_swapchain::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::blit::resolve_msaa_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0> @ 0x17575f5 -> 0x19847c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::resolve_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 649247d1c76bf08f85ea9185
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::blit::resolve_to_swapchain::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::blit::resolve_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0> @ 0x17575b1 -> 0x1996ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::custom_cursor::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 626bdb1dd2b93a7e03701ab5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::custom_cursor::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::custom_cursor::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0> @ 0x1767520 -> 0x1994170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 28b4d2477c33bb52efd45171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::egui::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> @ 0x452807 -> 0x235150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::final_post::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: ee3a5a67387051bc917b9bba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::final_post::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::final_post::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure$1::closure_env$0> @ 0x175648a -> 0x1992c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::card_splat_resolve::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: fb9c7f6ba29ecba4aea66b9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_resolve::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass2::card_splat_resolve::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::grass2::grass2_card_splat::closure$0::closure$1::closure_env$0> @ 0x1763e59 -> 0x1982d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::card_splat_shadow_resolve::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 0b9cfa798e22d31893c342d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::card_splat_shadow_resolve::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass2::card_splat_shadow_resolve::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment>,country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass::closure$0::closure$0::closure_env$1> @ 0x1765a8f -> 0x198d880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::debug_cull_sphere::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 3bcb378c35980e4f4d12480f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::debug_cull_sphere::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass2::debug_cull_sphere::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::grass2::grass2_debug_overlays::closure$0::closure_env$0> @ 0x1757b8d -> 0x199ae60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::debug_hiz_tiles::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: f4745eea9ba99794c8362093
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::debug_hiz_tiles::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass2::debug_hiz_tiles::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::compute_passes::grass2::grass2_debug_overlays::closure$0::closure_env$1> @ 0x1761d97 -> 0x199c8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::velocity_reconstruct::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: f139275a92503577a5824267
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::grass2::velocity_reconstruct::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass2::velocity_reconstruct::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::grass_velocity_reconstruct::reconstruct_grass_velocity::closure_env$0> @ 0x1764f43 -> 0x1988640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass_object_push::vs_main>
// 信息 ID: 85710876cbd678cc0b0398ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::grass_object_push::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$2> @ 0x1756db9 -> 0x19927e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::logo_overlay::vs_main>
// 信息 ID: 16d9b68c63914fe254203d07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::logo_overlay::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay::closure_env$0> @ 0x1766212 -> 0x1984270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::lut_gallery::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 659ca07dde109ce4b3d6bf57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::lut_gallery::thumbnail::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::lut_gallery::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$1> @ 0x175865a -> 0x1985c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::mei_shadow::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: aeb10e71fb1586a6f93dd49a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::mei_shadow::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::mei_shadow::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1> @ 0x17542f7 -> 0x19999d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::painterly::raster_strokes::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: fdf91172c91d8df74231450d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::painterly::raster_strokes::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::painterly::raster_strokes::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> @ 0x1754a03 -> 0x1995610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::background::vs_main>
// 信息 ID: 49b03ba4c5012f2281982b87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::session_selection::background::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$2> @ 0x175b75f -> 0x198f360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::new_glade_thumbnail::vs_main>
// 信息 ID: 5632fe825287b9c11435894a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::session_selection::new_glade_thumbnail::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$2> @ 0x175b957 -> 0x199c2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 8dec5bf5eb318535ad3e642f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::session_selection::thumbnail::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::session_selection::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$2> @ 0x175bb25 -> 0x198af60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::thumbnail_painterly::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: c8bb550731f7890754598191
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::session_selection::thumbnail_painterly::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::session_selection::thumbnail_painterly::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$2> @ 0x175be13 -> 0x1987130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: f3906516423d639ce22e7ca0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::shadow_receiver_frustum::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> @ 0x45315c -> 0x233ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::stairs_foliage_exclusion::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: c89aecf2db7bf9a70f57f472
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::stairs_foliage_exclusion::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::stairs_foliage_exclusion::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> @ 0x17550da -> 0x198c460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::text::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: e6022a0bc0614366637e97f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::text::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::text::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0> @ 0x1760ce3 -> 0x198f8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::ui_2d::vs_main>
// 信息 ID: 1f165ed956170c239481f348
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::ui_2d::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> @ 0x175a764 -> 0x198ed80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::ui_2d_color_button::vs_main>
// 信息 ID: d1f94733e991957026cf3517
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::ui_2d_color_button::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> @ 0x175a553 -> 0x1992230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::ui_2d_ws::vs_main>
// 信息 ID: bfc7510b86fb84dd4c1505fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::ui_2d_ws::vs_main"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> @ 0x175a973 -> 0x1997f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::vermeer::combine::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 078940a64fc8aff2fcc141ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::combine::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::vermeer::combine::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::vermeer::raster_combine::closure_env$0> @ 0x176443a -> 0x1990db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::vermeer::raster_strokes::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: ecf8ff735de06584ef81074f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::vermeer::raster_strokes::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader -> ["country_core::shader::vermeer::raster_strokes::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::vermeer::raster_strokes::closure_env$0> @ 0x1762a2c -> 0x1998510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader_with_compatible_args<country_core::shader::deferred::solid_light_sun_sky::vs_main<rhapsody::rg::binding::RgProtocol> >
// 信息 ID: 2de78d797bd5f8f05b8f823a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::deferred::solid_light_sun_sky::vs_main -> ["rhapsody::rg::binding::RgProtocol"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader_with_compatible_args -> ["country_core::shader::deferred::solid_light_sun_sky::vs_main<rhapsody::rg::binding::RgProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster::closure$0::closure_env$0> @ 0x175e6ea -> 0x197d8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster::closure$0::closure_env$0> @ 0x175e83e -> 0x197d8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster::closure$0::closure_env$0> @ 0x175e993 -> 0x197d8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster::closure$0::closure_env$0> @ 0x175eaee -> 0x197d8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster::closure$0::closure_env$0> @ 0x175ec54 -> 0x197d8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::bind_sets_from_stack
// 信息 ID: 2576c8e9b5e3f7f5fd11d5e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x233d49 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2351f9 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2366a8 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d125 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d529 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader_with_compatible_args<country_core::shader::deferred::solid_light_sun_sky::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197d994 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x197ee7b -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::water_plane_refraction_copy::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19804f8 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::prebuilt_pipeline_with_compatible_args<country_core::shader::ice_plane::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1981928 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::card_splat_resolve::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1982e25 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::logo_overlay::vs_main> @ 0x1984307 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::resolve_msaa_to_swapchain::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1984869 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::lut_gallery::thumbnail::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1985cf9 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::session_selection::thumbnail_painterly::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19871d9 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::grass2::velocity_reconstruct::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x19886e8 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch_decode_srgb::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x1989b78 -> 0xe202a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::bind_stack_sets
// 信息 ID: 4145ae49b5a736f7d4de7949
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae3444 -> 0xe1ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf672e -> 0xe1ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fbb1 -> 0xe1ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x17633ac -> 0xe1ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768819 -> 0xe1ff00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::binding_set
// 信息 ID: 9148c1a295bc52cd77e945fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae345a -> 0xe1f0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf6744 -> 0xe1f0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d1d9 -> 0xe1f0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d545 -> 0xe1f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw
// 信息 ID: 689afb141c9f422b524633b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 53 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> @ 0x45283a -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> @ 0x453199 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> @ 0x453728 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::line_draw_fn @ 0x9c11d2 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::dashed_draw_fn @ 0x9c169f -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::thick_line_draw_fn @ 0x9c3216 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae34c0 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_ivy::startup_ivy::draw_fn @ 0xaf5593 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf67b0 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf69be -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::menu_background::render_menu_bg::closure_env$0> | rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg::closure_env$0> @ 0x175380e -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1> @ 0x1754315 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> @ 0x1754a2a -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> @ 0x17550f5 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure$1::closure_env$0> @ 0x17564a8 -> 0xe20590 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$2> @ 0x1756e53 -> 0xe20590 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect
// 信息 ID: c0f92ae668a7484855369efa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_nani::coarse_sorted_draw_fn @ 0x9f0e43 -> 0xe1fdb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn @ 0xae2fda -> 0xe1fdb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect_count
// 信息 ID: 072a7ec7f6f3f5fe365e1518
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_nani::fine_sorted_draw_fn @ 0x9f08c4 -> 0xe200a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::pipeline_default_cull_mode
// 信息 ID: 712067f3afe4a58980f9b1c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::line_draw_fn @ 0x9c10f1 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::dashed_draw_fn @ 0x9c14d1 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::thick_line_draw_fn @ 0x9c2c05 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::fine_sorted_draw_fn @ 0x9f085d -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::coarse_sorted_draw_fn @ 0x9f0e2b -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn @ 0xae2fbf -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae36aa -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_ivy::startup_ivy::draw_fn @ 0xaf5581 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf699d -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d1c0 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fdf8 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x17635ec -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768a69 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo> @ 0x1ad11d2 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roof_tile_ssbo::RoofSpireSsbo> @ 0x1ad1582 -> 0xe20550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> @ 0x1ad1932 -> 0xe20550 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers<ref$<alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> > >
// 信息 ID: 6403a2178f9fe8eb93b7c555
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::vertex_buffers -> ["ref$<alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 19 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::line_draw_fn @ 0x9c1095 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::dashed_draw_fn @ 0x9c1475 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::thick_line_draw_fn @ 0x9c2ba5 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::fine_sorted_draw_fn @ 0x9f0801 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::coarse_sorted_draw_fn @ 0x9f0ddb -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn @ 0xae2f63 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae365c -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_ivy::startup_ivy::draw_fn @ 0xaf5526 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf6955 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d13b -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fd9f -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x1763594 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768a15 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo> @ 0x1ad1177 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roof_tile_ssbo::RoofSpireSsbo> @ 0x1ad1527 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> @ 0x1ad18d7 -> 0x199ddf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::RecordedRasterStream::render
// 信息 ID: d48987d6b7f1755a81981df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> > | rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0> > | rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> > @ 0x40be28 -> 0xe1cc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::impl$2::into_index_buffer_ref
// 信息 ID: cd0f4f4164d6ff4e4182f6b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> @ 0x452816 -> 0xe2f200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> @ 0x453172 -> 0xe2f200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::impl$5::into_index_buffer_ref
// 信息 ID: a2959cdf4e7378799cc489cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::line_draw_fn @ 0x9c10a4 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::dashed_draw_fn @ 0x9c1484 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::thick_line_draw_fn @ 0x9c2bb4 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::fine_sorted_draw_fn @ 0x9f0810 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::coarse_sorted_draw_fn @ 0x9f0dea -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::indexed_indirect::draw_fn @ 0xae2f72 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae366c -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_ivy::startup_ivy::draw_fn @ 0xaf5535 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::render_simple_bucket @ 0xaf6965 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175d152 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fdb6 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x17635ab -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768a2c -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo> @ 0x1ad1186 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::resources::roof_tile_ssbo::RoofSpireSsbo> @ 0x1ad1536 -> 0xe12530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::impl$0::instanced_indexed::draw_fn<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> @ 0x1ad18e6 -> 0xe12530 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::impl$9::bind_stack_sets::closure$0
// 信息 ID: f535137d7c009c2c1c681a34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::node_builder::draw_stream::impl$9::bind_stack_sets", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::bind_stack_sets @ 0xe2005a -> 0x2da36b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::packet::DrawStream::with_capacity
// 信息 ID: 3fe981ccd9d82f70e602f037
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::GraphRecord::new_with_capacity @ 0xe72a9f -> 0xe20620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::packet::DrawStreamRecorder::submit_draw
// 信息 ID: 3bffe3dee844b4990deb5dc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect @ 0xe1fea8 -> 0xe20790 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw_indirect_count @ 0xe20237 -> 0xe20790 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::draw @ 0xe205e1 -> 0xe20790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::draw_stream::packet::impl$9::default
// 信息 ID: 187956875e5d8c5449524f20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 48 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> @ 0x452637 -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$2> | rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$5> @ 0x452df7 -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> @ 0x453043 -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> @ 0x45362f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::menu_background::render_menu_bg::closure_env$0> | rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg::closure_env$0> @ 0x175371f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3> @ 0x1753c9f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1> @ 0x175421f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> @ 0x17547dd -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> @ 0x1754fff -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3> @ 0x175558f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1> @ 0x1755b0f -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure$1::closure_env$0> @ 0x17560eb -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$2> @ 0x1756cc7 -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0> @ 0x17574b0 -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::grass2::grass2_debug_overlays::closure$0::closure_env$0> @ 0x1757aaf -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$1> @ 0x175808c -> 0xe1a1f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
