// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/error.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ffda398e3c02fb497de198dceae33f877392d81208c788e9b3d3f25c3e49a76b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: error.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 33, 43, 48, 61, 102, 105, 145, 149, 238, 245, 437, 500, 502, 503, 505, 510, 543, 548, 559, 575, 624, 684, 689, 696, 699, 703, 705, 707, 1190, 1198, 1533, 1610, 1633, 1655, 1935, 2787, 3014, 3095, 3273, 3477, 3490, 4323, 4466, 4698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::error::impl$1::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=51
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2066
// 首条位置: {"module":33,"record_offset":4444,"rva":906256,"va":5369615376}
// 编译实例 RVAs: 0xdd410, 0xf9eb0, 0x109f50, 0x1570b0, 0x2b12e0, 0x2c1a60, 0x364a20, 0x371830, 0x589170, 0x5b8fc0, 0x7cef30, 0x8ed880, 0x8fca00, 0x9046b0, 0x916140, 0x939790, 0xa32480, 0xa532d0, 0xa96e10, 0xb00560, 0xb8e1c0, 0xbe9270, 0xbecc90, 0xbff930, 0xc02bf0, 0xc08c30, 0xfafb80, 0x147a500, 0x160f190, 0x1697480, 0x18b4da0, 0x1e530e0, 0x1fd3bc0, 0x205f650, 0x2138520, 0x21b5040, 0x285a470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=37
//     evidence={"assembly":"\tcallq\t13276740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369615376,"raw_hex":"e84496ca00","size":5,"soft_fail":false,"target_va":5382892528,"va":5369615783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=37
//     evidence={"assembly":"\tcallq\t13277162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369615376,"raw_hex":"e8ea97ca00","size":5,"soft_fail":false,"target_va":5382892704,"va":5369615537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field3_finish
//     owner=rust_standard_library; count=37
//     evidence={"assembly":"\tcallq\t13277239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369615376,"raw_hex":"e83798ca00","size":5,"soft_fail":false,"target_va":5382892944,"va":5369615700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":689,"record_offset":256,"rva":12440560,"va":5381149680}
// 编译实例 RVAs: 0xbdd3f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1738057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381149680,"raw_hex":"e849851a00","size":5,"soft_fail":false,"target_va":5382888144,"va":5381150082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":689,"record_offset":376,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/00689.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::error::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9968
// 首条位置: {"module":684,"inlinee":39272}
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
// 完整结构及原始类型记录: error.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::error::BackendError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 134
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::error::BackendError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::error::BackendError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ContextError", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_drop", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_downcast<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_downcast", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop_front<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_drop_front", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$3::fmt<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$3::fmt", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::texture::Texture", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::cause", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::type_id", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_ref", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::object_boxed", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::Error::construct", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::impl$1::from<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::impl$1::from", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$4::source<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$4::source", "arguments": ["alloc::string::String", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::ContextError", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::ErrorImpl", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::description<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::description", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::cause<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::cause", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::type_id<anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::error::Error::type_id", "arguments": ["anyhow::error::ErrorImpl<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_drop<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_drop", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_downcast<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_downcast", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::context_drop_rest<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::error::context_drop_rest", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$2::fmt<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$2::fmt", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$3::fmt<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$3::fmt", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::provide<anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "core::error::Error::provide", "arguments": ["anyhow::error::ContextError<alloc::string::String,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_ref<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_ref", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::object_boxed<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::object_boxed", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::Error::construct", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$4::source<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "anyhow::context::impl$4::source", "arguments": ["ref$<str$>", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::source<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::source", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::error::Error::provide<enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::error::Error::provide", "arguments": ["enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::error::impl$10::fmt<anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "anyhow::error::impl$10::fmt", "arguments": ["anyhow::error::ContextError<ref$<str$>,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 16; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$0::from
// 信息 ID: f49f0dee2d69c8bf7270535d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=684, inline_record=12016; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_framebuffer; module=703, inline_record=115844; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::wait_submit_done; module=703, inline_record=124584; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure_env$0>; module=707, inline_record=31028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$1::fmt
// 信息 ID: 50553fcb18880507c1fe27c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=145, inline_record=5904; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=500, inline_record=1040; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=1198, inline_record=408; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=1533, inline_record=416; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=3490, inline_record=416; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=4323, inline_record=412; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::error::BackendError> >; module=4466, inline_record=412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::error::impl$3::fmt
// 信息 ID: 09505835bba13174778665c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
