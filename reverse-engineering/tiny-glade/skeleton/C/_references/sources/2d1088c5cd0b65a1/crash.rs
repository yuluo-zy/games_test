// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/2d1088c5cd0b65a1/crash.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/crash.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: crash.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: crash.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: crash.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::crash::CrashMarkerNames::insert_name
// 信息 ID: 998db5c860821b6df01c5498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x192351 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7d2f -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<tiny_glade::run_game::GraphExecutorBevy> @ 0x5856d6 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x58745a -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x587772 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish<tiny_glade::run_game::GraphExecutorBevy> @ 0x5879d2 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9c8b36 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca07e -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca3a1 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x9ca628 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RecordedRasterStream::render @ 0xe1d314 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf39f -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d580f6 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d5963e -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59961 -> 0xe85950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::finish_and_wait<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1d59be8 -> 0xe85950 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::crash::log_gpu_debug_info
// 信息 ID: b03ddfadf6267dfd148c0ffe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::log_gpu_crash_info @ 0xe4e698 -> 0xe85b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
