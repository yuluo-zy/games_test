// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/d5a35b4d5c5b3074/recorder.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/recorder.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: recorder.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: recorder.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: recorder.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::BindingStream::extend
// 信息 ID: 3c5c8e47c6202eb820f08a9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 165 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x23490a -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x235dca -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x23724a -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x4773c0 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x4779b0 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x477fa0 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x478580 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478b70 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> > @ 0x47913f -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol> > @ 0x479730 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol> > @ 0x479d20 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody::rg::binding::BindDynamic> @ 0xe24510 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set_from_stack @ 0xe278c6 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e29e0 -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_reproject::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e2ffc -> 0xe73c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e360c -> 0xe73c40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::BindingStream::range_from
// 信息 ID: b450e5efe30c06ca7a94355b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 129 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x4773ef -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x4779df -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x477fcf -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x4785af -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478b9f -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> > @ 0x47916e -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol> > @ 0x47975f -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol> > @ 0x479d4f -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e2a09 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_reproject::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e3025 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e3635 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2_decide::decide<rhapsody::rg::binding::RgProtocol> > @ 0x10e3c59 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_blur_lum::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e422e -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::taa::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e4835 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::wall::generate_draw_lists::cs_pass1<rhapsody::rg::binding::RgProtocol> > @ 0x1778514 -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::nani::generate_dispatch_args::main<rhapsody::rg::binding::RgProtocol> > @ 0x1778aaf -> 0xe73bd0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::import<rhapsody::rg::import_export::impl$1::import::closure_env$0>
// 信息 ID: 1ce91b85c2a4616b51989a08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::GraphRecord::import -> ["rhapsody::rg::import_export::impl$1::import::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::import_export::impl$1::import @ 0xe19ae1 -> 0xe737a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::binding_set @ 0xe1f730 -> 0xe737a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::import_binding_set @ 0xe2685e -> 0xe737a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::import<rhapsody::rg::import_export::impl$2::import::closure_env$0>
// 信息 ID: bcc045681a33f71174fdc46b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::GraphRecord::import -> ["rhapsody::rg::import_export::impl$2::import::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::import_export::impl$2::import @ 0xe195c2 -> 0xe73370 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline::binding_set @ 0xe1f435 -> 0xe73370 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::import_binding_set @ 0xe2660e -> 0xe73370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::import_mut<rhapsody::rg::import_export::impl$1::import_mut::closure_env$0>
// 信息 ID: 6775426c88eacdd656225975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::GraphRecord::import_mut -> ["rhapsody::rg::import_export::impl$1::import_mut::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::import_export::impl$1::import_mut @ 0xe19791 -> 0xe71ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::import_mut<rhapsody::rg::import_export::impl$2::import_mut::closure_env$0>
// 信息 ID: e79295a9ae3c9afa29c6b1b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::GraphRecord::import_mut -> ["rhapsody::rg::import_export::impl$2::import_mut::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::import_export::impl$2::import_mut @ 0xe19312 -> 0xe72110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::new_with_capacity
// 信息 ID: dddf52c1a68c2e4f5dae62c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x395723 -> 0xe72580 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3981b5 -> 0xe72580 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,rhapsody::rg::executor::impl$12::present::closure$0::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > > @ 0x39cf4d -> 0xe72580 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x1772333 -> 0xe72580 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e6646 -> 0xe72580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::GraphRecord::register_binding_set_node_resource_access
// 信息 ID: 250c3e6d73208a2ba5646e62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 31 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x453c79 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x454f36 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x45543e -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x455906 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455ef8 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x456440 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x456926 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::frame::FrameCounter,rhapsody_core::shader::frame::FrameCounter> | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::current_view::CurrentView,country_core::shader::inc::current_view::CurrentView> @ 0x456deb -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x4572a6 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x4577ae -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::frame::FrameConstants,country_core::shader::inc::frame::FrameConstants> @ 0x457cdf -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<rhapsody::rg::binding::RgProtocol> > @ 0x458206 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::flag::FlagTextures<rhapsody::rg::binding::RgProtocol> > @ 0x45870e -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::roof_color_data::RoofColors<rhapsody::rg::binding::RgProtocol> > @ 0x458bfe -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set_from_stack @ 0xe27907 -> 0xe73220 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x176a289 -> 0xe73220 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::api::impl$4::init_texture_as_debug_garbage::closure_env$0>
// 信息 ID: a9be9a3b4f582d2a86c968dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::api::impl$4::init_texture_as_debug_garbage::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::api::impl$7::clear_buffer::closure_env$0>
// 信息 ID: b7b2c36e554249660aca1a51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::api::impl$7::clear_buffer::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::api::impl$7::clear_color_texture::closure_env$0>
// 信息 ID: 0b08255ea6660d02ce7c3bfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::api::impl$7::clear_color_texture::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::api::impl$7::clear_color_texture_u32::closure_env$0>
// 信息 ID: 876f549134135e0953a82d75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::api::impl$7::clear_color_texture_u32::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0>
// 信息 ID: 96f5ad6b541bfa13ab22407a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1> >
// 信息 ID: 1ef95929e5193f473088ec62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment"]; 字段布局 = unknown。
// [类型关系] array$ -> ["enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>", "4"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>", "enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment>"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0 -> ["4", "tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >", "clutter_icon_pipe::render_icon::closure$0::closure_env$1"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::recorder::impl$8::new_in::render_impl -> ["rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<4,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,4>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,clutter_icon_pipe::render_icon::closure$0::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::recorder::impl$9::drop
// 信息 ID: ac51c6be74cadded1d8b4e9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<typed_index_collections::vec::TiVec<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> > | core::ptr::drop_in_place<typed_index_collections::vec::TiVec<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> > | core::ptr::drop_in_place<typed_index_collections::vec::TiVec<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> > @ 0x8b925 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::api::GraphApi> @ 0x9cf65 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::compiled::CompiledGraph> @ 0xa7ff5 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> | core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> | core::ptr::drop_in_place<rhapsody::rg::node_builder::NodeBuilder> @ 0xab827 -> 0xe7b460 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0xbd1ca -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$2> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$5> > | 其余 1 个同地址别名见共享索引 @ 0xf4df7 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::egui_backend::impl$0::draw::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::render_world::closure$6::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$2> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$5> > | 其余 1 个同地址别名见共享索引 @ 0xf4e31 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::api::GraphApi> @ 0x98e185 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x9a151a -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> @ 0x10e252a -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass::closure_env$0> > | 其余 40 个同地址别名见共享索引 @ 0x18048f7 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::painterly::raster_strokes::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask::closure_env$0> > | rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass::closure_env$0> > | 其余 40 个同地址别名见共享索引 @ 0x1804931 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::readback_draw_list_lengths::closure$0::closure_env$0> @ 0x1804b40 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::readback_draw_list_lengths::closure$0::closure_env$0> @ 0x1804b6e -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::buffer_copy_from_slice::closure_env$0<u32> > @ 0x1804cf8 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::api::impl$7::buffer_copy_from_slice::closure_env$0<u32> > @ 0x1804d26 -> 0xe7b460 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
