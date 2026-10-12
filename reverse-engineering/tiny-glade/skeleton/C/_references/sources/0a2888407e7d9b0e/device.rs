// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/0a2888407e7d9b0e/device.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: device.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: device.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: device.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::allow_debug_markers
// 信息 ID: b12aaff9ba4a9bd6255b32b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::amd_buffer_marker
// 信息 ID: 129bccdad8e6580288a5fa4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x19235c -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7d3a -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x5856e5 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x587465 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x58777d -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x5879dd -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8b45 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca089 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca3ac -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca633 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf3aa -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d58105 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59649 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5996c -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59bf3 -> 0xe53110 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e953d -> 0xe53110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create
// 信息 ID: 633e2d0ffb0db6ec5af0a266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e89f -> 0xe537a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::setup::GraphicsInstanceInfo::create_first_available_device<rhapsody::setup::GraphicsDeviceDesc> @ 0x1d63489 -> 0xe537a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::debug_utils
// 信息 ID: c9434f7fe81235a195b0e9f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cea20 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cebe8 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x5cff72 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d430d -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5d4e0f -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x11a52fd -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147cf19 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d350 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f550 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f718 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed2db -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed49d -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed850 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14eda18 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3c79 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3e28 -> 0xe518d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
// 信息 ID: 47e624a01ea0a20c0eef2f6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d1314 -> 0xe53170 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1aca2e4 -> 0xe53170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::is_device_lost
// 信息 ID: f8001616a72e66e73648f1d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::mouse_capture::CustomCursorSsboGpuData::update_visibility @ 0x95da34 -> 0xe53100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::mouse_capture::CustomCursorSsboGpuData::write @ 0x95da90 -> 0xe53100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
// 信息 ID: 2a1c5f2ba9303eb0efe48557
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layout<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>,core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > > > @ 0x59e5d0 -> 0xe53620 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layout<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>,core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > > > @ 0x1a6b550 -> 0xe53620 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layout<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>,core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > > > @ 0x1d667e0 -> 0xe53620 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layout<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::Binding> >,alloc::alloc::Global>,core::iter::sources::empty::Empty<tuple$<shaderface::reflection::BindingIndex,ref$<shaderface::reflection::SamplerState> > > > @ 0x2a822cf -> 0xe53620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::recycle_gpu_epoch_resources
// 信息 ID: ec9073d3dd07c3bad9c1e8cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d28df -> 0xe535e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acb8af -> 0xe535e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
// 信息 ID: c3fa2d16213a3f2efff046b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56f8a0 -> 0xe53150 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x57e555 -> 0xe53150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle
// 信息 ID: b01c417969be493e7dcd627e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x4ba352 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<tiny_glade::run_game::GraphExecutorBevy> @ 0x587189 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<tiny_glade::run_game::GraphExecutorBevy> @ 0x587280 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9cb439 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9cb530 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::image_baker::finish_pending_image_bakes @ 0xafc9c2 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xe4b893 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xe5366c -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket @ 0xe60576 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5a9f9 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5aaf0 -> 0xe58f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
// 信息 ID: ac567d55a558dfb2364f61c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x56ee5c -> 0xe53650 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x57f666 -> 0xe53650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::swap_replacement_mods::ModSwapReloads::apply @ 0x90790b -> 0xe53650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2d155 -> 0xe53650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2e355 -> 0xe53650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::meshwatch::meshwatch @ 0xb32c92 -> 0xe53650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >
// 信息 ID: e9384eaec9049292d856b953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0 -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cece4 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xe4fd19 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f814 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14edb14 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x15e4494 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2831954 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2b1dd04 -> 0xe51d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<ref$<slice2$<u8> > > >
// 信息 ID: 79142f6ea1de5fb6d689c40d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0 -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$19::create_buffer::closure_env$0<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_index_buffer @ 0xe50986 -> 0xe51900 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d464 -> 0xe51900 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed582 -> 0xe51900 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3f07 -> 0xe51900 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x1ac7cb7 -> 0xe51900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0>
// 信息 ID: 5565f5a5f3770299faf0f07c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0xe50eb9 -> 0xe52cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1>
// 信息 ID: 91f53bf51dab68f4ca55c761
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::init_texture_data @ 0xe5e57e -> 0xe52200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::update_texture_impl::closure_env$2>
// 信息 ID: 91a5eea8a8fd46d61a19d6e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::texture::impl$15::update_texture_impl::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0xe5edd9 -> 0xe52510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::reclaim_command_buffer_resources
// 信息 ID: 25702dfdc92566ed4445dfe4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<tiny_glade::run_game::GraphExecutorBevy> > @ 0x5d28c9 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xe4b8c7 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0xe4b8f6 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xe536b7 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0xe536e6 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xe5c2c4 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0xe5c2f6 -> 0xe59990 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$12::submit_pending_cbs::closure_env$0<rhapsody::rg::executor::GraphExecutorSerial> > @ 0x1acb899 -> 0xe59990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::with_raw<rhapsody::raw::swapchain::impl$6::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >
// 信息 ID: 33e3b4ef4bce55a7c083ad38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::result::Result -> ["bool", "ash::vk::enums::Result"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<bool,ash::vk::enums::Result>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::Queue::with_raw -> ["rhapsody::raw::swapchain::impl$6::present_image::closure_env$0", "enum2$<core::result::Result<bool,ash::vk::enums::Result> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::FallibleSwapchain::present @ 0xe7aabb -> 0xe5b320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$5::drop
// 信息 ID: 647d31bde96a05548b9cc836
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xa731d -> 0xe4b860 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global> @ 0xe3caa8 -> 0xe4b860 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xe4972d -> 0xe4b860 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<alloc::sync::ArcInner<rhapsody::raw::device::GraphicsDevice> > @ 0x1d56728 -> 0xe4b860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$9::fmt
// 信息 ID: b993b4aabeeba40d88775b26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::query_device_extension_support::closure$0
// 信息 ID: 1b050567305fa2a91fb891b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::query_device_extension_support", "captures": "unknown"}
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe53962 -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe5397f -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe5399e -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe539bd -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe539dc -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe539fb -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe53a31 -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe53a5c -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe53a7e -> 0xe59670 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe53aba -> 0xe59670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
