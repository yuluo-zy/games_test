// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/8de70a72a29706a9/ash.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/crates/gpu-profiler/src/backend/ash.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: ash.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ash.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ash.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::create_marker
// 信息 ID: fc87556d469e786b8ce55c32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a6afa -> 0xe8f4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x585880 -> 0xe8f4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8ce0 -> 0xe8f4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1ace15a -> 0xe8f4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d582a0 -> 0xe8f4b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::end_frame<rhapsody::raw::profiler::ProfilerBuffer>
// 信息 ID: 8058653fb9dc7bb4e0e775f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] gpu_profiler::backend::ash::ActiveProfiler::end_frame -> ["rhapsody::raw::profiler::ProfilerBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5980f -> 0x26e5a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::write_marker
// 信息 ID: 4e6a361a6275ad2af79e03d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x19246b -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7e49 -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x5858a0 -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8d00 -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf4b9 -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d582c0 -> 0xe8f450 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e964e -> 0xe8f450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::Profiler::begin_frame
// 信息 ID: a4d6c510737d17bd177b8f96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x585860 -> 0xe8f510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8cc0 -> 0xe8f510 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d58280 -> 0xe8f510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::Profiler::new
// 信息 ID: 774afef7092df01d1e5cda30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xe572f2 -> 0xe8f570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
