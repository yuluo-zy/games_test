// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/mei/widgets/tweaker.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8dd18efee96951639920d7aa83029dacb8eed173244551f306833a715797ed38
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: tweaker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 15, 58, 170, 321, 1018, 2468, 3685
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"PDB_registry_path_only","version":"1.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$0::format
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":321,"record_offset":244,"rva":4472656,"va":5373181776}
// 编译实例 RVAs: 0x443f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4347941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373181776,"raw_hex":"e825584200","size":5,"soft_fail":false,"target_va":5377529888,"va":5373181942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":321,"record_offset":376,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":321,"record_offset":424,"parent_record_offset":400,"ancestor_record_offset":244,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":321,"record_offset":400,"parent_record_offset":376,"ancestor_record_offset":244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$1::decrease
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":321,"record_offset":460,"rva":4472848,"va":5373181968}
// 编译实例 RVAs: 0x444010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30412249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373181968,"raw_hex":"e8d90dd001","size":5,"soft_fail":false,"target_va":5403594288,"va":5373182034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29796730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373181968,"raw_hex":"e87aa9c601","size":5,"soft_fail":false,"target_va":5402978944,"va":5373182209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":321,"record_offset":712,"parent_record_offset":460,"ancestor_record_offset":460,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$55::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":321,"record_offset":600,"parent_record_offset":460,"ancestor_record_offset":460,"inlinee":17482,"source_line":343,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":321,"record_offset":684,"parent_record_offset":460,"ancestor_record_offset":460,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":321,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":460,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":321,"record_offset":760,"parent_record_offset":712,"ancestor_record_offset":460,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$1::increase
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":321,"record_offset":796,"rva":4473104,"va":5373182224}
// 编译实例 RVAs: 0x444110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30411993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182224,"raw_hex":"e8d90cd001","size":5,"soft_fail":false,"target_va":5403594288,"va":5373182290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29796474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182224,"raw_hex":"e87aa8c601","size":5,"soft_fail":false,"target_va":5402978944,"va":5373182465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":321,"record_offset":1048,"parent_record_offset":796,"ancestor_record_offset":796,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$55::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":321,"record_offset":936,"parent_record_offset":796,"ancestor_record_offset":796,"inlinee":17482,"source_line":343,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":321,"record_offset":1020,"parent_record_offset":796,"ancestor_record_offset":796,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":321,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":796,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":321,"record_offset":1096,"parent_record_offset":1048,"ancestor_record_offset":796,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$3::decrease
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 94, 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x445D
// 首条位置: {"module":321,"record_offset":1132,"rva":4473360,"va":5373182480}
// 编译实例 RVAs: 0x444210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$3::increase
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 98, 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x445E
// 首条位置: {"module":321,"record_offset":1256,"rva":4473376,"va":5373182496}
// 编译实例 RVAs: 0x444220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$3::format
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x445F
// 首条位置: {"module":321,"record_offset":1380,"rva":4473392,"va":5373182512}
// 编译实例 RVAs: 0x444230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-432900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182512,"raw_hex":"e8fc64f9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373182591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30410734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182512,"raw_hex":"e8ee07d001","size":5,"soft_fail":false,"target_va":5403593360,"va":5373182621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29795476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182512,"raw_hex":"e894a4c601","size":5,"soft_fail":false,"target_va":5402978144,"va":5373182663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4344465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182512,"raw_hex":"e8914a4200","size":5,"soft_fail":false,"target_va":5377527120,"va":5373182650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2970317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373182512,"raw_hex":"e8cd522d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5373182558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":321,"record_offset":1512,"parent_record_offset":1380,"ancestor_record_offset":1380,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":321,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1380,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":321,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1380,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":321,"record_offset":1788,"parent_record_offset":1752,"ancestor_record_offset":1380,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":321,"record_offset":1752,"parent_record_offset":1724,"ancestor_record_offset":1380,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":321,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":1380,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":321,"record_offset":1904,"parent_record_offset":1724,"ancestor_record_offset":1380,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":321,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":1380,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":321,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":1380,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":321,"record_offset":1580,"parent_record_offset":1540,"ancestor_record_offset":1380,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":321,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":1380,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":321,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":1380,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":321,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":1380,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":321,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1380,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":321,"record_offset":1944,"parent_record_offset":1664,"ancestor_record_offset":1380,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":321,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1380,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$4::decrease
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E4
// 首条位置: {"module":15,"inlinee":5604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] enum_iterator::previous
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":2232,"parent_record_offset":2204,"ancestor_record_offset":2204,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_controls::_::impl$0::previous
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_controls.rs
//     evidence={"module":15,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":2204,"inlinee":5608,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::previous
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":8200,"inlinee":5663,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$4::increase
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1610
// 首条位置: {"module":15,"inlinee":5648}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] enum_iterator::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":3872,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_controls::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_controls.rs
//     evidence={"module":15,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3872,"inlinee":5650,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":9452,"inlinee":5664,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$4::format
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E9
// 首条位置: {"module":15,"inlinee":5609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":2324,"parent_record_offset":2292,"ancestor_record_offset":2292,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2292,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":2628,"parent_record_offset":2572,"ancestor_record_offset":2292,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":2292,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":2292,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":2376,"parent_record_offset":2324,"ancestor_record_offset":2292,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":2404,"parent_record_offset":2324,"ancestor_record_offset":2292,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":2432,"parent_record_offset":2324,"ancestor_record_offset":2292,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":2496,"parent_record_offset":2324,"ancestor_record_offset":2292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":2460,"parent_record_offset":2324,"ancestor_record_offset":2292,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$6::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":7840,"parent_record_offset":7808,"ancestor_record_offset":7704,"inlinee":5661,"source_line":1036,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$5::decrease
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1616
// 首条位置: {"module":15,"inlinee":5654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":5016,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$5::increase
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x161B
// 首条位置: {"module":15,"inlinee":5659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$5::format
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1618
// 首条位置: {"module":15,"inlinee":5656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5108,"parent_record_offset":5072,"ancestor_record_offset":5072,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":20784,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":5072,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5424,"parent_record_offset":5368,"ancestor_record_offset":5072,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":15,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20784,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":5072,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":5072,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5164,"parent_record_offset":5108,"ancestor_record_offset":5072,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":23120,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":5192,"parent_record_offset":5108,"ancestor_record_offset":5072,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":18140,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":23264,"parent_record_offset":23156,"ancestor_record_offset":23120,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":5292,"parent_record_offset":5108,"ancestor_record_offset":5072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":5256,"parent_record_offset":5108,"ancestor_record_offset":5072,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$2::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":18252,"parent_record_offset":18176,"ancestor_record_offset":18140,"inlinee":5666,"source_line":996,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$3::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":28064,"parent_record_offset":27988,"ancestor_record_offset":27952,"inlinee":5669,"source_line":1005,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$4::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":5220,"parent_record_offset":5108,"ancestor_record_offset":5072,"inlinee":5658,"source_line":1017,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$10::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F21
// 首条位置: {"module":170,"inlinee":12065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::HintTracker::text
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4460
// 首条位置: {"module":321,"record_offset":2028,"rva":4473552,"va":5373182672}
// 编译实例 RVAs: 0x4442d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::HintTracker::confirm
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4450
// 首条位置: {"module":321,"inlinee":17488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::MaybeHint::confirm
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 176, 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4462
// 首条位置: {"module":321,"record_offset":2156,"rva":4473568,"va":5373182688}
// 编译实例 RVAs: 0x4442e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alloc::str::_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$::clone_into
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t4347613","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373182688,"raw_hex":"e9dd564200","size":5,"soft_fail":false,"target_va":5377530320,"va":5373182702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::HintTracker::confirm
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":321,"record_offset":2292,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":17488,"source_line":165,"archive":"_pdb/A/modules/00321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$8::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":321,"record_offset":2320,"rva":4473600,"va":5373182720}
// 编译实例 RVAs: 0x444300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::impl$9::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":321,"record_offset":2444,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker_w_font_size
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1614
// 首条位置: {"module":15,"inlinee":5652}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":6696,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":2752,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17189,"source_line":252,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":6844,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=22
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4724,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":672,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":1700,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":5540,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":7948,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17223,"source_line":208,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3316,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3136,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17224,"source_line":214,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3196,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17222,"source_line":202,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=22
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":4836,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3256,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=22
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":4984,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::on_click
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":4576,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17218,"source_line":170,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":2864,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":5740,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":2808,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17506,"source_line":176,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$3::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":7532,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17501,"source_line":94,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$3::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":6904,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17503,"source_line":102,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$3::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":11520,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":17502,"source_line":98,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::decrease
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":8200,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":5604,"source_line":117,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::format
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":7704,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":5609,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::increase
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":9452,"parent_record_offset":7316,"ancestor_record_offset":7316,"inlinee":5648,"source_line":123,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":5016,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":5072,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":6668,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":416,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":416,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":4636,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7288,"parent_record_offset":7240,"ancestor_record_offset":416,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5424,"parent_record_offset":5368,"ancestor_record_offset":4636,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":15,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20352,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":4636,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7240,"parent_record_offset":7204,"ancestor_record_offset":416,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7204,"parent_record_offset":7168,"ancestor_record_offset":416,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7408,"parent_record_offset":7204,"ancestor_record_offset":416,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4636,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7128,"parent_record_offset":7092,"ancestor_record_offset":416,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7092,"parent_record_offset":7056,"ancestor_record_offset":416,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7020,"parent_record_offset":6984,"ancestor_record_offset":416,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7056,"parent_record_offset":7020,"ancestor_record_offset":416,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":6984,"parent_record_offset":6948,"ancestor_record_offset":416,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3685,"record_offset":6948,"parent_record_offset":6904,"ancestor_record_offset":416,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5164,"parent_record_offset":5108,"ancestor_record_offset":4636,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":22684,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5108,"parent_record_offset":5072,"ancestor_record_offset":4636,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":20352,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3685,"record_offset":7168,"parent_record_offset":7128,"ancestor_record_offset":416,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4636,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4636,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":4636,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":5192,"parent_record_offset":5108,"ancestor_record_offset":4636,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":17704,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":23264,"parent_record_offset":23156,"ancestor_record_offset":22684,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":7808,"parent_record_offset":7740,"ancestor_record_offset":7316,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4636,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4636,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4636,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":6600,"parent_record_offset":5540,"ancestor_record_offset":4636,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3685,"record_offset":7448,"parent_record_offset":7128,"ancestor_record_offset":416,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":5292,"parent_record_offset":5108,"ancestor_record_offset":4636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":4636,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":4636,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3685,"record_offset":7364,"parent_record_offset":7336,"ancestor_record_offset":416,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":5256,"parent_record_offset":5108,"ancestor_record_offset":4636,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":9480,"parent_record_offset":9452,"ancestor_record_offset":7316,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":7316,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":4636,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3685,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":416,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3685,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":416,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=15
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4636,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=15
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":4636,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":6308,"parent_record_offset":6088,"ancestor_record_offset":4636,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5652,"parent_record_offset":5620,"ancestor_record_offset":4636,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":4636,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6196,"parent_record_offset":6088,"ancestor_record_offset":4636,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4636,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6516,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":4636,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":4636,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3685,"record_offset":1852,"parent_record_offset":1796,"ancestor_record_offset":416,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=15
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5680,"parent_record_offset":5620,"ancestor_record_offset":4636,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":4636,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5968,"parent_record_offset":5940,"ancestor_record_offset":4636,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":4636,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":6056,"parent_record_offset":5940,"ancestor_record_offset":4636,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5940,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5772,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6088,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":5800,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=36
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":5884,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":6424,"parent_record_offset":5728,"ancestor_record_offset":4636,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3685,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":416,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":5572,"parent_record_offset":5540,"ancestor_record_offset":4636,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3685,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":416,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":7316,"inlinee":5664,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::previous
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7316,"inlinee":5663,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$2::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":18252,"parent_record_offset":18176,"ancestor_record_offset":17704,"inlinee":5666,"source_line":996,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$3::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":28064,"parent_record_offset":27988,"ancestor_record_offset":27516,"inlinee":5669,"source_line":1005,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$4::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":5220,"parent_record_offset":5108,"ancestor_record_offset":4636,"inlinee":5658,"source_line":1017,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$6::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":7840,"parent_record_offset":7808,"ancestor_record_offset":7316,"inlinee":5661,"source_line":1036,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":5704,"parent_record_offset":4608,"ancestor_record_offset":416,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":2924,"parent_record_offset":2888,"ancestor_record_offset":416,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":416,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":416,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":5768,"parent_record_offset":5740,"ancestor_record_offset":416,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":4640,"parent_record_offset":4608,"ancestor_record_offset":416,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":4608,"parent_record_offset":4576,"ancestor_record_offset":416,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3685,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":416,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":416,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4636,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4636,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4800,"parent_record_offset":4748,"ancestor_record_offset":4636,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=22
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4636,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3012,"parent_record_offset":2888,"ancestor_record_offset":416,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::HintTracker::confirm
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":416,"inlinee":17488,"source_line":165,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":6572,"parent_record_offset":5540,"ancestor_record_offset":4636,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":1644,"rva":120928,"va":5368830048}
// 编译实例 RVAs: 0x1d860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t398397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e83d140600","size":5,"soft_fail":false,"target_va":5369228752,"va":5368830350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t398538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8ca140600","size":5,"soft_fail":false,"target_va":5369229136,"va":5368830593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t782584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8f8f00b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368830931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t785889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8e1fd0b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368830186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t786272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e860ff0b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368830203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t983098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e83a000f00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368830449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2468482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e882aa2500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368830361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ef4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2955661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e88d192d00","size":5,"soft_fail":false,"target_va":5371786448,"va":5368830782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3918422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e856ca3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368831285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4314879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8ffd64100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368830620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4319410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8b2e84100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368830425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4319573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e855e94100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368830278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4328650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8ca0c4200","size":5,"soft_fail":false,"target_va":5373158800,"va":5368830145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4348910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8ee5b4200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368830333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4348843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8ab5b4200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368830576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34147542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8d60c0902","size":5,"soft_fail":false,"target_va":5402978880,"va":5368831333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34148748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e88c110902","size":5,"soft_fail":false,"target_va":5402980128,"va":5368831375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4328589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e88d0c4200","size":5,"soft_fail":false,"target_va":5373158880,"va":5368830286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4328214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8160b4200","size":5,"soft_fail":false,"target_va":5373158864,"va":5368830645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4330645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e895144200","size":5,"soft_fail":false,"target_va":5373160864,"va":5368830214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4331446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8b6174200","size":5,"soft_fail":false,"target_va":5373161712,"va":5368830261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4351284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e834654200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368830247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4351187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8d3644200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368830232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4315158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e816d84100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368830885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4315818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8aada4100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368830417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4315730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e852da4100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368830377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4315570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e8b2d94100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368830393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4315765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e875da4100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368830406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4352380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368830048,"raw_hex":"e87c694200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368830303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":3960,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":1912,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":2744,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":2024,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":2172,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":2204,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5604,"source_line":117,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":2292,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5609,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::impl$4::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":3872,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":5648,"source_line":123,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":1644,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":2628,"parent_record_offset":2572,"ancestor_record_offset":1644,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":1644,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":1644,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":2376,"parent_record_offset":2324,"ancestor_record_offset":1644,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":2324,"parent_record_offset":2292,"ancestor_record_offset":1644,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":1644,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":1644,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":1644,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":2404,"parent_record_offset":2324,"ancestor_record_offset":1644,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":2432,"parent_record_offset":2324,"ancestor_record_offset":1644,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":1644,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":1644,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":3804,"parent_record_offset":2744,"ancestor_record_offset":1644,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":2496,"parent_record_offset":2324,"ancestor_record_offset":1644,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":1644,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":1644,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":2460,"parent_record_offset":2324,"ancestor_record_offset":1644,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":1644,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":2232,"parent_record_offset":2204,"ancestor_record_offset":1644,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":1644,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":1644,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1644,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":3512,"parent_record_offset":3292,"ancestor_record_offset":1644,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":1644,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":1644,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3400,"parent_record_offset":3292,"ancestor_record_offset":1644,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":1644,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3720,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":1644,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":1644,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":2884,"parent_record_offset":2824,"ancestor_record_offset":1644,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":1644,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":1644,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":1644,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":3260,"parent_record_offset":3144,"ancestor_record_offset":1644,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":3144,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":2976,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":3292,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":3004,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":3088,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":3628,"parent_record_offset":2932,"ancestor_record_offset":1644,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":2776,"parent_record_offset":2744,"ancestor_record_offset":1644,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_controls::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_controls.rs
//     evidence={"module":15,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":1644,"inlinee":5650,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_controls::_::impl$0::previous
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_controls.rs
//     evidence={"module":15,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1644,"inlinee":5608,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":1644,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1644,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":1988,"parent_record_offset":1936,"ancestor_record_offset":1644,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1644,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":3776,"parent_record_offset":2744,"ancestor_record_offset":1644,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160F
// 首条位置: {"module":15,"inlinee":5647}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
// 记录形式: compiled_procedure; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":25168,"rva":133840,"va":5368842960}
// 编译实例 RVAs: 0x20ad0, 0x1b88800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8e7e10500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368843236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e86ae20500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368843489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=9
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t769811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e813bf0b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368843704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t772996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e884cb0b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368843079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t773379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e803cd0b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368843096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27962200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8a85455fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5397579843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t970206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8decd0e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368843341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2455587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e823782500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368843256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24831870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8821885fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397581561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3905679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e88f983b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368844028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4301982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e89ea44100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368843517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4306515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e853b64100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368843320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4306680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8f8b64100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368843171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4315757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e86dda4100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368843038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4336024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e898294200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368843219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4335947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e84b294200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368843472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4338918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8e6344200","size":5,"soft_fail":false,"target_va":5373182480,"va":5368843557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4338487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e837334200","size":5,"soft_fail":false,"target_va":5373182496,"va":5368844004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4338937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8f9344200","size":5,"soft_fail":false,"target_va":5373182512,"va":5368843570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18017268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e80c14edfe","size":5,"soft_fail":false,"target_va":5379563248,"va":5397580511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18014648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8481eedfe","size":5,"soft_fail":false,"target_va":5379566416,"va":5397581059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18008389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8bb36edfe","size":5,"soft_fail":false,"target_va":5379572176,"va":5397580560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18007490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e83e3aedfe","size":5,"soft_fail":false,"target_va":5379573040,"va":5397580525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6011760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e870bb5b00","size":5,"soft_fail":false,"target_va":5403593360,"va":5397581595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5395443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8f3535200","size":5,"soft_fail":false,"target_va":5402978144,"va":5397582696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20055568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8f0f9cdfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5397582683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$::clone_into
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20050077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8630fcefe","size":5,"soft_fail":false,"target_va":5377530320,"va":5397580392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5396295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e847575200","size":5,"soft_fail":false,"target_va":5402978912,"va":5397582612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34134809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e819db0802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368844066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21428649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e85706b9fe","size":5,"soft_fail":false,"target_va":5376152880,"va":5397581524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21537169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e86f5eb7fe","size":5,"soft_fail":false,"target_va":5376044672,"va":5397581836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21536626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e88e60b7fe","size":5,"soft_fail":false,"target_va":5376043968,"va":5397580589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21536085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e8ab62b7fe","size":5,"soft_fail":false,"target_va":5376044544,"va":5397580624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21536148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397579776,"raw_hex":"e86c62b7fe","size":5,"soft_fail":false,"target_va":5376044512,"va":5397580655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4315696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e830da4100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368843179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4315317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8b5d84100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368843542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t4317752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e838e24100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368843107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4318553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e859e54100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368843154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4338391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8d7324200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368843140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4338294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e876324200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368843125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4302385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e831a64100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368843658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4302923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e84ba84100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368843312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4302835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e8f3a74100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368843272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4302675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e853a74100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368843288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4302870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e816a84100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368843301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4339487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368842960,"raw_hex":"e81f374200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368843196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":25340,"parent_record_offset":25168,"ancestor_record_offset":25168,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":27032,"parent_record_offset":27008,"ancestor_record_offset":25168,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3685,"record_offset":7288,"parent_record_offset":7240,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":27008,"parent_record_offset":26952,"ancestor_record_offset":25168,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":26980,"parent_record_offset":26952,"ancestor_record_offset":25168,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7240,"parent_record_offset":7204,"ancestor_record_offset":244,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7204,"parent_record_offset":7168,"ancestor_record_offset":244,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3685,"record_offset":7408,"parent_record_offset":7204,"ancestor_record_offset":244,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":26952,"parent_record_offset":26924,"ancestor_record_offset":25168,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7128,"parent_record_offset":7092,"ancestor_record_offset":244,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7092,"parent_record_offset":7056,"ancestor_record_offset":244,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7020,"parent_record_offset":6984,"ancestor_record_offset":244,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":7056,"parent_record_offset":7020,"ancestor_record_offset":244,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3685,"record_offset":6984,"parent_record_offset":6948,"ancestor_record_offset":244,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3685,"record_offset":6948,"parent_record_offset":6904,"ancestor_record_offset":244,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3685,"record_offset":7168,"parent_record_offset":7128,"ancestor_record_offset":244,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":25168,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":26032,"parent_record_offset":26008,"ancestor_record_offset":25168,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":26116,"parent_record_offset":26092,"ancestor_record_offset":25168,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":26056,"parent_record_offset":26032,"ancestor_record_offset":25168,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":26228,"parent_record_offset":26204,"ancestor_record_offset":25168,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":26808,"parent_record_offset":25748,"ancestor_record_offset":25168,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3685,"record_offset":7448,"parent_record_offset":7128,"ancestor_record_offset":244,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":26876,"parent_record_offset":25340,"ancestor_record_offset":25168,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":26476,"parent_record_offset":26452,"ancestor_record_offset":25168,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":26368,"parent_record_offset":26344,"ancestor_record_offset":25168,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3685,"record_offset":7364,"parent_record_offset":7336,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":25168,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3685,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":244,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3685,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":244,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":25828,"parent_record_offset":25804,"ancestor_record_offset":25168,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":25804,"parent_record_offset":25780,"ancestor_record_offset":25168,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":26516,"parent_record_offset":26296,"ancestor_record_offset":25168,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":25860,"parent_record_offset":25828,"ancestor_record_offset":25168,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26428,"parent_record_offset":26404,"ancestor_record_offset":25168,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26404,"parent_record_offset":26296,"ancestor_record_offset":25168,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26344,"parent_record_offset":26320,"ancestor_record_offset":25168,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26724,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26320,"parent_record_offset":26296,"ancestor_record_offset":25168,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":25912,"parent_record_offset":25888,"ancestor_record_offset":25168,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3685,"record_offset":1852,"parent_record_offset":1796,"ancestor_record_offset":244,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":25888,"parent_record_offset":25828,"ancestor_record_offset":25168,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":25168,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":26176,"parent_record_offset":26148,"ancestor_record_offset":25168,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":26204,"parent_record_offset":26176,"ancestor_record_offset":25168,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":26264,"parent_record_offset":26148,"ancestor_record_offset":25168,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":26148,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":25980,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":26296,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":26008,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":26092,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":26632,"parent_record_offset":25936,"ancestor_record_offset":25168,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3685,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":244,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":25780,"parent_record_offset":25748,"ancestor_record_offset":25168,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3685,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":244,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":2752,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17189,"source_line":252,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":5704,"parent_record_offset":4608,"ancestor_record_offset":244,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":6844,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":25420,"parent_record_offset":25340,"ancestor_record_offset":25168,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":244,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":244,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":672,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":1700,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":4640,"parent_record_offset":4608,"ancestor_record_offset":244,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":4608,"parent_record_offset":4576,"ancestor_record_offset":244,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":25748,"parent_record_offset":25340,"ancestor_record_offset":25168,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3685,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":244,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3685,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":244,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":25168,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":25468,"parent_record_offset":25444,"ancestor_record_offset":25168,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":25496,"parent_record_offset":25444,"ancestor_record_offset":25168,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":25168,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":7948,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17223,"source_line":208,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3316,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3136,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17224,"source_line":214,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3196,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17222,"source_line":202,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":25532,"parent_record_offset":25340,"ancestor_record_offset":25168,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3256,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":25680,"parent_record_offset":25340,"ancestor_record_offset":25168,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::on_click
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":4576,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17218,"source_line":170,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":2864,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":3012,"parent_record_offset":2888,"ancestor_record_offset":244,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3685,"record_offset":5740,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::HintTracker::confirm
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":244,"inlinee":17488,"source_line":165,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":2808,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17506,"source_line":176,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$3::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":7532,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17501,"source_line":94,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$3::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":6904,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17503,"source_line":102,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$3::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":3685,"record_offset":11520,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":17502,"source_line":98,"archive":"_pdb/A/modules/03685.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":26780,"parent_record_offset":25748,"ancestor_record_offset":25168,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":12660,"rva":127008,"va":5368836128}
// 编译实例 RVAs: 0x1f020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t392344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e898fc0500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368836403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t392476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e81cfd0500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368836655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t776554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e86ad90b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368836961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t779836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e83ce60b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368836239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t780219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8bbe70b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368836256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t977036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e88ce80e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368836511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2462420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8d4922500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368836423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3912426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8eab23b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368837281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4308816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e850bf4100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368836683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4313348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e804d14100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368836487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4313520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8b0d14100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368836331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4322597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e825f54100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368836198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4342857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e849444200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368836386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4342781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8fd434200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368836638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140664780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6575284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8b4546400","size":5,"soft_fail":false,"target_va":5375412096,"va":5368836807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140666d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6584490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8aa786400","size":5,"soft_fail":false,"target_va":5375421744,"va":5368837249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140666d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6585033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8c97a6400","size":5,"soft_fail":false,"target_va":5375421760,"va":5368836722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34141553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e871f50802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368837322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34142759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e827fa0802","size":5,"soft_fail":false,"target_va":5402980128,"va":5368837364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4322536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8e8f44100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368836339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4322151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e867f34100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368836708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4324592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8f0fc4100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368836267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4325393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e811004200","size":5,"soft_fail":false,"target_va":5373161712,"va":5368836314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4345231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e88f4d4200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368836300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4345134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e82e4d4200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368836285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4309128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e888c04100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368836915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4309756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8fcc24100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368836479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4309668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8a4c24100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368836439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4309508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e804c24100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368836455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4309703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8c7c24100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368836468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4346327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368836128,"raw_hex":"e8d7514200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368836356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":12876,"parent_record_offset":12660,"ancestor_record_offset":12660,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":12660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":13648,"parent_record_offset":13592,"ancestor_record_offset":12660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":12660,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":13592,"parent_record_offset":13564,"ancestor_record_offset":12660,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":13396,"parent_record_offset":13344,"ancestor_record_offset":12660,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":13344,"parent_record_offset":13312,"ancestor_record_offset":12660,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":12660,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":12660,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":14132,"parent_record_offset":14108,"ancestor_record_offset":12660,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":13424,"parent_record_offset":13344,"ancestor_record_offset":12660,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":13452,"parent_record_offset":13344,"ancestor_record_offset":12660,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":14072,"parent_record_offset":14048,"ancestor_record_offset":12660,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":12660,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":14824,"parent_record_offset":13764,"ancestor_record_offset":12660,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":13516,"parent_record_offset":13344,"ancestor_record_offset":12660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":12660,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":14384,"parent_record_offset":14360,"ancestor_record_offset":12660,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":13480,"parent_record_offset":13344,"ancestor_record_offset":12660,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":14920,"parent_record_offset":14892,"ancestor_record_offset":12660,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":13280,"parent_record_offset":13252,"ancestor_record_offset":12660,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":12660,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":12660,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":12660,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":14532,"parent_record_offset":14312,"ancestor_record_offset":12660,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":13876,"parent_record_offset":13844,"ancestor_record_offset":12660,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":12660,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14420,"parent_record_offset":14312,"ancestor_record_offset":12660,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":12660,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14740,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":12660,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":12660,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":13904,"parent_record_offset":13844,"ancestor_record_offset":12660,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":13952,"parent_record_offset":13928,"ancestor_record_offset":12660,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":14192,"parent_record_offset":14164,"ancestor_record_offset":12660,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":14220,"parent_record_offset":14192,"ancestor_record_offset":12660,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":14280,"parent_record_offset":14164,"ancestor_record_offset":12660,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":14164,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":13996,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":14312,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":14024,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":14108,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":14648,"parent_record_offset":13952,"ancestor_record_offset":12660,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":13796,"parent_record_offset":13764,"ancestor_record_offset":12660,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":12960,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":13764,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":12660,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":12660,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":13036,"parent_record_offset":12984,"ancestor_record_offset":12660,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":12660,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":13072,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":13220,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":13252,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5604,"source_line":117,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":13312,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5609,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":14892,"parent_record_offset":12876,"ancestor_record_offset":12660,"inlinee":5648,"source_line":123,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":14796,"parent_record_offset":13764,"ancestor_record_offset":12660,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":9948,"rva":125584,"va":5368834704}
// 编译实例 RVAs: 0x1ea90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t393768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e828020600","size":5,"soft_fail":false,"target_va":5369228752,"va":5368834979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t393900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8ac020600","size":5,"soft_fail":false,"target_va":5369229136,"va":5368835231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t777978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8fade0b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368835537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t781260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8cceb0b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368834815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t781643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e84bed0b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368834832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t978460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e81cee0e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368835087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2463844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e864982500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368834999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3913850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e87ab83b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368835857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4310240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8e0c44100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368835259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4314772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e894d64100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368835063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4314944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e840d74100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368834907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4324021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8b5fa4100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368834774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4344281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8d9494200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368834962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4344205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e88d494200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368835214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b3490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5850964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e854475900","size":5,"soft_fail":false,"target_va":5374686352,"va":5368835383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b3820
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5851434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e82a495900","size":5,"soft_fail":false,"target_va":5374687264,"va":5368835825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b3830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5851977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8494b5900","size":5,"soft_fail":false,"target_va":5374687280,"va":5368835298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34142977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e801fb0802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368835898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34144183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8b7ff0802","size":5,"soft_fail":false,"target_va":5402980128,"va":5368835940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4323960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e878fa4100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368834915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4323575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8f7f84100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368835284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4326016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e880024200","size":5,"soft_fail":false,"target_va":5373160864,"va":5368834843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4326817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8a1054200","size":5,"soft_fail":false,"target_va":5373161712,"va":5368834890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4346655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e81f534200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368834876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4346558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e8be524200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368834861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4310552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e818c64100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368835491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4311180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e88cc84100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368835055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4311092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e834c84100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368835015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4310932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e894c74100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368835031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4311127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e857c84100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368835044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4347751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368834704,"raw_hex":"e867574200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368834932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":10176,"parent_record_offset":9948,"ancestor_record_offset":9948,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":9948,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":10948,"parent_record_offset":10892,"ancestor_record_offset":9948,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":10920,"parent_record_offset":10892,"ancestor_record_offset":9948,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":10892,"parent_record_offset":10864,"ancestor_record_offset":9948,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":10696,"parent_record_offset":10644,"ancestor_record_offset":9948,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":10644,"parent_record_offset":10612,"ancestor_record_offset":9948,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":9948,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":11348,"parent_record_offset":11324,"ancestor_record_offset":9948,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":11432,"parent_record_offset":11408,"ancestor_record_offset":9948,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":10724,"parent_record_offset":10644,"ancestor_record_offset":9948,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":10752,"parent_record_offset":10644,"ancestor_record_offset":9948,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":11372,"parent_record_offset":11348,"ancestor_record_offset":9948,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":9948,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":12124,"parent_record_offset":11064,"ancestor_record_offset":9948,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":10816,"parent_record_offset":10644,"ancestor_record_offset":9948,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":9948,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":9948,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":10780,"parent_record_offset":10644,"ancestor_record_offset":9948,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":12220,"parent_record_offset":12192,"ancestor_record_offset":9948,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":10580,"parent_record_offset":10552,"ancestor_record_offset":9948,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":9948,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":9948,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":9948,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":11832,"parent_record_offset":11612,"ancestor_record_offset":9948,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":11176,"parent_record_offset":11144,"ancestor_record_offset":9948,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":9948,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11720,"parent_record_offset":11612,"ancestor_record_offset":9948,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":9948,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":12040,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":9948,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":9948,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11204,"parent_record_offset":11144,"ancestor_record_offset":9948,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":9948,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":11492,"parent_record_offset":11464,"ancestor_record_offset":9948,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":11520,"parent_record_offset":11492,"ancestor_record_offset":9948,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":11580,"parent_record_offset":11464,"ancestor_record_offset":9948,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":11464,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11296,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":11612,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":11324,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":11408,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":11948,"parent_record_offset":11252,"ancestor_record_offset":9948,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":11096,"parent_record_offset":11064,"ancestor_record_offset":9948,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":10260,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":11064,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":11904,"parent_record_offset":11880,"ancestor_record_offset":9948,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":9948,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":10336,"parent_record_offset":10284,"ancestor_record_offset":9948,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":9948,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":10372,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":10520,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":10552,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5604,"source_line":117,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":10612,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5609,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":12192,"parent_record_offset":10176,"ancestor_record_offset":9948,"inlinee":5648,"source_line":123,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":12096,"parent_record_offset":11064,"ancestor_record_offset":9948,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":7104,"rva":123984,"va":5368833104}
// 编译实例 RVAs: 0x1e450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t395375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e86f080600","size":5,"soft_fail":false,"target_va":5369228752,"va":5368833372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t395503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8ef080600","size":5,"soft_fail":false,"target_va":5369229136,"va":5368833628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t779425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8a1e40b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368834090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t782860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e80cf20b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368833215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t783243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e88bf30b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368833232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t980067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e863f40e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368833480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2465451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8ab9e2500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368833392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3915282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e812be3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368834425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4311843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e823cb4100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368833656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4316379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8dbdc4100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368833456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4316544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e880dd4100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368833307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4325621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8f5004200","size":5,"soft_fail":false,"target_va":5373158800,"va":5368833174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4345888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e820504200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368833355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4345808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8d04f4200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368833611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8749979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e89b838500","size":5,"soft_fail":false,"target_va":5377583920,"va":5368833936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34144409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e899000902","size":5,"soft_fail":false,"target_va":5402978880,"va":5368834466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34145615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e84f050902","size":5,"soft_fail":false,"target_va":5402980128,"va":5368834508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7319059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e813ae6f00","size":5,"soft_fail":false,"target_va":5376152880,"va":5368833816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4325560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8b8004200","size":5,"soft_fail":false,"target_va":5373158880,"va":5368833315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4325178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e83aff4100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368833681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4327616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8c0084200","size":5,"soft_fail":false,"target_va":5373160864,"va":5368833243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4328417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8e10b4200","size":5,"soft_fail":false,"target_va":5373161712,"va":5368833290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4348255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e85f594200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368833276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4348158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8fe584200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368833261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4311999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8bfcb4100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368834044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4312787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8d3ce4100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368833448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4312699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e87bce4100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368833408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4312539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8dbcd4100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368833424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4312734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e89ece4100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368833437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4349351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368833104,"raw_hex":"e8a75d4200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368833332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":7316,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":7104,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":8120,"parent_record_offset":8064,"ancestor_record_offset":7104,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":8092,"parent_record_offset":8064,"ancestor_record_offset":7104,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":8064,"parent_record_offset":8036,"ancestor_record_offset":7104,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":7888,"parent_record_offset":7740,"ancestor_record_offset":7104,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":7740,"parent_record_offset":7704,"ancestor_record_offset":7104,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":7104,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":7104,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":7104,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":7920,"parent_record_offset":7740,"ancestor_record_offset":7104,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$54::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":7808,"parent_record_offset":7740,"ancestor_record_offset":7104,"inlinee":5611,"source_line":2337,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":7104,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":7104,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":9384,"parent_record_offset":8324,"ancestor_record_offset":7104,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":7988,"parent_record_offset":7740,"ancestor_record_offset":7104,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":7104,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":7104,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":7952,"parent_record_offset":7740,"ancestor_record_offset":7104,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":9480,"parent_record_offset":9452,"ancestor_record_offset":7104,"inlinee":5649,"source_line":133,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":15,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":7104,"inlinee":5606,"source_line":163,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":7104,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":7104,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":7104,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":9092,"parent_record_offset":8872,"ancestor_record_offset":7104,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":8436,"parent_record_offset":8404,"ancestor_record_offset":7104,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":9004,"parent_record_offset":8980,"ancestor_record_offset":7104,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8980,"parent_record_offset":8872,"ancestor_record_offset":7104,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":7104,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":9300,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":7104,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":7104,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8464,"parent_record_offset":8404,"ancestor_record_offset":7104,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":7104,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":7104,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":7104,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":8840,"parent_record_offset":8724,"ancestor_record_offset":7104,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":8724,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8556,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":8872,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":8584,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":8668,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":9208,"parent_record_offset":8512,"ancestor_record_offset":7104,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":8356,"parent_record_offset":8324,"ancestor_record_offset":7104,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":7104,"inlinee":5664,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::_::impl$0::previous
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7104,"inlinee":5663,"source_line":974,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$6::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":7840,"parent_record_offset":7808,"ancestor_record_offset":7104,"inlinee":5661,"source_line":1036,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":7412,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":8324,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":7104,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":7104,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":7488,"parent_record_offset":7436,"ancestor_record_offset":7104,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":7104,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":7524,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":7672,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":8200,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5604,"source_line":117,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":7704,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5609,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$4::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":9452,"parent_record_offset":7316,"ancestor_record_offset":7104,"inlinee":5648,"source_line":123,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":9356,"parent_record_offset":8324,"ancestor_record_offset":7104,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":15360,"rva":128432,"va":5368837552}
// 编译实例 RVAs: 0x1f5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t390925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e80df70500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368837822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t391056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e890f70500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368838075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t775227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e83bd40b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368838288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t778415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8afe00b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368837660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t778798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e82ee20b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368837677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t975620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e804e30e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368837927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2461001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8498d2500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368837842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3911104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8c0ad3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368838603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4307396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8c4b94100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368838103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4311929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e879cb4100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368837906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4312100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e824cc4100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368837751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4321176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e898ef4100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368837619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4341438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8be3e4200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368837805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4341361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8713e4200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368838058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443f50
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4343617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e841474200","size":5,"soft_fail":false,"target_va":5373181776,"va":5368838154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4343821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e80d484200","size":5,"soft_fail":false,"target_va":5373181968,"va":5368838142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140444110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4343640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e858474200","size":5,"soft_fail":false,"target_va":5373182224,"va":5368838579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34140234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e84af00802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368838641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4321116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e85cef4100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368837759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4320731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8dbed4100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368838128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4323171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e863f74100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368837688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4323972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e884fa4100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368837735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4343810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e802484200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368837721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4343713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e8a1474200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368837706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4307801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e859bb4100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368838242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4308337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e871bd4100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368837898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4308249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e819bd4100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368837858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4308089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e879bc4100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368837874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4308284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e83cbd4100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368837887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4344907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368837552,"raw_hex":"e84b4c4200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368837776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":15540,"parent_record_offset":15360,"ancestor_record_offset":15360,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":15360,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":17208,"parent_record_offset":17152,"ancestor_record_offset":15360,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":15360,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":15360,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":15360,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":16232,"parent_record_offset":16208,"ancestor_record_offset":15360,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":15360,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15360,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":15360,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":17008,"parent_record_offset":15948,"ancestor_record_offset":15360,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":17076,"parent_record_offset":15540,"ancestor_record_offset":15360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":15360,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":16568,"parent_record_offset":16544,"ancestor_record_offset":15360,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":16740,"parent_record_offset":16716,"ancestor_record_offset":15360,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":15360,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":15360,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":16716,"parent_record_offset":16496,"ancestor_record_offset":15360,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":16060,"parent_record_offset":16028,"ancestor_record_offset":15360,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16628,"parent_record_offset":16604,"ancestor_record_offset":15360,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16604,"parent_record_offset":16496,"ancestor_record_offset":15360,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":15360,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16924,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":15360,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":15360,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16088,"parent_record_offset":16028,"ancestor_record_offset":15360,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":15360,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":16376,"parent_record_offset":16348,"ancestor_record_offset":15360,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":16404,"parent_record_offset":16376,"ancestor_record_offset":15360,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":16464,"parent_record_offset":16348,"ancestor_record_offset":15360,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":16348,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16180,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":16496,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":16208,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":16292,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":16832,"parent_record_offset":16136,"ancestor_record_offset":15360,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":15980,"parent_record_offset":15948,"ancestor_record_offset":15360,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":15620,"parent_record_offset":15540,"ancestor_record_offset":15360,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":15948,"parent_record_offset":15540,"ancestor_record_offset":15360,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":15360,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":15360,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":15696,"parent_record_offset":15644,"ancestor_record_offset":15360,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":15644,"parent_record_offset":15620,"ancestor_record_offset":15360,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":15732,"parent_record_offset":15540,"ancestor_record_offset":15360,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":15880,"parent_record_offset":15540,"ancestor_record_offset":15360,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":16980,"parent_record_offset":15948,"ancestor_record_offset":15360,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":20156,"rva":131120,"va":5368840240}
// 编译实例 RVAs: 0x20030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t388244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e894ec0500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368840503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t388375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e817ed0500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368840756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t772506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e89ac90b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368841009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t775727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e82fd60b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368840348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t776110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8aed70b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368840365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t791568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e810140c00","size":5,"soft_fail":false,"target_va":5369632448,"va":5368840875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t972939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e88bd80e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368840608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2458320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8d0822500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368840523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3908373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e815a33b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368841334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4304715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e84baf4100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368840784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4309248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e800c14100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368840587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4309412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8a4c14100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368840439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4318488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e818e54100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368840307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4338757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e845344200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368840486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4338680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8f8334200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368840739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34137488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e890e50802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368841387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34137964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e86ce70802","size":5,"soft_fail":false,"target_va":5402979344,"va":5368841375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4318428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8dce44100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368840447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4318050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e862e34100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368840809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4320483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8e3ec4100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368840376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4321284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e804f04100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368840423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4341122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8823d4200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368840409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4341025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8213d4200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368840394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4305080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8b8b04100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368840963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4305656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8f8b24100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368840579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4305568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8a0b24100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368840539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4305408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e800b24100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368840555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4305603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8c3b24100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368840568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4342219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368840240,"raw_hex":"e8cb414200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368840464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":20352,"parent_record_offset":20156,"ancestor_record_offset":20156,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":20156,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":22196,"parent_record_offset":22140,"ancestor_record_offset":20156,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":15,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20156,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":22168,"parent_record_offset":22140,"ancestor_record_offset":20156,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":22140,"parent_record_offset":22112,"ancestor_record_offset":20156,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":20156,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":21724,"parent_record_offset":21700,"ancestor_record_offset":20156,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":20156,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":20156,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":21216,"parent_record_offset":21192,"ancestor_record_offset":20156,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":20752,"parent_record_offset":20728,"ancestor_record_offset":20156,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":21388,"parent_record_offset":21364,"ancestor_record_offset":20156,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":21968,"parent_record_offset":20908,"ancestor_record_offset":20156,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":22064,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":20156,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":20156,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":21700,"parent_record_offset":21676,"ancestor_record_offset":20156,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":20988,"parent_record_offset":20964,"ancestor_record_offset":20156,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":20964,"parent_record_offset":20940,"ancestor_record_offset":20156,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":21676,"parent_record_offset":21456,"ancestor_record_offset":20156,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":21020,"parent_record_offset":20988,"ancestor_record_offset":20156,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":20156,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21564,"parent_record_offset":21456,"ancestor_record_offset":20156,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21504,"parent_record_offset":21480,"ancestor_record_offset":20156,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21884,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21480,"parent_record_offset":21456,"ancestor_record_offset":20156,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21072,"parent_record_offset":21048,"ancestor_record_offset":20156,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21048,"parent_record_offset":20988,"ancestor_record_offset":20156,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":20156,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":21336,"parent_record_offset":21308,"ancestor_record_offset":20156,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":21364,"parent_record_offset":21336,"ancestor_record_offset":20156,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":21424,"parent_record_offset":21308,"ancestor_record_offset":20156,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":21308,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21140,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":21456,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":21168,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":21252,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":21792,"parent_record_offset":21096,"ancestor_record_offset":20156,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":20940,"parent_record_offset":20908,"ancestor_record_offset":20156,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":20436,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":20908,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":20156,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":20484,"parent_record_offset":20460,"ancestor_record_offset":20156,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":20512,"parent_record_offset":20460,"ancestor_record_offset":20156,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":20156,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":20548,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":20696,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":20728,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":20784,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":22036,"parent_record_offset":20352,"ancestor_record_offset":20156,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":21940,"parent_record_offset":20908,"ancestor_record_offset":20156,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":4420,"rva":122464,"va":5368831584}
// 编译实例 RVAs: 0x1de60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t396895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e85f0e0600","size":5,"soft_fail":false,"target_va":5369228752,"va":5368831852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t397023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8df0e0600","size":5,"soft_fail":false,"target_va":5369229136,"va":5368832108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t781027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8e3ea0b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368832488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t784380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8fcf70b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368831695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t784763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e87bf90b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368831712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t981587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e853fa0e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368831960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2466971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e89ba42500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368831872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3916885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e855c43b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368832822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4313363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e813d14100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368832136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4317899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8cbe24100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368831936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4318064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e870e34100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368831787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4327141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8e5064200","size":5,"soft_fail":false,"target_va":5373158800,"va":5368831654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4347408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e810564200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368831835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4347328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8c0554200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368832091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8751600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8f0898500","size":5,"soft_fail":false,"target_va":5377583920,"va":5368832315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8778285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e82df28500","size":5,"soft_fail":false,"target_va":5377610624,"va":5368832334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34145997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8cd060902","size":5,"soft_fail":false,"target_va":5402978880,"va":5368832878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34146473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8a9080902","size":5,"soft_fail":false,"target_va":5402979344,"va":5368832866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34147203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8830b0902","size":5,"soft_fail":false,"target_va":5402980128,"va":5368832920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7320575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8ffb36f00","size":5,"soft_fail":false,"target_va":5376152880,"va":5368832300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4327080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8a8064200","size":5,"soft_fail":false,"target_va":5373158880,"va":5368831795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4326698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e82a054200","size":5,"soft_fail":false,"target_va":5373158864,"va":5368832161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4329136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8b00e4200","size":5,"soft_fail":false,"target_va":5373160864,"va":5368831723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4329937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8d1114200","size":5,"soft_fail":false,"target_va":5373161712,"va":5368831770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4349775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e84f5f4200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368831756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4349678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8ee5e4200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368831741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4313601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e801d24100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368832442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4314307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8c3d44100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368831928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4314219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e86bd44100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368831888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4314059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e8cbd34100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368831904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4314254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e88ed44100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368831917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4350871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368831584,"raw_hex":"e897634200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368831812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":4636,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":4420,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":5424,"parent_record_offset":5368,"ancestor_record_offset":4420,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":4420,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4420,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5164,"parent_record_offset":5108,"ancestor_record_offset":4420,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":5108,"parent_record_offset":5072,"ancestor_record_offset":4420,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4420,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4420,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":4420,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":5192,"parent_record_offset":5108,"ancestor_record_offset":4420,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4420,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4420,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":6600,"parent_record_offset":5540,"ancestor_record_offset":4420,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":5292,"parent_record_offset":5108,"ancestor_record_offset":4420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":4420,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":4420,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":5256,"parent_record_offset":5108,"ancestor_record_offset":4420,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":4420,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4420,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":4420,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":6308,"parent_record_offset":6088,"ancestor_record_offset":4420,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":5652,"parent_record_offset":5620,"ancestor_record_offset":4420,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":4420,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6196,"parent_record_offset":6088,"ancestor_record_offset":4420,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4420,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6516,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":4420,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":4420,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5680,"parent_record_offset":5620,"ancestor_record_offset":4420,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":4420,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5968,"parent_record_offset":5940,"ancestor_record_offset":4420,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":4420,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":6056,"parent_record_offset":5940,"ancestor_record_offset":4420,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":5940,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":5772,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":6088,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":5800,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":5884,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":6424,"parent_record_offset":5728,"ancestor_record_offset":4420,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":5572,"parent_record_offset":5540,"ancestor_record_offset":4420,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$4::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":5220,"parent_record_offset":5108,"ancestor_record_offset":4420,"inlinee":5658,"source_line":1017,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4724,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":5540,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4420,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4420,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4800,"parent_record_offset":4748,"ancestor_record_offset":4420,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4420,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":4836,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":4984,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":5016,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":5072,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":6668,"parent_record_offset":4636,"ancestor_record_offset":4420,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":6572,"parent_record_offset":5540,"ancestor_record_offset":4420,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":22480,"rva":132368,"va":5368841488}
// 编译实例 RVAs: 0x20510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8746257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e811758500","size":5,"soft_fail":false,"target_va":5377588464,"va":5368842202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t386991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8afe70500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368841756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t387119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e82fe80500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368842012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t771159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e857c40b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368842356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t774476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e84cd10b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368841599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t774859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8cbd20b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368841616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t971683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8a3d30e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368841864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2457067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8eb7d2500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368841776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3907029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8d59d3b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368842678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4303459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e863aa4100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368842040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4307995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e81bbc4100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368841840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4308160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8c0bc4100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368841691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4317237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e835e04100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368841558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4337504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8602f4200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368841739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4337424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8102f4200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368841995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34136141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e84de00802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368842734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34136617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e829e20802","size":5,"soft_fail":false,"target_va":5402979344,"va":5368842722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34137347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e803e50802","size":5,"soft_fail":false,"target_va":5402980128,"va":5368842776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4317176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8f8df4100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368841699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4316794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e87ade4100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368842065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4319232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e800e84100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368841627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4320033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e821eb4100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368841674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4339871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e89f384200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368841660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4339774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e83e384200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368841645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4303733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e875ab4100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368842310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4304403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e813ae4100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368841832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4304315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8bbad4100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368841792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4304155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e81bad4100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368841808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4304350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8dead4100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368841821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4340967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368841488,"raw_hex":"e8e73c4200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368841716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":22684,"parent_record_offset":22480,"ancestor_record_offset":22480,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":23512,"parent_record_offset":23488,"ancestor_record_offset":22480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":23488,"parent_record_offset":23432,"ancestor_record_offset":22480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":22480,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":22480,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":23208,"parent_record_offset":23156,"ancestor_record_offset":22480,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":22480,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":23156,"parent_record_offset":23120,"ancestor_record_offset":22480,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":22480,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":22480,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":23972,"parent_record_offset":23948,"ancestor_record_offset":22480,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":23236,"parent_record_offset":23156,"ancestor_record_offset":22480,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":23264,"parent_record_offset":23156,"ancestor_record_offset":22480,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":23912,"parent_record_offset":23888,"ancestor_record_offset":22480,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":22480,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":22480,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":24664,"parent_record_offset":23604,"ancestor_record_offset":22480,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":23356,"parent_record_offset":23156,"ancestor_record_offset":22480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":24332,"parent_record_offset":24308,"ancestor_record_offset":22480,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":22480,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":23320,"parent_record_offset":23156,"ancestor_record_offset":22480,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":22480,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":23684,"parent_record_offset":23660,"ancestor_record_offset":22480,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":22480,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":24372,"parent_record_offset":24152,"ancestor_record_offset":22480,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":23716,"parent_record_offset":23684,"ancestor_record_offset":22480,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24284,"parent_record_offset":24260,"ancestor_record_offset":22480,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24260,"parent_record_offset":24152,"ancestor_record_offset":22480,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":22480,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24580,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24176,"parent_record_offset":24152,"ancestor_record_offset":22480,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":22480,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":23744,"parent_record_offset":23684,"ancestor_record_offset":22480,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":23792,"parent_record_offset":23768,"ancestor_record_offset":22480,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":24032,"parent_record_offset":24004,"ancestor_record_offset":22480,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":24060,"parent_record_offset":24032,"ancestor_record_offset":22480,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":24120,"parent_record_offset":24004,"ancestor_record_offset":22480,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":24004,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":23836,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":24152,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":23864,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":23948,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":24488,"parent_record_offset":23792,"ancestor_record_offset":22480,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":23636,"parent_record_offset":23604,"ancestor_record_offset":22480,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":22772,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":23604,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":24444,"parent_record_offset":24420,"ancestor_record_offset":22480,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":22480,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":22848,"parent_record_offset":22796,"ancestor_record_offset":22480,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":22480,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":22884,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":23032,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":23064,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":23120,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":24732,"parent_record_offset":22684,"ancestor_record_offset":22480,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":24636,"parent_record_offset":23604,"ancestor_record_offset":22480,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":27292,"rva":135040,"va":5368844160}
// 编译实例 RVAs: 0x20f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t384316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e83cdd0500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368844431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t384448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8c0dd0500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368844683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t768392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e888b90b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368845123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t771801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8d9c60b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368844274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t772184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e858c80b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368844291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t969011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e833c90e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368844536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2454392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e878732500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368844451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3904262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e806933b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368845445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4300788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8f49f4100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368844711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4305320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8a8b14100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368844515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4305485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e84db24100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368844366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4314562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8c2d54100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368844233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4334829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8ed244200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368844414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4334753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8a1244200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368844666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8735553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8414b8500","size":5,"soft_fail":false,"target_va":5377580528,"va":5368844970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34133374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e87ed50802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368845501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34133850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e85ad70802","size":5,"soft_fail":false,"target_va":5402979344,"va":5368845489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34134580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e834da0802","size":5,"soft_fail":false,"target_va":5402980128,"va":5368845543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4314501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e885d54100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368844374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4314124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e80cd44100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368844735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4316557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e88ddd4100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368844302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4317358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8aee04100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368844349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4337196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e82c2e4200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368844335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4337099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8cb2d4200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368844320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4300966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8a6a04100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368845077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4301728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8a0a34100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368844507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4301640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e848a34100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368844467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4301480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e8a8a24100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368844483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4301675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e86ba34100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368844496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4338292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368844160,"raw_hex":"e874324200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368844391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":27516,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":28312,"parent_record_offset":28288,"ancestor_record_offset":27292,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":28288,"parent_record_offset":28232,"ancestor_record_offset":27292,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":28260,"parent_record_offset":28232,"ancestor_record_offset":27292,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":28232,"parent_record_offset":28204,"ancestor_record_offset":27292,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":28036,"parent_record_offset":27988,"ancestor_record_offset":27292,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":27988,"parent_record_offset":27952,"ancestor_record_offset":27292,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":29220,"parent_record_offset":29196,"ancestor_record_offset":27292,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":27292,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":27292,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":27292,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":28712,"parent_record_offset":28688,"ancestor_record_offset":27292,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":27920,"parent_record_offset":27896,"ancestor_record_offset":27292,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":27292,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":29464,"parent_record_offset":28404,"ancestor_record_offset":27292,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":28156,"parent_record_offset":27988,"ancestor_record_offset":27292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":29132,"parent_record_offset":29108,"ancestor_record_offset":27292,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":29024,"parent_record_offset":29000,"ancestor_record_offset":27292,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":28120,"parent_record_offset":27988,"ancestor_record_offset":27292,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":29196,"parent_record_offset":29172,"ancestor_record_offset":27292,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":28484,"parent_record_offset":28460,"ancestor_record_offset":27292,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":28460,"parent_record_offset":28436,"ancestor_record_offset":27292,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":29172,"parent_record_offset":28952,"ancestor_record_offset":27292,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":28516,"parent_record_offset":28484,"ancestor_record_offset":27292,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":27292,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":29060,"parent_record_offset":28952,"ancestor_record_offset":27292,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":29000,"parent_record_offset":28976,"ancestor_record_offset":27292,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":29380,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28976,"parent_record_offset":28952,"ancestor_record_offset":27292,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":27292,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28544,"parent_record_offset":28484,"ancestor_record_offset":27292,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":27292,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":28832,"parent_record_offset":28804,"ancestor_record_offset":27292,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":28860,"parent_record_offset":28832,"ancestor_record_offset":27292,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":28920,"parent_record_offset":28804,"ancestor_record_offset":27292,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":28804,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28636,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":28952,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":28664,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":28748,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":29288,"parent_record_offset":28592,"ancestor_record_offset":27292,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":28436,"parent_record_offset":28404,"ancestor_record_offset":27292,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$3::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":28064,"parent_record_offset":27988,"ancestor_record_offset":27292,"inlinee":5669,"source_line":1005,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":27604,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":28404,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":29244,"parent_record_offset":29220,"ancestor_record_offset":27292,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":27292,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":27680,"parent_record_offset":27628,"ancestor_record_offset":27292,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":27292,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":27716,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":27864,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":27896,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":27952,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":29532,"parent_record_offset":27516,"ancestor_record_offset":27292,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":29436,"parent_record_offset":28404,"ancestor_record_offset":27292,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":17492,"rva":129616,"va":5368838736}
// 编译实例 RVAs: 0x1fa50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t389740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e86cf20500","size":5,"soft_fail":false,"target_va":5369228752,"va":5368839007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t389868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8ecf20500","size":5,"soft_fail":false,"target_va":5369229136,"va":5368839263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t773881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8f9ce0b00","size":5,"soft_fail":false,"target_va":5369613520,"va":5368839634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t777225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e809dc0b00","size":5,"soft_fail":false,"target_va":5369616080,"va":5368838850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t777608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e888dd0b00","size":5,"soft_fail":false,"target_va":5369616480,"va":5368838867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t974432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e860de0e00","size":5,"soft_fail":false,"target_va":5369813552,"va":5368839115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2459816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8a8882500","size":5,"soft_fail":false,"target_va":5371298848,"va":5368839027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3909749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e875a83b00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368839958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4306208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e820b54100","size":5,"soft_fail":false,"target_va":5373145504,"va":5368839291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4310744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8d8c64100","size":5,"soft_fail":false,"target_va":5373149840,"va":5368839091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4310909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e87dc74100","size":5,"soft_fail":false,"target_va":5373149856,"va":5368838942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4319986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8f2ea4100","size":5,"soft_fail":false,"target_va":5373158800,"va":5368838809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4340253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e81d3a4200","size":5,"soft_fail":false,"target_va":5373179248,"va":5368838990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: tweaker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4340173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8cd394200","size":5,"soft_fail":false,"target_va":5373179424,"va":5368839246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8741043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8b3608500","size":5,"soft_fail":false,"target_va":5377580528,"va":5368839480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34138861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8edea0802","size":5,"soft_fail":false,"target_va":5402978880,"va":5368840014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34139337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8c9ec0802","size":5,"soft_fail":false,"target_va":5402979344,"va":5368840002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34140067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8a3ef0802","size":5,"soft_fail":false,"target_va":5402980128,"va":5368840056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::hover
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4319925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8b5ea4100","size":5,"soft_fail":false,"target_va":5373158880,"va":5368838950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4319543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e837e94100","size":5,"soft_fail":false,"target_va":5373158864,"va":5368839316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4321981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8bdf24100","size":5,"soft_fail":false,"target_va":5373160864,"va":5368838878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4322782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8def54100","size":5,"soft_fail":false,"target_va":5373161712,"va":5368838925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::space_between
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4342620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e85c434200","size":5,"soft_fail":false,"target_va":5373181536,"va":5368838911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::horizontal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4342523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8fb424200","size":5,"soft_fail":false,"target_va":5373181424,"va":5368838896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4306455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e817b64100","size":5,"soft_fail":false,"target_va":5373146048,"va":5368839588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4307152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8d0b84100","size":5,"soft_fail":false,"target_va":5373146240,"va":5368839083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4307064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e878b84100","size":5,"soft_fail":false,"target_va":5373146112,"va":5368839043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4306904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8d8b74100","size":5,"soft_fail":false,"target_va":5373145968,"va":5368839059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4307099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e89bb84100","size":5,"soft_fail":false,"target_va":5373146176,"va":5368839072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::tweaker::MaybeHint::confirm
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4343716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368838736,"raw_hex":"e8a4474200","size":5,"soft_fail":false,"target_va":5373182688,"va":5368838967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::tweaker::tweaker_w_font_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":17704,"parent_record_offset":17492,"ancestor_record_offset":17492,"inlinee":5652,"source_line":195,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":18500,"parent_record_offset":18476,"ancestor_record_offset":17492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":18476,"parent_record_offset":18420,"ancestor_record_offset":17492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":18448,"parent_record_offset":18420,"ancestor_record_offset":17492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":18420,"parent_record_offset":18392,"ancestor_record_offset":17492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":18224,"parent_record_offset":18176,"ancestor_record_offset":17492,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":15,"record_offset":18176,"parent_record_offset":18140,"ancestor_record_offset":17492,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":15,"record_offset":19408,"parent_record_offset":19384,"ancestor_record_offset":17492,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":17492,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":15,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":17492,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":15,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":17492,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":15,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":17492,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":18108,"parent_record_offset":18084,"ancestor_record_offset":17492,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":15,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":17492,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":15,"record_offset":19652,"parent_record_offset":18592,"ancestor_record_offset":17492,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":18344,"parent_record_offset":18176,"ancestor_record_offset":17492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":19320,"parent_record_offset":19296,"ancestor_record_offset":17492,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":15,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":17492,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":15,"record_offset":18308,"parent_record_offset":18176,"ancestor_record_offset":17492,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":15,"record_offset":19384,"parent_record_offset":19360,"ancestor_record_offset":17492,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":18672,"parent_record_offset":18648,"ancestor_record_offset":17492,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":18648,"parent_record_offset":18624,"ancestor_record_offset":17492,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":19360,"parent_record_offset":19140,"ancestor_record_offset":17492,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":15,"record_offset":18704,"parent_record_offset":18672,"ancestor_record_offset":17492,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":17492,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19248,"parent_record_offset":19140,"ancestor_record_offset":17492,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19188,"parent_record_offset":19164,"ancestor_record_offset":17492,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19568,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19164,"parent_record_offset":19140,"ancestor_record_offset":17492,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":17492,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":18732,"parent_record_offset":18672,"ancestor_record_offset":17492,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":17492,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":19020,"parent_record_offset":18992,"ancestor_record_offset":17492,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":19048,"parent_record_offset":19020,"ancestor_record_offset":17492,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":19108,"parent_record_offset":18992,"ancestor_record_offset":17492,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":15,"record_offset":18992,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":18824,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":15,"record_offset":19140,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":18852,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":18936,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":15,"record_offset":19476,"parent_record_offset":18780,"ancestor_record_offset":17492,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":15,"record_offset":18624,"parent_record_offset":18592,"ancestor_record_offset":17492,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::settings::impl$2::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/settings.rs
//     evidence={"module":15,"record_offset":18252,"parent_record_offset":18176,"ancestor_record_offset":17492,"inlinee":5666,"source_line":996,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":17792,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::with_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":18592,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5612,"source_line":833,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":19432,"parent_record_offset":19408,"ancestor_record_offset":17492,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":17840,"parent_record_offset":17816,"ancestor_record_offset":17492,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":17868,"parent_record_offset":17816,"ancestor_record_offset":17492,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":15,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":17492,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":17904,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":15,"record_offset":18052,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::decrease
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":18084,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5654,"source_line":139,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::format
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":18140,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5656,"source_line":147,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::impl$5::increase
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":19720,"parent_record_offset":17704,"ancestor_record_offset":17492,"inlinee":5659,"source_line":143,"archive":"_pdb/A/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets/tweaker.rs
//     evidence={"module":15,"record_offset":19624,"parent_record_offset":18592,"ancestor_record_offset":17492,"inlinee":5647,"source_line":252,"archive":"_pdb/A/modules/00015.jsonl.gz"}
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
// 完整结构及原始类型记录: tweaker.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::HintTracker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::MaybeHint
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::TweakableBool
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableBool"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::TweakableEnum
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["tiny_glade::gui::gui_controls::GuiKeyBindingsCategory"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker_w_font_size", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["tiny_glade::gui::settings::DisplayMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["country_core::game_config::ScreenshotFormat"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::TweakableFloatWithStep
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableFloatWithStep"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::tweaker::TweakableList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> >", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["enum2$<tiny_glade::gui::settings::FpsCap>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["tiny_glade::gui::settings::Resolution"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["alloc::string::String"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> >", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["ref$<alloc::string::String>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>", "constructor": "toolbar::mei::widgets::tweaker::TweakableList", "arguments": ["tiny_glade::gui::settings::RefreshRateMillihertz"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >", "constructor": "toolbar::mei::widgets::tweaker::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: tweaker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: tweaker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 120; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)
// 候选参数类型: ["bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::system::commands::Commands", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::HintTracker::confirm
// 信息 ID: e2bc2ee95b9f58b7e935e3c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::MaybeHint::confirm; module=321, inline_record=2292; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=2832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::HintTracker::text
// 信息 ID: 457368cfb2e7efd6fe0fda5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f04d2 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0bfa -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0d03 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0e03 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0edc -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34121b -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x3412d2 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34132a -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341429 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x340f55 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x340ff4 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34107c -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341104 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::apply_decorator_op::apply_decorator_op @ 0xd06585 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::apply_decorator_op::apply_decorator_op @ 0xd063c9 -> 0x4442d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$4; module=2468, inline_record=115980; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$4; module=2468, inline_record=116064; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::MaybeHint::confirm
// 信息 ID: 309b07ea8728e07152b584f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d95f -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1df44 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e534 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb74 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f104 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f690 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fb37 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20110 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x205f4 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20bbc -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x21067 -> 0x4442e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=2808; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$0::format
// 信息 ID: 7804cb1df32582ce74866bd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f80a -> 0x443f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$10::default
// 信息 ID: caae09967ca87ad128e64fab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::settings::do_gui_escape_menu_settings; module=170, inline_record=6532; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$4; module=2468, inline_record=115956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$1::decrease
// 信息 ID: 27885d197329f045a8a79195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f7fe -> 0x444010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$1::increase
// 信息 ID: c8aee99ee71d6ae5348db337
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f9b3 -> 0x444110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$3::decrease
// 信息 ID: 1e3905a2249927cc7bf911f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20d25 -> 0x444210 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=7532; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$3::format
// 信息 ID: 602e53f98ce313789b117e3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20d32 -> 0x444230 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=6904; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$3::increase
// 信息 ID: 0d2cc3a76ace1af46c9a0cb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20ee4 -> 0x444220 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=11520; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$4::decrease
// 信息 ID: cc35f5535a44081828eec53f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=2204; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=8200; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=10552; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >; module=15, inline_record=13252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$4::format
// 信息 ID: 2ff27c2804c5f439606995b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=2292; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=7704; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=10612; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >; module=15, inline_record=13312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$4::increase
// 信息 ID: e4fb17174bfc112e810fb427
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=3872; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=9452; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=12192; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >; module=15, inline_record=14892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$5::decrease
// 信息 ID: 63394e7c5c9d342c7edf2850
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=5016; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=18084; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >; module=15, inline_record=20728; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >; module=15, inline_record=23064; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >; module=15, inline_record=27896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$5::format
// 信息 ID: c9db9125526ef6d5cb89a0ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=5072; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=18140; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >; module=15, inline_record=20784; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >; module=15, inline_record=23120; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >; module=15, inline_record=27952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$5::increase
// 信息 ID: df7aae6a7d66b371d0df86b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=6668; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=19720; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >; module=15, inline_record=22036; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >; module=15, inline_record=24732; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >; module=15, inline_record=29532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$8::default
// 信息 ID: b4439ba08f4bcc96d4245bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0c1a -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0d23 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0e23 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0efc -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341220 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x3412d7 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34132f -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34142e -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x340f5a -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x340ff9 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341081 -> 0x444300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341109 -> 0x444300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::impl$9::from
// 信息 ID: fb3e119d2a24f1c6f84b5763
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
// 信息 ID: cd73f63b5c6399178229b6a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableBool"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0e6f -> 0x20ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0f48 -> 0x20ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341363 -> 0x20ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341462 -> 0x20ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui::closure$4 @ 0x1620c32 -> 0x1b88800 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui::closure$4 @ 0x1620c7a -> 0x1b88800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >
// 信息 ID: 2b7998b8ed187a02c1bb18eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableEnum -> ["country_core::game_config::ScreenshotFormat"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34130b -> 0x1f020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >
// 信息 ID: a657260fdf0e3d87e665beb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableEnum -> ["country_core::resources::ui_settings::ToolbarAlignment"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >
// 信息 ID: 3084547c79de98b5ecf2440c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableEnum -> ["tiny_glade::gui::settings::DisplayMode"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>
// 信息 ID: e5a2e614596889be50cfa86e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableFloatWithStep"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0c96 -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0d93 -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x34127a -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x340fbb -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341043 -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x3410cb -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x341153 -> 0x1f5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >
// 信息 ID: 39bc1adaa92345b11b3511ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableList -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >
// 信息 ID: 03bb23d2efbeb1888cfbecf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["tiny_glade::gui::settings::FpsCap"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableList -> ["enum2$<tiny_glade::gui::settings::FpsCap>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >
// 信息 ID: d20118a964ed4988b3584c01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableList -> ["ref$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> >
// 信息 ID: b38c8d458433af79abd3687e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableList -> ["tiny_glade::gui::settings::RefreshRateMillihertz"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >
// 信息 ID: 228ec311e51a2ffade4ec37b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableList -> ["tiny_glade::gui::settings::Resolution"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker -> ["toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker_w_font_size
// 信息 ID: a63e509d146bbffce2389873
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=4636; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=7316; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=10176; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >; module=15, inline_record=12876; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>; module=15, inline_record=15540; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=17704; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >; module=15, inline_record=20352; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > >; module=15, inline_record=22684; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker_w_font_size::closure$0
// 信息 ID: 458d04fd2ffff869ddf88c97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::tweaker::tweaker_w_font_size", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=3776; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=6572; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=9356; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=12096; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> >; module=15, inline_record=14796; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>; module=15, inline_record=16980; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> >; module=15, inline_record=19624; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> >; module=15, inline_record=21940; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >
// 信息 ID: acb6b7538c4b81c73845dc5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::tweaker::TweakableEnum -> ["tiny_glade::gui::gui_controls::GuiKeyBindingsCategory"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::tweaker::tweaker_w_font_size -> ["toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f054e -> 0x1d860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
