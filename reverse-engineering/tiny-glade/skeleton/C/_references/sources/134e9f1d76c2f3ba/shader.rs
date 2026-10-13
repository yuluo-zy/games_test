// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/134e9f1d76c2f3ba/shader.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/shader.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: shader.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools<core::iter::sources::once::Once<ash::vk::definitions::DescriptorPool> >
// 信息 ID: e040c4ae71084c406cb5d938
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::iter::sources::once::Once -> ["ash::vk::definitions::DescriptorPool"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools -> ["core::iter::sources::once::Once<ash::vk::definitions::DescriptorPool>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::Queue::reclaim_command_buffer_resources @ 0xe5a1c7 -> 0xe37c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
// 信息 ID: 233cecef9f9a5380e3fa1b3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$1> > @ 0xe5d128 -> 0xe37610 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0> @ 0xe7433a -> 0xe37610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layout<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>,core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > > >
// 信息 ID: 5e4a9e58a3403d740350cc3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["shaderface::reflection::Binding"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["shaderface::reflection::BindingIndex", "ref$<shaderface::reflection::Binding>"]; 字段布局 = unknown。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["shaderface::reflection::SamplerState"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["shaderface::reflection::BindingIndex", "ref$<shaderface::reflection::SamplerState>"]; 字段布局 = unknown。
// [类型关系] core::iter::sources::empty::Empty -> ["tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::create_descriptor_set_layout -> ["alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>", "core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_core::shader::view::ViewConstants> > @ 0x1ac5a22 -> 0x1d66600 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_core::shader::static_samplers::Samplers> > @ 0x1ac6c92 -> 0x1d66600 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_shadow::shader::shadow::CascadedShadowSetup<shabriri_rt::api::EmptyProtocol> > > @ 0x2660864 -> 0x2a820d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_core::shader::view::ViewConstants> > @ 0x2661c24 -> 0x2a820d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layouts
// 信息 ID: 5bb0057ec539fca59d763dfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x5cefae -> 0xe37da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe5b526 -> 0xe37da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x15e484e -> 0xe37da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x1ac7f7e -> 0xe37da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states
// 信息 ID: f23cef0591cfce3caf3f55d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x5d00a2 -> 0xe37d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x15e5942 -> 0xe37d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x1ac9072 -> 0xe37d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::get_vk_blend_mode
// 信息 ID: 81a80a36125afc88dde502bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x5cfbc9 -> 0xe375b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x15e5469 -> 0xe375b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x1ac8b99 -> 0xe375b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$12::from
// 信息 ID: 889a5f2d561041267155a54d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists @ 0xae4c6f -> 0xe38c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$14::default
// 信息 ID: b75518390cb71ce3115bdfaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,tiny_glade::render_loop::compile_main_render_graph::closure$0::closure_env$6> @ 0x4536ec -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefe71 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0> @ 0x1757596 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env$0> @ 0x17575da -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259ede3 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a6303 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f5df8 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc0f2 -> 0xe36cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$3::destroy_resource
// 信息 ID: 3904bf991e760a71603369d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::shader::ComputePipelineSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6b346 -> 0xe760a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$4::deref
// 信息 ID: f48d7a245af93a1308994eb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 387 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x477269 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x4774ec -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::capture_panic_screenshot::main<rhapsody::rg::binding::RgProtocol> > @ 0x477497 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x47784f -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x477adc -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::main<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::blur_downsample::cs_main_multisampled<rhapsody::rg::binding::RgProtocol> > @ 0x477a87 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x477e46 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x4780cc -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_buffer::sample4<rhapsody::rg::binding::RgProtocol> > @ 0x478077 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x47841f -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x4786ac -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::tgsr_capture_hires::main<rhapsody::rg::binding::RgProtocol> > @ 0x478657 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478a12 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478c9c -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample4<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::screenshot::capture_screenshot_texture::sample1<rhapsody::rg::binding::RgProtocol> > @ 0x478c47 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<country_core::shader::upload_cpu_raster::main<rhapsody::rg::binding::RgProtocol> > @ 0x478fe3 -> 0xe6ac50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$5::deref
// 信息 ID: 7f80ee3ae61b9999903751cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::render_bucket @ 0xae3319 -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::vao_update::build_missing_vaos @ 0xc0e92a -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x175fa5f -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass::closure$0::closure_env$0> @ 0x17600af -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass::closure_env$0> @ 0x176331b -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768611 -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::raster<5,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,5>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass::closure_env$0> @ 0x1768dca -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure$1 @ 0x2d78798 -> 0xe6ac30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$6::destroy_resource
// 信息 ID: b5280c95c31191370a8053e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::shader::RasterPipelineSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6b095 -> 0xe75940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$8::create_compute_pipeline::closure$0<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: ddeeb9eb52529ded7148f029
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["rhapsody::raw::shader::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::impl$8::create_compute_pipeline::closure$0 -> ["alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::impl$8::create_compute_pipeline", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe5ba93 -> 0x2da46b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
