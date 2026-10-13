// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00799.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00799.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00799.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00799.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::GpuEpochTracker::current
// 信息 ID: fd425781b2c351c7b0f140b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 388 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::buffer::Buffer> @ 0x9f196 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::sampler::Sampler> @ 0xa1750 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::texture::Texture> @ 0xa1a29 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::RasterPipeline> @ 0xa7c26 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSet> @ 0xa9c9f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::render_pass::RenderPass> @ 0xaa1a0 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::ComputePipeline> @ 0xaa470 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::swapchain::FallibleSwapchain> @ 0xaff6f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSetLayout> @ 0xb17d8 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186dcb -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186ebf -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f87 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x187051 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x222070 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label_with_id<ref$<str$> > @ 0x223560 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<ref$<str$> > @ 0x223a5c -> 0xcb570 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::binding_set::BindingSetSchema>
// 信息 ID: e075c797e5af6b25a349012e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::binding_set::BindingSetSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_impl @ 0xe4cc52 -> 0xe6d6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::buffer::BufferSchema>
// 信息 ID: c43f30e6f969ac3810cb1fea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::buffer::BufferSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer_impl @ 0xe5038b -> 0xe6dad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::render_pass::RenderPassSchema>
// 信息 ID: bb6b83d1517ff7863ff38e3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::render_pass::RenderPassSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_render_pass @ 0xe4d344 -> 0xe6c820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::sampler::SamplerSchema>
// 信息 ID: 9c8c7e9d9db11142c3301bdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::sampler::SamplerSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::get_or_create_sampler @ 0xe5e1ca -> 0xe6cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::render_api::RenderApi::begin_render_pass @ 0xe6a79a -> 0xe6cf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::shader::ComputePipelineSchema>
// 信息 ID: 4d6e1caa1175fec4cc8df177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::shader::ComputePipelineSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe5b8af -> 0xe6e1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::swapchain::SwapchainSchema>
// 信息 ID: bb7cc1d35bebc626363bfef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::swapchain::SwapchainSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_swapchain @ 0xe64696 -> 0xe6d2c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::texture::TextureSchema>
// 信息 ID: d98373a0cafb2eba4f6f0cc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::texture::TextureSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::swapchain::impl$6::new_impl::closure_env$4>,rhapsody::raw::texture::Texture> @ 0xe43fd8 -> 0xe6e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe5f96a -> 0xe6e630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::binding_set::BindingSetLayoutSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: d7e968d04c543286f98e9e00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::binding_set::BindingSetLayoutSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358f8 -> 0xe6ac70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::binding_set::BindingSetSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: d8041056970acc3c1abf7d11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::binding_set::BindingSetSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe3590c -> 0xe6c570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::buffer::BufferSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: ffbb334e781cec59e9d6a96f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::buffer::BufferSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe35887 -> 0xe6c2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::framebuffer::FramebufferSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: 769cf8ae4328f4b15b7766f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::framebuffer::FramebufferSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358c5 -> 0xe6b780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::render_pass::RenderPassSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: c7c0a408f1f080e62b6f09aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::render_pass::RenderPassSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358a3 -> 0xe6bfd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::sampler::SamplerSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: 3679393b5a9e2c26e4f6e536
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::sampler::SamplerSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358b4 -> 0xe6ba40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::shader::ComputePipelineSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: 386eebf0cd443f1139933e1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::shader::ComputePipelineSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358d6 -> 0xe6b210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::shader::RasterPipelineSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: ed872654a8186c4245f6a67d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::shader::RasterPipelineSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe358e7 -> 0xe6af50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::swapchain::SwapchainSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: 0fa1de6dceafb9bd3a89e276
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::swapchain::SwapchainSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe35895 -> 0xe6b4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::texture::TextureSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> >
// 信息 ID: 48685fbc5ad9b9d85a21a6c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::disposal::DisposalBucket"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::respool::Pool::take_out_waste -> ["rhapsody::raw::texture::TextureSchema", "ref_mut$<rhapsody::raw::disposal::DisposalBucket>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe35879 -> 0xe6bd00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
