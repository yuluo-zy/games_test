// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/f031852c9140531d/shader_lib.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/shader_lib.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: shader_lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader_lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader_lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] u32 (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::compile_permutations
// 信息 ID: 023912a6a0014f93e3135b3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259e208 -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259d214 -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a5599 -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a4599 -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f50e4 -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f409f -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fb3bd -> 0xe40760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fa2bd -> 0xe40760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::compile_permutations_with_options
// 信息 ID: 2b6673e5f89c844f5d8075c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::compile_permutations @ 0xe40772 -> 0xe41a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::load_precompiled_shaders @ 0xe417ac -> 0xe41a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259ed1a -> 0xe41a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a6245 -> 0xe41a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f5d38 -> 0xe41a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc030 -> 0xe41a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::compile_pipeline
// 信息 ID: 0bfed4f1416dfda611259600
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::compile_permutations_with_options @ 0xe41f3c -> 0xe3f610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::get_compute<array$<tuple$<alloc::string::String,alloc::string::String>,1> >
// 信息 ID: 317db901c9813d44f9cba12e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] array$ -> ["tuple$<alloc::string::String,alloc::string::String>", "1"]; 字段布局 = unknown。
// [类型关系] rhapsody::shader_lib::ShaderLibrary::get_compute -> ["array$<tuple$<alloc::string::String,alloc::string::String>,1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage @ 0xe28156 -> 0xe3ed30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::get_compute<shaderface::permutation::ShaderPermutationKey>
// 信息 ID: bfe969bb91ceabca38e97819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::shader_lib::ShaderLibrary::get_compute -> ["shaderface::permutation::ShaderPermutationKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 34 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_taa @ 0x6f16c8 -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1d5a -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1efc -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f20fb -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2857 -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2937 -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2ba6 -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2d71 -> 0x10e1d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_shadow::setup_directional_light_cascades @ 0xca974d -> 0x1cb1540 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_post::blur_pyramid::render_blur_pyramid @ 0xcae96d -> 0x1cb98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_post::blur_pyramid::render_bidirectional_blur_pyramid @ 0xcaee87 -> 0x1cb98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource::Handle::concurrent_mut<rhapsody::raw::texture::Texture,2,tuple$<>,rhapsody_post::blur_pyramid::render_blur_pyramid::closure$0::closure_env$0> @ 0x1cbb6f5 -> 0x1cb98e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccbf90 -> 0x1cf7b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccc143 -> 0x1cf7b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccc3ce -> 0x1cf7b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xccc757 -> 0x1cf7b60 (direct_call); 同地址别名未裁定。
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
// 函数: rhapsody::shader_lib::ShaderLibrary::get_raster<clutter_icon_pipe::shader::clutter_icon_bake::vs_main<shabriri_rt::api::EmptyProtocol>,array$<tuple$<alloc::string::String,alloc::string::String>,0> >
// 信息 ID: 2cdd7f22cd4a7f5513714193
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] clutter_icon_pipe::shader::clutter_icon_bake::vs_main -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] array$ -> ["tuple$<alloc::string::String,alloc::string::String>", "0"]; 字段布局 = unknown。
// [类型关系] rhapsody::shader_lib::ShaderLibrary::get_raster -> ["clutter_icon_pipe::shader::clutter_icon_bake::vs_main<shabriri_rt::api::EmptyProtocol>", "array$<tuple$<alloc::string::String,alloc::string::String>,0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefe49 -> 0x2de24b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::get_raster_by_name<shaderface::permutation::ShaderPermutationKey>
// 信息 ID: fef288009e4a91032fae0674
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::shader_lib::ShaderLibrary::get_raster_by_name -> ["shaderface::permutation::ShaderPermutationKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::get_raster<clutter_icon_pipe::shader::clutter_icon_bake::vs_main<shabriri_rt::api::EmptyProtocol>,array$<tuple$<alloc::string::String,alloc::string::String>,0> > @ 0x2de24d4 -> 0x2de24e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::load_precompiled_shaders
// 信息 ID: c5d73fede1e71777d7b82bba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefb39 -> 0xe41550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::new
// 信息 ID: e69d4c72dc917759fd4afb70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::new @ 0xa50069 -> 0xe42400 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefa44 -> 0xe42400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::register_permutation
// 信息 ID: e2f7ed1b5cf55228bb279368
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::register_permutations @ 0xe4141f -> 0xe40780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::register_permutations
// 信息 ID: f3a14bc4943511d47893464c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::load_precompiled_shaders @ 0xe41829 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259f398 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259e2df -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259d2eb -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a68d2 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a5674 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a4674 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f63a6 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f51b0 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f416b -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc6a3 -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fb48d -> 0xe413a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fa38d -> 0xe413a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::ShaderLibrary::try_get_compute<shaderface::permutation::ShaderPermutationKey>
// 信息 ID: 704552b45c6209ee560adb17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::shader_lib::ShaderLibrary::try_get_compute -> ["shaderface::permutation::ShaderPermutationKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe254b1 -> 0xe3f270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::impl$24::fmt
// 信息 ID: 5995c22164e5a335fa3df522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::impl$26::fmt
// 信息 ID: 22039bf6fba2195614786b85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::impl$28::fmt
// 信息 ID: e7654a2994cba5aa1c214d9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::shader_lib::impl$2::get_raster_by_name::closure$0<shaderface::permutation::ShaderPermutationKey>
// 信息 ID: eb4bbf1a56b4996209f6da8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::shader_lib::impl$2::get_raster_by_name::closure$0 -> ["shaderface::permutation::ShaderPermutationKey"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::shader_lib::impl$2::get_raster_by_name", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::get_raster_by_name<shaderface::permutation::ShaderPermutationKey> @ 0x2de283d -> 0x2de29f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
