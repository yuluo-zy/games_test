// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/error.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ffda398e3c02fb497de198dceae33f877392d81208c788e9b3d3f25c3e49a76b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: error.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 39, 41, 52, 120, 129, 139, 178, 219, 228, 251, 390, 398, 399, 407, 436, 464, 514, 564, 565, 566, 567, 570, 572, 749, 1321, 1381, 1385, 1922, 2435, 2694, 2816, 2824, 3007, 3224, 4252, 4372, 4499
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::error::impl$1::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=44
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CB7
// 首条位置: {"module":39,"record_offset":5408,"rva":375808,"va":5369084928}
// 编译实例 RVAs: 0x5bc00, 0x65ac0, 0x743d0, 0x1cdf00, 0x257980, 0x269560, 0x2de800, 0x339a30, 0x3578e0, 0x3c0b50, 0x512c70, 0x53e800, 0x5451d0, 0x5613d0, 0x5db7c0, 0x642900, 0x6fbf80, 0x734560, 0x7375c0, 0x744f70, 0x746d50, 0xde8a00, 0xf7cb50, 0x1000890, 0x12c8b00, 0x1773610, 0x17f73b0, 0x18024e0, 0x1917e80, 0x1ebf8c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t8498676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369084928,"raw_hex":"e8f4ad8100","size":5,"soft_fail":false,"target_va":5377584016,"va":5369085335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t8499114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369084928,"raw_hex":"e8aaaf8100","size":5,"soft_fail":false,"target_va":5377584208,"va":5369085089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field3_finish
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t8499207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369084928,"raw_hex":"e807b08100","size":5,"soft_fail":false,"target_va":5377584464,"va":5369085252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::error::impl$3::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":564,"record_offset":360,"rva":7551312,"va":5376260432}
// 编译实例 RVAs: 0x733950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1319689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376260432,"raw_hex":"e809231400","size":5,"soft_fail":false,"target_va":5377580528,"va":5376260834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":564,"record_offset":480,"parent_record_offset":360,"ancestor_record_offset":360,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::error::impl$0::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x782B
// 首条位置: {"module":564,"record_offset":248,"rva":7551248,"va":5376260368}
// 编译实例 RVAs: 0x733910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t108091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376260368,"raw_hex":"e83ba60100","size":5,"soft_fail":false,"target_va":5376368480,"va":5376260384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: error.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::error::BackendError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 120
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::error::BackendError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::error::BackendError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::cause", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::type_id", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ContextError", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::source<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::source", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_ref", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_drop", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_boxed", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_downcast<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_downcast", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop_front<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_drop_front", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$3::fmt<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$3::fmt", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::texture::Texture", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::mem::manually_drop::ManuallyDrop", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<alloc::string::String,core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ContextError", "arguments": ["alloc::string::String", "core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<alloc::string::String,core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,core::mem::manually_drop::ManuallyDrop<enum2$<rhapsody::raw::error::BackendError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<core::mem::manually_drop::ManuallyDrop<alloc::string::String>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ContextError", "arguments": ["core::mem::manually_drop::ManuallyDrop<alloc::string::String>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<core::mem::manually_drop::ManuallyDrop<alloc::string::String>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<core::mem::manually_drop::ManuallyDrop<alloc::string::String>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<core::mem::manually_drop::ManuallyDrop<alloc::string::String>,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<core::mem::manually_drop::ManuallyDrop<alloc::string::String>,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "ref$", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::Error::construct", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::impl$1::from<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::impl$1::from", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ContextError", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::description<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::error::Error::description", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$3::fmt<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$3::fmt", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: error.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: error.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 13; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$0::from
// 信息 ID: 767146d40fd0c353b4231297
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x26a456 -> 0x733910 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=566, inline_record=10640; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_framebuffer; module=570, inline_record=63136; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=570, inline_record=71776; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$1::fmt
// 信息 ID: 5930839693f17f78944a5f29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=41, inline_record=8200; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=436, inline_record=404; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=749, inline_record=400; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=2435, inline_record=396; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=3224, inline_record=404; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=4252, inline_record=404; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=4372, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$3::fmt
// 信息 ID: 97cb04d88a264909ced0d072
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
