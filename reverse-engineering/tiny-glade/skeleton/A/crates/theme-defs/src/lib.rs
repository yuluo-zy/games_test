// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/theme-defs/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f792fea4a30df098d9e57558500523d4b44ef61474088eb18472da8260e481f3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 258
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: theme_defs::impl$0::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x362C
// 首条位置: {"module":258,"inlinee":13868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_defs::impl$2::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3645
// 首条位置: {"module":258,"inlinee":13893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":35436,"parent_record_offset":35404,"ancestor_record_offset":35404,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":258,"record_offset":35464,"parent_record_offset":35436,"ancestor_record_offset":35404,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":35492,"parent_record_offset":35464,"ancestor_record_offset":35404,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":35520,"parent_record_offset":35492,"ancestor_record_offset":35404,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_defs::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3647
// 首条位置: {"module":258,"inlinee":13895}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36692,"parent_record_offset":36652,"ancestor_record_offset":36652,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":37104,"parent_record_offset":37080,"ancestor_record_offset":36652,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":37128,"parent_record_offset":37104,"ancestor_record_offset":36652,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":37080,"parent_record_offset":37044,"ancestor_record_offset":36652,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":37044,"parent_record_offset":37016,"ancestor_record_offset":36652,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":37016,"parent_record_offset":36988,"ancestor_record_offset":36652,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":37200,"parent_record_offset":37016,"ancestor_record_offset":36652,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":36948,"parent_record_offset":36912,"ancestor_record_offset":36652,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":36912,"parent_record_offset":36876,"ancestor_record_offset":36652,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":36876,"parent_record_offset":36840,"ancestor_record_offset":36652,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":36988,"parent_record_offset":36948,"ancestor_record_offset":36652,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":36840,"parent_record_offset":36800,"ancestor_record_offset":36652,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":258,"record_offset":37312,"parent_record_offset":36800,"ancestor_record_offset":36652,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":258,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":36652,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":37240,"parent_record_offset":36948,"ancestor_record_offset":36652,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":37156,"parent_record_offset":37128,"ancestor_record_offset":36652,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":36728,"parent_record_offset":36692,"ancestor_record_offset":36652,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":258,"record_offset":36764,"parent_record_offset":36728,"ancestor_record_offset":36652,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":258,"record_offset":36800,"parent_record_offset":36764,"ancestor_record_offset":36652,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_defs::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3649
// 首条位置: {"module":258,"inlinee":13897}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":35724,"parent_record_offset":35572,"ancestor_record_offset":35572,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$29::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":258,"record_offset":35612,"parent_record_offset":35572,"ancestor_record_offset":35572,"inlinee":13899,"source_line":246,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":258,"record_offset":36160,"parent_record_offset":36060,"ancestor_record_offset":35572,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":36248,"parent_record_offset":36012,"ancestor_record_offset":35572,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":258,"record_offset":36364,"parent_record_offset":36332,"ancestor_record_offset":35572,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":36332,"parent_record_offset":36012,"ancestor_record_offset":35572,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":36096,"parent_record_offset":36060,"ancestor_record_offset":35572,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":35572,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35572,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":35572,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":35856,"parent_record_offset":35832,"ancestor_record_offset":35572,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":35832,"parent_record_offset":35808,"ancestor_record_offset":35572,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":35784,"parent_record_offset":35756,"ancestor_record_offset":35572,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36060,"parent_record_offset":36012,"ancestor_record_offset":35572,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36012,"parent_record_offset":35724,"ancestor_record_offset":35572,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":35756,"parent_record_offset":35724,"ancestor_record_offset":35572,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":35572,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36188,"parent_record_offset":36060,"ancestor_record_offset":35572,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36220,"parent_record_offset":36012,"ancestor_record_offset":35572,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$2::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":258,"record_offset":35636,"parent_record_offset":35612,"ancestor_record_offset":35572,"inlinee":13901,"source_line":111,"archive":"_pdb/A/modules/00258.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_defs::ActiveEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 37
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> >", "constructor": "core::result::Result", "arguments": ["theme_defs::ActiveEvent", "enum2$<theme_defs::EventDataError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_senders<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_senders", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_defs::EventDataError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 56
// [PDB 泛型关系，非字段关系] {"type": "enum2$<theme_defs::EventDataError>", "constructor": "enum2$", "arguments": ["theme_defs::EventDataError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> >", "constructor": "core::result::Result", "arguments": ["theme_defs::ActiveEvent", "enum2$<theme_defs::EventDataError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> >", "constructor": "core::result::Result", "arguments": ["std::path::PathBuf", "enum2$<theme_defs::EventDataError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > > >", "constructor": "alloc::sync::ArcInner", "arguments": ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure$0", "arguments": ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_senders<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_senders", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<core::result::Result<theme_defs::ActiveEvent,enum2$<theme_defs::EventDataError> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: theme_defs::impl$0::clone
// 信息 ID: 076d91ae183c50c6c54682c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=23592; 不是独立调用边。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=37360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_defs::impl$2::eq
// 信息 ID: 60ae763a1b36d380800d957f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=35404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_defs::impl$5::clone
// 信息 ID: ad29a7d8b466e804b1ae24be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=36652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_defs::impl$7::eq
// 信息 ID: 9e8961359e3fb7884bea4c6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=35572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
