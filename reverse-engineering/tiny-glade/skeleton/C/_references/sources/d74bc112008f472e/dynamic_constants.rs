// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/d74bc112008f472e/dynamic_constants.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/dynamic_constants.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: dynamic_constants.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dynamic_constants.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dynamic_constants.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate
// 信息 ID: ea3acee4dc673de9dad126fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 232 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x233ef7 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2353c4 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x236839 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x453ce3 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::buffer_from_slice<epaint::mesh::Vertex> @ 0x45403a -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::buffer_from_slice<u32> @ 0x454246 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x454f94 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x45549c -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x455964 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455f57 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x45649f -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x456984 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::frame::FrameCounter,rhapsody_core::shader::frame::FrameCounter> | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::current_view::CurrentView,country_core::shader::inc::current_view::CurrentView> @ 0x456e37 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x457304 -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x45780c -> 0xe21510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::frame::FrameConstants,country_core::shader::inc::frame::FrameConstants> @ 0x457d4a -> 0xe21510 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
// 信息 ID: f0e9c2aabebef5ad4b61d85b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::new @ 0xe21425 -> 0xe21180 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate @ 0xe215b7 -> 0xe21180 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::rcache::RenderResourceCache::new @ 0xe716ed -> 0xe21180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::new
// 信息 ID: b5092878acb819ce71788748
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe57a3c -> 0xe21400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::to_uniform_buffer_binding
// 信息 ID: 8d539623db6052997fd6e954
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 178 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x233f50 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x235414 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x236892 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x4771ee -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x4777d4 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x477dcb -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x4783a4 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478997 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> > @ 0x478f68 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_ref::main<rhapsody::rg::binding::RgProtocol> > @ 0x479553 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_native::main<rhapsody::rg::binding::RgProtocol> > @ 0x479b43 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e2809 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_reproject::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e2e25 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e3436 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr2_decide::decide<rhapsody::rg::binding::RgProtocol> > @ 0x10e3a54 -> 0xe21720 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody_taa::shader::tgsr_blur_lum::main<rhapsody::rg::binding::RgProtocol> > @ 0x10e4041 -> 0xe21720 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
