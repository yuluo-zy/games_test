// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/sparkle.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 57de1eb46f1363d46584a07a9f90e0eef5532970acbe4c595922dcaa4b39730a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sparkle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 520
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// 当前没有可证实的 crate 引用信息。
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::sparkle::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":520,"record_offset":126256,"rva":10321168,"va":5379030288}
// 编译实例 RVAs: 0x9d7d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

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

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: sparkle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::sparkle::SparkleSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::option::Option", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo>", "constructor": "core::cell::UnsafeCell", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "slice2$", "arguments": ["core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >", "constructor": "std::sync::once_lock::OnceLock", "arguments": ["alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >", "constructor": "slice2$", "arguments": ["std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<slice2$<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["slice2$<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<slice2$<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<slice2$<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<usize>", "country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::iter::impl$13::from_iter<std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1<country_core::resources::sparkle::SparkleSsbo> > >", "constructor": "alloc::boxed::iter::impl$13::from_iter", "arguments": ["std::sync::once_lock::OnceLock<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::render::sparse_instance_buffer::impl$11::new::closure_env$1<country_core::resources::sparkle::SparkleSsbo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$2::grow_gpu_buffer_to_fit_shadow::closure_env$0<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$2::grow_gpu_buffer_to_fit_shadow::closure_env$0", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once::closure$0<country_core::render::sparse_instance_buffer::impl$2::grow_gpu_buffer_to_fit_shadow::closure_env$0<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "std::sync::once::impl$2::call_once::closure$0", "arguments": ["country_core::render::sparse_instance_buffer::impl$2::grow_gpu_buffer_to_fit_shadow::closure_env$0<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::try_insert::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >", "constructor": "std::sync::once_lock::impl$0::try_insert::closure_env$0", "arguments": ["alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>,std::sync::once_lock::impl$0::try_insert::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>", "std::sync::once_lock::impl$0::try_insert::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>,std::sync::once_lock::impl$0::try_insert::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global>,std::sync::once_lock::impl$0::try_insert::closure_env$0<alloc::boxed::Box<slice2$<core::cell::UnsafeCell<country_core::resources::sparkle::SparkleSsbo> >,alloc::alloc::Global> > >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$12::as_any_mut", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$12::as_any", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::impl$12::buffer<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::sparse_instance_buffer::impl$12::buffer", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::sparkle::SparkleSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::draw_passes::draw_lists::impl$0::multi_instanced_indexed::draw_fn<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::systems::render_loop::draw_passes::draw_lists::impl$0::multi_instanced_indexed::draw_fn", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::gpu_write_ranges_nonoverlapping<country_core::resources::sparkle::SparkleSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::gpu_write_ranges_nonoverlapping", "arguments": ["country_core::resources::sparkle::SparkleSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sparkle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sparkle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::sparkle::impl$0::default
// 信息 ID: 37b341dca687983bb83f501e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::sparkle::SparkleSsbo> @ 0x140459c -> 0x9d7d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
