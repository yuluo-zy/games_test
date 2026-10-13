// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: rhapsody2/crates/gpu-profiler/src/shared.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: efb283114c990e0fd7c290a163aac56b0c657ede6f45697dde087df177b36c90
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shared.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 790, 811

// 归属: dependency_reuse_metadata_only
// Repository: PounceLight/rhapsody2
// Revision: 48efa87114d22e484592a0e9efe866faafa6a876
// Source kind: repository_source

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::MarkerId::as_u64
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xCD63
// 首条位置: {"module":811,"inlinee":52579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::MarkerId::from_u64
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":811,"record_offset":268,"rva":15266848,"va":5383975968}
// 编译实例 RVAs: 0xe8f420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::GpuTick::duration_since
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC4FD
// 首条位置: {"module":790,"inlinee":50429}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::GpuTickDuration::nanoseconds
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC4FF
// 首条位置: {"module":790,"inlinee":50431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::FrameMarkers::create_marker
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xCD64
// 首条位置: {"module":811,"inlinee":52580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::shared::FrameMarkers::marker_name
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC4FB
// 首条位置: {"module":790,"inlinee":50427}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...
// === 可用 IPI 函数身份、作用域字符串与构建信息引用图 ===
// IPI 0x10AC:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          ''
// ...
// IPI 0xC4FA:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'gpu_profiler::shared::FrameMarkers'
// ...
// IPI 0xC4FB:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     50426
//   FunctionType:    4097
//   Name:            marker_name
// ...
// IPI 0xC4FC:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'gpu_profiler::shared::GpuTick'
// ...
// IPI 0xC4FD:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     50428
//   FunctionType:    4097
//   Name:            duration_since
// ...
// IPI 0xC4FE:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'gpu_profiler::shared::GpuTickDuration'
// ...
// IPI 0xC4FF:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     50430
//   FunctionType:    4097
//   Name:            nanoseconds
// ...
// IPI 0xC543:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87'
// ...
// IPI 0xC544:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/lib.rs/@/rhapsody.73b8a89e5e911c82-cgu.14'
// ...
// IPI 0xC545:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 50499, 4268, 50500, 4268, 4268 ]
// ...
// IPI 0xCD62:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          'gpu_profiler::shared::MarkerId'
// ...
// IPI 0xCD63:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     52578
//   FunctionType:    4097
//   Name:            as_u64
// ...
// IPI 0xCD64:
// ---
// Kind:            LF_FUNC_ID
// FuncId:
//   ParentScope:     50426
//   FunctionType:    4097
//   Name:            create_marker
// ...
// IPI 0xCD6A:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/crates/gpu-profiler'
// ...
// IPI 0xCD6B:
// ---
// Kind:            LF_STRING_ID
// StringId:
//   Id:              0
//   String:          '/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/crates/gpu-profiler/src/lib.rs/@/gpu_profiler.936fbcfb89d39538-cgu.0'
// ...
// IPI 0xCD6C:
// ---
// Kind:            LF_BUILDINFO
// BuildInfo:
//   ArgIndices:      [ 52586, 4268, 52587, 4268, 4268 ]
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: shared.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::FrameMarkers
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::GpuTick
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "constructor": "tuple$", "arguments": ["gpu_profiler::shared::MarkerId", "gpu_profiler::shared::GpuTick"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "u64", "gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::GpuTickDuration
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::MarkerId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "constructor": "tuple$", "arguments": ["gpu_profiler::shared::MarkerId", "gpu_profiler::shared::GpuTick"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "u64", "gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<gpu_profiler::shared::MarkerId,core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["gpu_profiler::shared::MarkerId", "core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::NamedMarker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$3::drop<gpu_profiler::shared::NamedMarker,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$3::drop", "arguments": ["gpu_profiler::shared::NamedMarker", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<gpu_profiler::shared::NamedMarker,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["gpu_profiler::shared::NamedMarker", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] gpu_profiler::shared::TimedFrameMarkers
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<gpu_profiler::shared::TimedFrameMarkers>", "constructor": "core::ptr::drop_in_place", "arguments": ["gpu_profiler::shared::TimedFrameMarkers"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shared.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===
