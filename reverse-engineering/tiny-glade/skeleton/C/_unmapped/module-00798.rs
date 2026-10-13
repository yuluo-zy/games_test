// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00798.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00798.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00798.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00798.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::acquire_swapchain_image
// 信息 ID: 6dbfc3ee578fa2fcf54e2465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::with_next_image<rhapsody::rg::executor::impl$12::present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > > @ 0x492323 -> 0xe64760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::begin_label
// 信息 ID: a0834d74cb189201c5d4cacf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x1921ca -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x1922fd -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x19287a -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7baa -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7cdc -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a825a -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RecordedRasterStream::render @ 0xe1d275 -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<ref$<slice2$<u8> > > > @ 0xe51a1d -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > > @ 0xe51e8d -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1> @ 0xe52301 -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::update_texture_impl::closure_env$2> @ 0xe52639 -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0> @ 0xe52e13 -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_postamble @ 0xe80a89 -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf21a -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf34c -> 0xe4f830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf8ca -> 0xe4f830 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$1> >
// 信息 ID: 9c6742e27e683c33ab5487da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["shaderface::reflection::BindingIndex", "shaderface::reflection::BindingArrayElement", "enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$1"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::clone_binding_set
// 信息 ID: 3aa4bc59e7d25ef0bedc7923
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_transforms @ 0x8f5ec6 -> 0xe4ba90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,core::slice::iter::Iter<tuple$<bevy_ecs::entity::Entity,country_core::render::nani::gpu_world::GpuWorldSubsetKind,rhapsody::raw::binding_set::BindingSet> >,country_core::render::nani::maintenance::refresh_material_args::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<bevy_ecs::entity::Entity,rhapsody::raw::binding_set::BindingSet>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<bevy_ecs::entity::Entity,country_core::render::nani::gpu_world::GpuWorldSubsetKind,rhapsody::raw::binding_set::BindingSet> >,country_core::render::nani::maintenance::refresh_material_args::closure_env$0> > > > @ 0x1811984 -> 0xe4ba90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_binding_set
// 信息 ID: e2d2ecfafc2341c8b4eedfdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 51 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_tod_and_weather::update_snow @ 0x8f3b3a -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_glade_tod_and_weather::load_snow_particles @ 0x8f3e99 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_transforms @ 0x8f5ef6 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::loading_screen::update_loading_screen @ 0x8f74fe -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::loading_screen::startup_loading_screen @ 0x8f77b5 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::nani_on_scene_load @ 0x9efea7 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::animate_logo @ 0xb83aba -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::animate_logo @ 0xb83d15 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_animated_logo @ 0xb84630 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::logo_animation::startup_animated_logo @ 0xb847af -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba393b -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba3967 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba3eac -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefb6b -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1432bf8 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::timed_terrain_gizmo::MaterialData,country_core::shader::timed_terrain_gizmo::MaterialData> @ 0x16c9ca5 -> 0xe4c9d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_binding_set_impl
// 信息 ID: cec4b23102290778d8b8882a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::into_iter::impl$6::try_fold<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,core::iter::adapters::map::map_try_fold::closure_env$0<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> >,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> >,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> > > @ 0xe2e660 -> 0xe4c9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::clone_binding_set @ 0xe4bb5c -> 0xe4c9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set @ 0xe4c9d3 -> 0xe4c9e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_unique_binding_set @ 0xe4cef3 -> 0xe4c9e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 信息 ID: 7d215f4f047f611f4f35cf3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe15d9d -> 0xe4fa00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::FrameTimingReadback::take_buffer @ 0xe17d7d -> 0xe4fa00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer_impl
// 信息 ID: 80bee6258a0b47da2a7d3b64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cea76 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cec2d -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer @ 0xe212e5 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xe4fadf -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xe4fc8f -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_index_buffer @ 0xe5077e -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_index_buffer @ 0xe508ff -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0xe50dd3 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_mapped_buffer @ 0xe514db -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe56ef3 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0xe5ed07 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147cf7c -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d39f -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f5a6 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f75d -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed350 -> 0xe4ff50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_command_buffer
// 信息 ID: 778d9fbd8741d7e54d0d7973
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x191df1 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a77f9 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x5855dc -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x585a68 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::ensure_active_cb<tiny_glade::run_game::GraphExecutorBevy> @ 0x586dd4 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8a3c -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8ec8 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::ensure_active_cb<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9cb084 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<ref$<slice2$<u8> > > > @ 0xe51991 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > > @ 0xe51e01 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1> @ 0xe5228a -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::update_texture_impl::closure_env$2> @ 0xe525a9 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0> @ 0xe52d87 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acee69 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d57ffc -> 0xe4d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d58488 -> 0xe4d680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: 09e114848637e2a0d2961bc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["rhapsody::raw::shader::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline -> ["alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::shader_lib::ShaderLibrary::register_permutation @ 0xe40843 -> 0xe5b4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer
// 信息 ID: e684c2a571f27a56dd58fb2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x58550b -> 0xe643c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c896b -> 0xe643c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d57f2b -> 0xe643c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_gpu_scope
// 信息 ID: cc3a1d0c260d2cf9fa08d2a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56e0c7 -> 0xe644b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa934ee -> 0xe644b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::render_icon @ 0xceacba -> 0xe644b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_index_buffer
// 信息 ID: 31d6c78d9b5b0c032776a2d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_icon_pipe::render_icon @ 0xcea993 -> 0xe506b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_mapped_buffer
// 信息 ID: 555f59ff65204d1c10a2fbdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x3f829d -> 0xe51440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bcce2 -> 0xe51440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::start_recording @ 0x5af9ef -> 0xe51440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93479 -> 0xe51440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env @ 0xafd217 -> 0xe51440 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::render_icon @ 0xceac70 -> 0xe51440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_render_pass
// 信息 ID: c6317bb00487a081111cb8ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<0,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,0>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> > > @ 0x451cb2 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::create_rg_raster_node<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1> > @ 0x452362 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2a17 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2a60 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2aa0 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2af9 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2b4c -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2b9f -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2be8 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2c2d -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2c79 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2cbd -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2d25 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2d62 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_shaders::RenderPasses::new @ 0x8f2da9 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_gpu_bake_resources_for_device @ 0xcefe24 -> 0xe4cf00 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_swapchain
// 信息 ID: 5e30ce55036c9f2bbee44d6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x412f8c -> 0xe644f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::setup::create_swapchain_with_fallback<ref_mut$<enum2$<tiny_glade::run_game::WindowBackendWindow> > > @ 0x41336e -> 0xe644f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> >
// 信息 ID: 0d947d374ccea5a957f00318
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::texture::TextureInitialData"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location -> ["enum2$<rhapsody::raw::texture::TextureInitialData>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe162c1 -> 0xe5f290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_unique_binding_set
// 信息 ID: ad7b6715ab9f9790d7255ea5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::RenderParams::new @ 0xa7d1aa -> 0xe4cef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::end_label
// 信息 ID: 1cafb93b90e477fdb4436ecc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x1921a6 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x192681 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x192856 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7b86 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a8060 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a8236 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf1f6 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf6d0 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf8a6 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e9386 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e9845 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e9a16 -> 0xe4f9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_built_in_sampler
// 信息 ID: 665fb2a8b63cd9b56ca1131d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_or_create_sampler
// 信息 ID: f4dc002429f9e64786e526bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layouts @ 0xe38359 -> 0xe5dd90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::has_any_gpu_assertion_failed
// 信息 ID: aa9385d4a216d0f2df6d54e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d128f -> 0xe4f6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1aca25f -> 0xe4f6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket
// 信息 ID: c268437d4815bbe8bf94f365
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::resources::DeviceResources::take_out_waste @ 0xe3591f -> 0xe5fd60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::init_texture_data
// 信息 ID: 2d6618601bd7acf9964c871e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d483e -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5d5367 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe5f9a5 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x11a582e -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x15e7037 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<ref$<slice2$<u8> > > @ 0x15e7b67 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1acd807 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cc5097 -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cfcfcf -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1e4fe6f -> 0xe5e500 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x21d964f -> 0xe5e500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::issue_full_pipeline_barrier
// 信息 ID: 73a0872eaade718bc4d5cf1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x192122 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7b10 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$2,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x177500a -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$2,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x17750ca -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$2,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x177518a -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$2,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x1775240 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x17753b1 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x1775452 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x17754f3 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists::closure$2::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x1775590 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prepare_empty_unsorted_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x1775c64 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi> > @ 0x1776a04 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchApi::dispatch_threads<u32> @ 0x179aff5 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf180 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e9304 -> 0xe5d040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::issue_pipeline_barrier
// 信息 ID: 153696e76d007171f3217cc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x1920dd -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a69f7 -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7acb -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1ace05b -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf13b -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e92bf -> 0xe5cfc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::log_error_info
// 信息 ID: ad19f0b8d2cd829b3ac28ba3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d255d -> 0xe4dba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xe5c5ba -> 0xe4dba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xe5c558 -> 0xe4dba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acb52d -> 0xe4dba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::log_gpu_crash_info
// 信息 ID: 154e9ad1b22d4099f92eb0fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d2e13 -> 0xe4e430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::log_error_info @ 0xe4dc99 -> 0xe4e430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::log_vulkan_error_info @ 0xe4f40d -> 0xe4e430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::present @ 0xe7aed8 -> 0xe4e430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acbde3 -> 0xe4e430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::log_gpu_markers
// 信息 ID: 6049bd802330413184f72d71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::log_gpu_crash_info @ 0xe4e460 -> 0xe4dd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xe5c258 -> 0xe4dd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::log_vulkan_error_info
// 信息 ID: 87bd79dbb50772eb7a3c4785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::acquire_next_image @ 0xe79df8 -> 0xe4f320 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::present @ 0xe7aecb -> 0xe4f320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::queue_family_index
// 信息 ID: 9a9eaadd9dd84a9af6ed3ea2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x191dd3 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a77dc -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x5855a6 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x585a39 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::ensure_active_cb<tiny_glade::run_game::GraphExecutorBevy> @ 0x586db7 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8a06 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8e99 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::ensure_active_cb<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9cb067 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acee4c -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d57fc6 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d58459 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::ensure_active_cb<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5a627 -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e8fde -> 0xe5c8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::record_gpu_diagnostic
// 信息 ID: ba31b925e2caeb4f272db4c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x587bc3 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d27a9 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d28a1 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca82f -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::acquire_next_image @ 0xe796ea -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::present @ 0xe7ab80 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acb779 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acb871 -> 0xe4f430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59def -> 0xe4f430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name
// 信息 ID: 45945bd09687ac6635df1edf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe15dd6 -> 0xe515b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe1537b -> 0xe515b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::set_texture_debug_name
// 信息 ID: 4c1a29b32c2e39ab6a0bc122
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe16304 -> 0xe5ef60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::new @ 0xe159a9 -> 0xe5ef60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 信息 ID: 245edc5798a9c7fce29919bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x847386 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x86ef89 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x86f0e0 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments @ 0x94e13b -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x9536c2 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x95378b -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x953856 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::set_draw_list_params @ 0x9e0a56 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices @ 0x9e0fe6 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings @ 0xb37313 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0xb952d2 -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0xb954ed -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba90bb -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::StoneFoundationColorIdSsbo> @ 0x10c1aba -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::HalftimberColorIdSsbo> @ 0x10c1c6a -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::wall_color_slots::write_color_slot<country_core::resources::color_id::WallColorIdSsbo> @ 0x10c1e1a -> 0xe50bd0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_descriptor_set<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure$0::closure$0::closure_env$0>,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure$0::closure$0::closure_env$1>
// 信息 ID: 5df4105ad0a64e9a466848ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["shaderface::reflection::BindingIndex", "shaderface::reflection::BindingArrayElement", "enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure$0::closure$0::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::update_descriptor_set -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure$0::closure$0::closure_env$0>", "rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure$0::closure$0::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::into_iter::impl$6::try_fold<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,core::iter::adapters::map::map_try_fold::closure_env$0<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> >,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> >,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> > > @ 0xe2ebb1 -> 0xe5d680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_texture
// 信息 ID: a71cef4c9242e3ac2c2347e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3722 -> 0xe5e4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0xa0bee8 -> 0xe5e4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::upload_pending_glyphs @ 0xcb0fc5 -> 0xe5e4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_duration_or_until_submit_done
// 信息 ID: 762927b174cd88d3608d58bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim<tiny_glade::run_game::GraphExecutorBevy> @ 0x584d3a -> 0xe5c8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_submit_done
// 信息 ID: d3bcb6c442481c583daf0acc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x58531c -> 0xe5bc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c877c -> 0xe5bc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca9bd -> 0xe5bc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d57d3c -> 0xe5bc90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59f7d -> 0xe5bc90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
