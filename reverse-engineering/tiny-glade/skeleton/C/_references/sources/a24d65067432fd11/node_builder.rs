// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/a24d65067432fd11/node_builder.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: node_builder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: node_builder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: node_builder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] u32 (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$4::init_texture_as_debug_garbage::closure_env$0>
// 信息 ID: d191afb849e532135b1d6cde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::api::impl$4::init_texture_as_debug_garbage::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage @ 0xe2865c -> 0xe33dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::clear_buffer::closure_env$0>
// 信息 ID: a8686b8b7d56131cfe2b1aea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::api::impl$7::clear_buffer::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::clear_buffer @ 0xe2526e -> 0xe341a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::clear_color_texture::closure_env$0>
// 信息 ID: 76bc98ae350766fccb8b888d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::api::impl$7::clear_color_texture::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::clear_color_texture @ 0xe26c7e -> 0xe34290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::clear_color_texture_f32::closure_env$0>
// 信息 ID: 9a44a86af00bb3b328a70e6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::api::impl$7::clear_color_texture_f32::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::clear_color_texture_f32 @ 0xe26f57 -> 0xe340b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0>
// 信息 ID: d415c66968608ac87769169e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl @ 0xe240c2 -> 0xe33ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_indirect @ 0xe2495b -> 0xe33ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe2589c -> 0xe33ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage @ 0xe2835a -> 0xe33ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> >
// 信息 ID: fbc5bb30967e0961983bda27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>", "4"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>", "enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment>"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0 -> ["4", "tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >", "clutter_icon_pipe::render_icon::closure$0::closure_env$1"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::NodeBuilder::set_render_callback -> ["rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> @ 0x26ebfe7 -> 0x2ab0760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeResourceAccessType::merge
// 信息 ID: e538bf3167bc327922bbf66f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 54 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x4545c2 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x4545ef -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x45467a -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x454689 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x45469b -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x4546ac -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x45499d -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454c4a -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454cb9 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454cc7 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e169b -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e1702 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e1711 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x10e19f7 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x176af3a -> 0xe343a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x176afa9 -> 0xe343a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::NodeResourceAccessType::new
// 信息 ID: 345ced5aa624c8d450218d5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x4545b0 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x454945 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x454c37 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x10e1690 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x10e19a2 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x176af27 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x176b245 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<4> @ 0x176b560 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x1cb05f0 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x1cb0902 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x1cba000 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x1cba312 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x1cf60d0 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x1cf63e2 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<1> @ 0x1cfd692 -> 0xe34380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_resource_access_impl<2> @ 0x1cfd970 -> 0xe34380 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::impl$0::drop
// 信息 ID: cc496969d31413677663bc8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> | core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> | core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> @ 0xab806 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0xbd1a6 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$2> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$5> > | 其余 1 个同地址别名见共享索引 @ 0xf4e0f -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x9a14f6 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x10e2506 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass::closure_env$0> > | 其余 40 个同地址别名见共享索引 @ 0x180490f -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::readback_draw_list_lengths::closure$0::closure_env$0> @ 0x1804b52 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::buffer_copy_from_slice::closure_env$0<u32> > @ 0x1804d0a -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure$0::closure_env$0> @ 0x1804ef7 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> | core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x1cba646 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x1cf82d6 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x1cfe736 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x1d56466 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x1e4da96 -> 0xe320d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> > @ 0x2ab089f -> 0xe320d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::node_builder::impl$8::fmt
// 信息 ID: de9f3eacf0b1a6436005ffde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
