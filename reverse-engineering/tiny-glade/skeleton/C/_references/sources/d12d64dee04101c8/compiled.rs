// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/d12d64dee04101c8/compiled.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/compiled.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: compiled.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compiled.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compiled.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::add_pre_node_barriers
// 信息 ID: 41624ef5f42d2abd4cfe7cc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tiny_glade::run_game::impl$0::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<tiny_glade::run_game::GraphExecutorBevy> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> > > > > @ 0x1920b2 -> 0xe81060 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7aa0 -> 0xe81060 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acf110 -> 0xe81060 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$6::fold<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,tuple$<>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,alloc::alloc::Global>,rhapsody::rg::executor::impl$1::run_in_parallel::closure_env$0<ref_mut$<typed_index_collections::slice::TiSlice<rhapsody::rg::resource_info::NodeIndex,rhapsody::rg::recorder::RecordedNode> >,tuple$<rhapsody::rg::compiled::impl$3::record_batch::RecordedNodeRange,alloc::vec::Vec<rhapsody::rg::executor::NodePgoDuration,alloc::alloc::Global> >,rhapsody::rg::compiled::impl$3::record_batch::closure_env$2<rhapsody::rg::executor::GraphExecutorSerial> > > > > > > @ 0x26e9294 -> 0xe81060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::compute_batches
// 信息 ID: dfba34c1a10556f1cea65ce4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute<tiny_glade::run_game::GraphExecutorBevy> @ 0x5883dc -> 0xe80830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::compile_and_execute<rhapsody::rg::executor::GraphExecutorSerial,tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x9cb856 -> 0xe80830 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::compile_and_execute<rhapsody::rg::executor::GraphExecutorSerial,tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x1d5ae0c -> 0xe80830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::gather_batch_preamble_barriers
// 信息 ID: f29cf1c60795f6248df44872
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a699e -> 0xe81740 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1acdff7 -> 0xe81740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::new
// 信息 ID: 372a9318bb9db0fc0c869051
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x39713b -> 0xe81b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39b374 -> 0xe81b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,rhapsody::rg::executor::impl$12::present::closure$0::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,rhapsody::rg::executor::impl$9::execute_and_present::closure_env$0<tiny_glade::run_game::GraphExecutorBevy,tiny_glade::render_loop::render::closure_env$3> > > @ 0x39daf7 -> 0xe81b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x177378f -> 0xe81b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x26e8180 -> 0xe81b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial>
// 信息 ID: f4a5138b4931e2cc12649e91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledGraph::record_batch -> ["rhapsody::rg::executor::GraphExecutorSerial"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::compile_and_execute<rhapsody::rg::executor::GraphExecutorSerial,tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1> @ 0x9cbaf5 -> 0x1acdfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::compile_and_execute<rhapsody::rg::executor::GraphExecutorSerial,tuple$<>,clutter_icon_pipe::render_icon::closure_env$0> @ 0x1d5b094 -> 0x1acdfb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy>
// 信息 ID: f5886bb050fb44beb0842295
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledGraph::record_batch -> ["tiny_glade::run_game::GraphExecutorBevy"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute<tiny_glade::run_game::GraphExecutorBevy> @ 0x588675 -> 0x1a6950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledGraph::record_postamble
// 信息 ID: 0d7583584b7c557f167172d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a72c5 -> 0xe80a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1ace935 -> 0xe80a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$3::record_batch::closure$1<rhapsody::rg::executor::GraphExecutorSerial>
// 信息 ID: 4a02b2d6238be0c1c33dfe5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::impl$3::record_batch::closure$1 -> ["rhapsody::rg::executor::GraphExecutorSerial"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::compiled::impl$3::record_batch", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1ace6a3 -> 0x1acee00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<rhapsody::rg::executor::GraphExecutorSerial> @ 0x1ace41b -> 0x1acee00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$3::record_batch::closure$1<tiny_glade::run_game::GraphExecutorBevy>
// 信息 ID: 280180e458ea9ac33fbeb281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::impl$3::record_batch::closure$1 -> ["tiny_glade::run_game::GraphExecutorBevy"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::compiled::impl$3::record_batch", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a7030 -> 0x1a7790 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledGraph::record_batch<tiny_glade::run_game::GraphExecutorBevy> @ 0x1a6dce -> 0x1a7790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===
