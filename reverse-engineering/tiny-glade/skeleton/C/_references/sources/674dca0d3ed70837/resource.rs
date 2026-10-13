// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/674dca0d3ed70837/resource.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: resource.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::BufferDesc::new
// 信息 ID: 44136b549d7ad455f3c12fe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 44 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9b2cca -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m_indirect @ 0x9bbcf8 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bda6b -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bdad9 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bdcd7 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bdd4d -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc3f4 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc44c -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9bed95 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9bee31 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9beea9 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x9bf390 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0xa8d8be -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0xa8d913 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0xa8d3a4 -> 0xe20f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0xa8d4dd -> 0xe20f40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::ResourceOrigin::name
// 信息 ID: f8ab2a64970e103197aa1b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 44 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x4544d6 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x454709 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x45489e -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x4549fc -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454b7a -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454d35 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0xe270a4 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0xe27353 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e15d7 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e177e -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x10e18fe -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x10e1a56 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x176ae6a -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x176b025 -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x176b19e -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x176b2fc -> 0xe20fa0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::TexView::layer
// 信息 ID: c1b3c9749ba9907a90fb2ec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x451891 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x45179d -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] arrayvec::arrayvec::impl$26::from_iter<rhapsody::rg::render_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment> >,rhapsody::rg::node_builder::draw_stream::impl$1::create_rg_raster_node::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1> > > > @ 0x59dd3f -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1751d34 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1751c40 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > @ 0x1752651 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > @ 0x175255d -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1752f8e -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1752e9a -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175c695 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175c5a2 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1767d70 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1767c77 -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] arrayvec::arrayvec::impl$26::from_iter<rhapsody::rg::render_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment> >,rhapsody::rg::node_builder::draw_stream::impl$1::create_rg_raster_node::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > > > @ 0x1a6aa4f -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] arrayvec::arrayvec::impl$26::from_iter<rhapsody::rg::render_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment> >,rhapsody::rg::node_builder::draw_stream::impl$1::create_rg_raster_node::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > > > @ 0x26e3a7f -> 0xe21160 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> @ 0x26eaab2 -> 0xe21160 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::TexView::with_layer
// 信息 ID: da50a77d9f41a2f288f4400b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x4518a4 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x4517b0 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56bc9c -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] arrayvec::arrayvec::impl$26::from_iter<rhapsody::rg::render_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment> >,rhapsody::rg::node_builder::draw_stream::impl$1::create_rg_raster_node::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1> > > > @ 0x59dd52 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9b3187 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa911ae -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa911f3 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0xae8f3b -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0xae9065 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1751d47 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1751c53 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > @ 0x1752664 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > @ 0x1752570 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1752fa1 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x1752ead -> 0xe210d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::compute_passes::read_terrain_data::raster_visual_terrain_and_read_terrain_data::closure$0::closure_env$0> @ 0x175c6a8 -> 0xe210d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::TexView::with_stencil_aspect
// 信息 ID: cb76ad49a11c590b62a0ff49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass @ 0xae9177 -> 0xe21150 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows @ 0xaecab3 -> 0xe21150 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals @ 0xaed4cb -> 0xe21150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::TexView::with_ty_override
// 信息 ID: 5c9d4dfbe9ba4f0848bc8388
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9b3190 -> 0xe21130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9bc7c9 -> 0xe21130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa911b7 -> 0xe21130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass2::grass2_splat_shadow_cascade @ 0xa911fc -> 0xe21130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::impl$107::fmt
// 信息 ID: 65b6028e7337f4efe9d8735b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::impl$125::fmt
// 信息 ID: 940c2c443c6e19a66ea90ce0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::impl$1::init_as_debug_garbage
// 信息 ID: 217f09b87bbb0bf9500c80e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 77 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x395c77 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x399594 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::impl$6::create<rhapsody::rg::resource::TextureDesc> @ 0x4515e2 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bc925 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b0b2 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b17f -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56bac0 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_taa @ 0x6f1641 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1991 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1b13 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1be3 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1cd1 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1e53 -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f23da -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f255f -> 0xe14680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2631 -> 0xe14680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::impl$61::fmt
// 信息 ID: 52d2a8f5c09ad1024082dd7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource::impl$79::fmt
// 信息 ID: d5ef44cb9e55b4e26040ceb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
