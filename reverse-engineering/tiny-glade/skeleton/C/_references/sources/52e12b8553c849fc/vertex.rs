// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/52e12b8553c849fc/vertex.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/vertex.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: vertex.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vertex.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vertex.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(array$<f64,4>,ref_mut$<slice2$<u8> >)
// 候选参数类型: ["array$<f64,4>", "ref_mut$<slice2$<u8> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<shaderface::binding_set::BindingSetLayoutItemIdent> (*)(enum2$<shaderface::binding_set::BindingSetLayoutItemIdent>)
// 候选参数类型: ["enum2$<shaderface::binding_set::BindingSetLayoutItemIdent>"]
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<rhapsody::vertex::SourceVertexDataArray>
// 信息 ID: a77315f2678b48568bc82eb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cf604e93a68ccde53a657c75: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// [类型关系] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer -> ["rhapsody::vertex::SourceVertexDataArray"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<rhapsody::vertex::SourceVertexDataArray>,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<rhapsody::vertex::SourceVertexDataArray> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,tuple$<> > > > @ 0x1d5cd34 -> 0x26e2870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::SourceVertexDataArray::new
// 信息 ID: 1628f220ff5961a07d034299
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_icon_pipe::render_icon @ 0xcea615 -> 0xe481d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::VertexAttributeValues::append
// 信息 ID: a91a9685b8f5b7e319c54a7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::mesh_atlas::MeshAtlas::add @ 0xa6570a -> 0xe48690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::VertexAttributeValues::as_f32x2
// 信息 ID: 388d8228adf5c98745f16e44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa88e31 -> 0xe48dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::uvs_without_indices @ 0xc2a40d -> 0xe48dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::VertexAttributeValues::as_f32x3
// 信息 ID: 2dc7a9df8b9ce701a3964f24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2> @ 0xa295fd -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1> @ 0xa29c2d -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::mesh_atlas::MeshAtlas::add @ 0xa64f3c -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa67185 -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa673e6 -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa67531 -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa679cc -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa88c4f -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa88d3f -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::aabb_bounds @ 0xc29bd1 -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::as_triangles @ 0xc29d91 -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::normals_without_indices @ 0xc2a51d -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::positions_without_indices @ 0xc2a61d -> 0xe48e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::positions @ 0xc2aacd -> 0xe48e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::VertexFormat::encode
// 信息 ID: 3fefc7285f3b61b3f5d9931b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<country_core::render::mesh::Mesh> @ 0x1a34c9c -> 0xe44830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<rhapsody::vertex::SourceVertexDataArray> @ 0x26e2970 -> 0xe44830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::VertexLayout::to_static
// 信息 ID: 29e915a155ada04a49593371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x259f17e -> 0xe481b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)> @ 0x25a66ad -> 0xe481b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadOnlyBootupShaders>,tuple$<> > > > @ 0x25f618c -> 0xe481b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<shaderface::shader::PrecompiledShaderModule,alloc::alloc::Global>,shaderface::shader::PrecompiledShaderModule,country_core::startup::load_shaders::preload_shaders::closure_env$2<country_core::startup::load_shaders::LoadAllGameShaders>,tuple$<> > > > @ 0x25fc483 -> 0xe481b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$12::interleave_vertices
// 信息 ID: e031d4eacea1a69a726be006
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_shaders::create_vertex_layout_via_pipeline_reflection @ 0x8f3475 -> 0xe42650 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefce6 -> 0xe42650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$15::from
// 信息 ID: 275966a5251d97ae9ad6e8be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da873 -> 0xe42700 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> > @ 0xac76f4 -> 0xe42700 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> > @ 0xac7642 -> 0xe42700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$17::from
// 信息 ID: 5953b88f1d64bd6f9a0d8830
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da683 -> 0xe42690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> > @ 0xac7ff4 -> 0xe42690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> > @ 0xac7f42 -> 0xe42690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$18::from
// 信息 ID: c1fc58ccf1b2064050bfcc6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da93c -> 0xe42980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > @ 0xac7b74 -> 0xe42980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > @ 0xac7ac2 -> 0xe42980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > @ 0x24e1ca4 -> 0xe42980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> > @ 0x24e1bf2 -> 0xe42980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$19::from
// 信息 ID: e728c058c8d2becbc117dd1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da8d9 -> 0xe429f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > @ 0xac8474 -> 0xe429f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > @ 0xac83c2 -> 0xe429f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > @ 0x24e2124 -> 0xe429f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> > @ 0x24e2072 -> 0xe429f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$20::from
// 信息 ID: 817c6601874ca92f9d74319b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9da7cf -> 0xe42a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> > @ 0xac6ef4 -> 0xe42a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> > @ 0xac6e42 -> 0xe42a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$3::find_attrib
// 信息 ID: 56546e6e2f2a12f686986900
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<rhapsody::vertex::SourceVertexDataArray> @ 0x26e2940 -> 0xe48e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$3::vertex_count
// 信息 ID: c38427c16600a7c2f42c7e3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<rhapsody::vertex::SourceVertexDataArray> @ 0x26e28a9 -> 0xe48ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$42::fmt
// 信息 ID: 344ba4ce9c6e49d2915233d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$44::fmt
// 信息 ID: f465ce6c5b13dc76d0173d61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::vertex::impl$9::interleave_vertices
// 信息 ID: 63a37c5beec7ced3b1e38179
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_shaders::create_vertex_layout_via_pipeline_reflection @ 0x8f34ab -> 0xe42770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::vertex::impl$12::interleave_vertices @ 0xe42674 -> 0xe42770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
