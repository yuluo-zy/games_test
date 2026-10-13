// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/physical_device.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2065935b4d54a13396ffd5646d7786ebe857bcb48d71faed485e65f4e7930dd2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: physical_device.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 222, 256, 548, 553, 570, 778, 1586, 1589, 1593, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"PDB_registry_path_only","version":"1.7.1"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$3::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":3816,"rva":7503152,"va":5376212272}
// 编译实例 RVAs: 0x727d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1367870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212272,"raw_hex":"e83edf1400","size":5,"soft_fail":false,"target_va":5377580528,"va":5376212653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":548,"record_offset":3944,"parent_record_offset":3816,"ancestor_record_offset":3816,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$13::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":87676,"rva":7622944,"va":5376332064}
// 编译实例 RVAs: 0x745120, 0x1176ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1253774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332064,"raw_hex":"e88e211300","size":5,"soft_fail":false,"target_va":5377585888,"va":5376332109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$16::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7995
// 首条位置: {"module":570,"record_offset":87796,"rva":7623008,"va":5376332128}
// 编译实例 RVAs: 0x745160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1251967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332128,"raw_hex":"e87f1a1300","size":5,"soft_fail":false,"target_va":5377584208,"va":5376332236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35E5
// 首条位置: {"module":256,"inlinee":13797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":35212,"parent_record_offset":35164,"ancestor_record_offset":35164,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":35288,"parent_record_offset":35164,"ancestor_record_offset":35164,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":35440,"parent_record_offset":35412,"ancestor_record_offset":35164,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":256,"record_offset":35412,"parent_record_offset":35384,"ancestor_record_offset":35164,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":256,"record_offset":35384,"parent_record_offset":35360,"ancestor_record_offset":35164,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":35312,"parent_record_offset":35288,"ancestor_record_offset":35164,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$18::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":8212,"rva":3432832,"va":5372141952}
// 编译实例 RVAs: 0x346180, 0x1918030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field5_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5442906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372141952,"raw_hex":"e85a0d5300","size":5,"soft_fail":false,"target_va":5377585120,"va":5372142209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":1800,"rva":7502272,"va":5376211392}
// 编译实例 RVAs: 0x7279c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6579546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8a69a9bff","size":5,"soft_fail":false,"target_va":5369632448,"va":5376211989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3462228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8ac2bcbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376211935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8f7020000","size":5,"soft_fail":false,"target_va":5376212272,"va":5376211508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1372106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8caef1400","size":5,"soft_fail":false,"target_va":5377583920,"va":5376211809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27381665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8a1cfa101","size":5,"soft_fail":false,"target_va":5403593344,"va":5376211674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1318260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8741d1400","size":5,"soft_fail":false,"target_va":5377529888,"va":5376211623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1368614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e826e21400","size":5,"soft_fail":false,"target_va":5377580528,"va":5376211909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26768089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376211392,"raw_hex":"e8d9729801","size":5,"soft_fail":false,"target_va":5402980128,"va":5376212034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":548,"record_offset":2328,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":548,"record_offset":1948,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":548,"record_offset":2824,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":2300,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":2852,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":548,"record_offset":2412,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":12644,"source_line":1213,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":2224,"parent_record_offset":2168,"ancestor_record_offset":1800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":548,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":1800,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":2196,"parent_record_offset":2168,"ancestor_record_offset":1800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":548,"record_offset":2000,"parent_record_offset":1948,"ancestor_record_offset":1800,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$19::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":548,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1800,"inlinee":19421,"source_line":2305,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":548,"record_offset":2028,"parent_record_offset":1948,"ancestor_record_offset":1800,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1800,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1800,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":2092,"parent_record_offset":1948,"ancestor_record_offset":1800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":2056,"parent_record_offset":1948,"ancestor_record_offset":1800,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":548,"record_offset":2664,"parent_record_offset":2520,"ancestor_record_offset":1800,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":548,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1800,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":548,"record_offset":2492,"parent_record_offset":2464,"ancestor_record_offset":1800,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":2520,"parent_record_offset":2492,"ancestor_record_offset":1800,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":1800,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":1800,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":1800,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":548,"record_offset":2464,"parent_record_offset":2436,"ancestor_record_offset":1800,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$21::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":222,"record_offset":8332,"rva":3433104,"va":5372142224}
// 编译实例 RVAs: 0x346290, 0x1918140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5441805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142224,"raw_hex":"e80d095300","size":5,"soft_fail":false,"target_va":5377584208,"va":5372142398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5443350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372142224,"raw_hex":"e8160f5300","size":5,"soft_fail":false,"target_va":5377585888,"va":5372142533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":3672,"rva":7503120,"va":5376212240}
// 编译实例 RVAs: 0x727d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::name
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":4064,"rva":7503664,"va":5376212784}
// 编译实例 RVAs: 0x727f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3463183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e8f127cbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376212874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3463238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e8ba27cbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376212945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27380452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e8e4caa101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376212903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27381572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e844cfa101","size":5,"soft_fail":false,"target_va":5403594400,"va":5376212823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26765152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e860679801","size":5,"soft_fail":false,"target_va":5402978144,"va":5376212987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::_$LT$impl$u20$core..ffi..c_str..CStr$GT$::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1317012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e894181400","size":5,"soft_fail":false,"target_va":5377529856,"va":5376212839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1314147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376212784,"raw_hex":"e8630d1400","size":5,"soft_fail":false,"target_va":5377527120,"va":5376212968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":548,"record_offset":4296,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":548,"record_offset":4208,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":4808,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":4064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":4572,"parent_record_offset":4536,"ancestor_record_offset":4064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":4064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":4064,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":4064,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":4688,"parent_record_offset":4508,"ancestor_record_offset":4064,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":548,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":548,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":4064,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":548,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":4064,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":548,"record_offset":4364,"parent_record_offset":4324,"ancestor_record_offset":4064,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":548,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":4064,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":548,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":4064,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":548,"record_offset":4480,"parent_record_offset":4448,"ancestor_record_offset":4064,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":548,"record_offset":4236,"parent_record_offset":4208,"ancestor_record_offset":4064,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":548,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":4064,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":548,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4064,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":4728,"parent_record_offset":4448,"ancestor_record_offset":4064,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76D1
// 首条位置: {"module":548,"record_offset":5204,"rva":7503952,"va":5376213072}
// 编译实例 RVAs: 0x728050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26766982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213072,"raw_hex":"e8866e9801","size":5,"soft_fail":false,"target_va":5402980304,"va":5376213317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":5368,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":5532,"parent_record_offset":5204,"ancestor_record_offset":5204,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":5204,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":548,"record_offset":5396,"parent_record_offset":5368,"ancestor_record_offset":5204,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":5588,"parent_record_offset":5560,"ancestor_record_offset":5204,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":5708,"parent_record_offset":5616,"ancestor_record_offset":5204,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":5560,"parent_record_offset":5532,"ancestor_record_offset":5204,"inlinee":30390,"source_line":49,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":5800,"parent_record_offset":5708,"ancestor_record_offset":5204,"inlinee":30396,"source_line":53,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":5680,"parent_record_offset":5616,"ancestor_record_offset":5204,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":5424,"parent_record_offset":5396,"ancestor_record_offset":5204,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":5480,"parent_record_offset":5452,"ancestor_record_offset":5204,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":5452,"parent_record_offset":5424,"ancestor_record_offset":5204,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":548,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":5204,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":548,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":5204,"inlinee":30392,"source_line":134,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B8
// 首条位置: {"module":548,"inlinee":30392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":5860,"rva":7504208,"va":5376213328}
// 编译实例 RVAs: 0x728150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010cba0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6403897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213328,"raw_hex":"e8c7489eff","size":5,"soft_fail":false,"target_va":5369809824,"va":5376213716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfc50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4556168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213328,"raw_hex":"e8787abaff","size":5,"soft_fail":false,"target_va":5371657296,"va":5376213459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10802309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213328,"raw_hex":"e885d4a400","size":5,"soft_fail":false,"target_va":5387015680,"va":5376213366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26766620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213328,"raw_hex":"e81c6d9801","size":5,"soft_fail":false,"target_va":5402980304,"va":5376213679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":548,"record_offset":6036,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":13864,"source_line":453,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":6092,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":6256,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":6744,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":5860,"inlinee":9129,"source_line":1510,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":7172,"parent_record_offset":7140,"ancestor_record_offset":5860,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":5860,"inlinee":9128,"source_line":1485,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":7140,"parent_record_offset":7120,"ancestor_record_offset":5860,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":548,"record_offset":6984,"parent_record_offset":6784,"ancestor_record_offset":5860,"inlinee":9124,"source_line":65,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":548,"record_offset":6900,"parent_record_offset":6784,"ancestor_record_offset":5860,"inlinee":9127,"source_line":96,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":6784,"parent_record_offset":6764,"ancestor_record_offset":5860,"inlinee":9121,"source_line":104,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":5860,"inlinee":9133,"source_line":186,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":5860,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":548,"record_offset":6120,"parent_record_offset":6092,"ancestor_record_offset":5860,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":548,"record_offset":6492,"parent_record_offset":6452,"ancestor_record_offset":5860,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":5860,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":6592,"parent_record_offset":6368,"ancestor_record_offset":5860,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":5860,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":5860,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":6524,"parent_record_offset":6452,"ancestor_record_offset":5860,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":6452,"parent_record_offset":6416,"ancestor_record_offset":5860,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":6416,"parent_record_offset":6368,"ancestor_record_offset":5860,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":5860,"inlinee":30390,"source_line":49,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":6688,"parent_record_offset":6592,"ancestor_record_offset":5860,"inlinee":30396,"source_line":53,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":548,"record_offset":7096,"parent_record_offset":6784,"ancestor_record_offset":5860,"inlinee":9125,"source_line":140,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":548,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":5860,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":548,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":5860,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":6816,"parent_record_offset":6784,"ancestor_record_offset":5860,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":6764,"parent_record_offset":6744,"ancestor_record_offset":5860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":7204,"parent_record_offset":7172,"ancestor_record_offset":5860,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":5860,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":7064,"parent_record_offset":6984,"ancestor_record_offset":5860,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":6148,"parent_record_offset":6120,"ancestor_record_offset":5860,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":6204,"parent_record_offset":6176,"ancestor_record_offset":5860,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":6176,"parent_record_offset":6148,"ancestor_record_offset":5860,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":5860,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":5860,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":7232,"parent_record_offset":7172,"ancestor_record_offset":5860,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":7264,"parent_record_offset":7140,"ancestor_record_offset":5860,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":548,"record_offset":6624,"parent_record_offset":6592,"ancestor_record_offset":5860,"inlinee":30398,"source_line":152,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76BE
// 首条位置: {"module":548,"inlinee":30398}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":6624,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76D3
// 首条位置: {"module":548,"record_offset":7320,"rva":7504624,"va":5376213744}
// 编译实例 RVAs: 0x7282f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26766310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376213744,"raw_hex":"e8e66b9801","size":5,"soft_fail":false,"target_va":5402980304,"va":5376213989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":7476,"parent_record_offset":7320,"ancestor_record_offset":7320,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":7640,"parent_record_offset":7320,"ancestor_record_offset":7320,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":7320,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":548,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":7320,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":7696,"parent_record_offset":7668,"ancestor_record_offset":7320,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":7816,"parent_record_offset":7724,"ancestor_record_offset":7320,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":7668,"parent_record_offset":7640,"ancestor_record_offset":7320,"inlinee":30390,"source_line":49,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":7908,"parent_record_offset":7816,"ancestor_record_offset":7320,"inlinee":30396,"source_line":53,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":7788,"parent_record_offset":7724,"ancestor_record_offset":7320,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":7320,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":7320,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":7320,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":548,"record_offset":7724,"parent_record_offset":7696,"ancestor_record_offset":7320,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":548,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":7320,"inlinee":30400,"source_line":166,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76C0
// 首条位置: {"module":548,"inlinee":30400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":7968,"rva":7504880,"va":5376214000}
// 编译实例 RVAs: 0x7283f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010cba0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6404569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214000,"raw_hex":"e827469eff","size":5,"soft_fail":false,"target_va":5369809824,"va":5376214388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfc50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4556840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214000,"raw_hex":"e8d877baff","size":5,"soft_fail":false,"target_va":5371657296,"va":5376214131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10801637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214000,"raw_hex":"e8e5d1a400","size":5,"soft_fail":false,"target_va":5387015680,"va":5376214038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26765948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214000,"raw_hex":"e87c6a9801","size":5,"soft_fail":false,"target_va":5402980304,"va":5376214351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":548,"record_offset":8136,"parent_record_offset":7968,"ancestor_record_offset":7968,"inlinee":13864,"source_line":453,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":8192,"parent_record_offset":7968,"ancestor_record_offset":7968,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":8356,"parent_record_offset":7968,"ancestor_record_offset":7968,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":8844,"parent_record_offset":7968,"ancestor_record_offset":7968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":7968,"inlinee":9129,"source_line":1510,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":7968,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":7968,"inlinee":9128,"source_line":1485,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":9240,"parent_record_offset":9220,"ancestor_record_offset":7968,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":548,"record_offset":9084,"parent_record_offset":8884,"ancestor_record_offset":7968,"inlinee":9124,"source_line":65,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":548,"record_offset":9000,"parent_record_offset":8884,"ancestor_record_offset":7968,"inlinee":9127,"source_line":96,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":8884,"parent_record_offset":8864,"ancestor_record_offset":7968,"inlinee":9121,"source_line":104,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":7968,"inlinee":9133,"source_line":186,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":7968,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":548,"record_offset":8220,"parent_record_offset":8192,"ancestor_record_offset":7968,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":548,"record_offset":8592,"parent_record_offset":8552,"ancestor_record_offset":7968,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":8412,"parent_record_offset":8384,"ancestor_record_offset":7968,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":548,"record_offset":8692,"parent_record_offset":8468,"ancestor_record_offset":7968,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":8440,"parent_record_offset":8412,"ancestor_record_offset":7968,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":8468,"parent_record_offset":8440,"ancestor_record_offset":7968,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":8624,"parent_record_offset":8552,"ancestor_record_offset":7968,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":8552,"parent_record_offset":8516,"ancestor_record_offset":7968,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":548,"record_offset":8516,"parent_record_offset":8468,"ancestor_record_offset":7968,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":8384,"parent_record_offset":8356,"ancestor_record_offset":7968,"inlinee":30390,"source_line":49,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$40::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":548,"record_offset":8788,"parent_record_offset":8692,"ancestor_record_offset":7968,"inlinee":30396,"source_line":53,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":548,"record_offset":9196,"parent_record_offset":8884,"ancestor_record_offset":7968,"inlinee":9125,"source_line":140,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":548,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":7968,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":548,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":7968,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":8916,"parent_record_offset":8884,"ancestor_record_offset":7968,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":8864,"parent_record_offset":8844,"ancestor_record_offset":7968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":7968,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":7968,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":9164,"parent_record_offset":9084,"ancestor_record_offset":7968,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":7968,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":8304,"parent_record_offset":8276,"ancestor_record_offset":7968,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":8276,"parent_record_offset":8248,"ancestor_record_offset":7968,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":7968,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":7968,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":9332,"parent_record_offset":9272,"ancestor_record_offset":7968,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":9364,"parent_record_offset":9240,"ancestor_record_offset":7968,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":548,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":7968,"inlinee":30402,"source_line":184,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76C2
// 首条位置: {"module":548,"inlinee":30402}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::MemoryHeapFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":548,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":8724,"inlinee":30394,"source_line":32,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
// 记录形式: compiled_procedure; instances=1
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":9420,"rva":7505296,"va":5376214416}
// 编译实例 RVAs: 0x728590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfc50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4557471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e86175baff","size":5,"soft_fail":false,"target_va":5371657296,"va":5376214762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3465404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e8441fcbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376215095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27378282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e86ac2a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376215073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27378931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e8f3c4a101","size":5,"soft_fail":false,"target_va":5403593392,"va":5376214456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26762802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e8325e9801","size":5,"soft_fail":false,"target_va":5402978144,"va":5376215337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10801074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e8b2cfa400","size":5,"soft_fail":false,"target_va":5387015680,"va":5376214601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26763825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e831629801","size":5,"soft_fail":false,"target_va":5402979152,"va":5376215322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1228391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376214416,"raw_hex":"e867be1200","size":5,"soft_fail":false,"target_va":5377442944,"va":5376214548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*216(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376214416,"raw_hex":"ff90d8000000","size":6,"soft_fail":false,"va":5376214572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":11176,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":548,"record_offset":9872,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":13864,"source_line":453,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":548,"record_offset":11392,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":13888,"source_line":461,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_memory_properties2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":548,"record_offset":9844,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":30408,"source_line":238,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties2::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":9648,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":30406,"source_line":11393,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2404::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":548,"record_offset":9620,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":30404,"source_line":27480,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":548,"record_offset":11080,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":10692,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":10608,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":548,"record_offset":11052,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":9253,"source_line":1547,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":9928,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":548,"record_offset":10524,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":9420,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":9420,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":11232,"parent_record_offset":11204,"ancestor_record_offset":9420,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":548,"record_offset":11264,"parent_record_offset":11232,"ancestor_record_offset":9420,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":548,"record_offset":11204,"parent_record_offset":11176,"ancestor_record_offset":9420,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":10144,"parent_record_offset":10120,"ancestor_record_offset":9420,"inlinee":9129,"source_line":1510,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":9420,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":11608,"parent_record_offset":11584,"ancestor_record_offset":9420,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":10380,"parent_record_offset":10344,"ancestor_record_offset":9420,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":9420,"inlinee":9128,"source_line":1485,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":548,"record_offset":10344,"parent_record_offset":10320,"ancestor_record_offset":9420,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":548,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":9420,"inlinee":13889,"source_line":466,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":548,"record_offset":10180,"parent_record_offset":9976,"ancestor_record_offset":9420,"inlinee":9124,"source_line":65,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":548,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":9420,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":548,"record_offset":10096,"parent_record_offset":9976,"ancestor_record_offset":9420,"inlinee":9127,"source_line":96,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":548,"record_offset":11560,"parent_record_offset":11416,"ancestor_record_offset":9420,"inlinee":9140,"source_line":121,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":9420,"inlinee":9121,"source_line":104,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":9420,"inlinee":9133,"source_line":186,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":548,"record_offset":11504,"parent_record_offset":11416,"ancestor_record_offset":9420,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":548,"record_offset":9792,"parent_record_offset":9772,"ancestor_record_offset":9420,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":548,"record_offset":10752,"parent_record_offset":10720,"ancestor_record_offset":9420,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":548,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":9420,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":548,"record_offset":11116,"parent_record_offset":11080,"ancestor_record_offset":9420,"inlinee":6744,"source_line":1555,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":548,"record_offset":11144,"parent_record_offset":11080,"ancestor_record_offset":9420,"inlinee":6123,"source_line":1553,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":548,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":9420,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":548,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":9420,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":548,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":9420,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":9420,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":10656,"parent_record_offset":10632,"ancestor_record_offset":9420,"inlinee":4243,"source_line":154,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":548,"record_offset":10720,"parent_record_offset":10692,"ancestor_record_offset":9420,"inlinee":8595,"source_line":165,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":9676,"parent_record_offset":9648,"ancestor_record_offset":9420,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9420,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":548,"record_offset":11732,"parent_record_offset":11392,"ancestor_record_offset":9420,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":548,"record_offset":10296,"parent_record_offset":9976,"ancestor_record_offset":9420,"inlinee":9125,"source_line":140,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":548,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":9420,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":548,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9420,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":9420,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":548,"record_offset":10012,"parent_record_offset":9976,"ancestor_record_offset":9420,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":9420,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":9420,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":9420,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":548,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":9420,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":548,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9420,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":9420,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":10412,"parent_record_offset":10380,"ancestor_record_offset":9420,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":548,"record_offset":11012,"parent_record_offset":10920,"ancestor_record_offset":9420,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":9420,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":11340,"parent_record_offset":11312,"ancestor_record_offset":9420,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":548,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":9420,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":10264,"parent_record_offset":10180,"ancestor_record_offset":9420,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":548,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":9420,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":548,"record_offset":10956,"parent_record_offset":10920,"ancestor_record_offset":9420,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":548,"record_offset":10920,"parent_record_offset":10720,"ancestor_record_offset":9420,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":11448,"parent_record_offset":11416,"ancestor_record_offset":9420,"inlinee":12880,"source_line":1493,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":9420,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":11816,"parent_record_offset":11780,"ancestor_record_offset":9420,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":9420,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":10440,"parent_record_offset":10380,"ancestor_record_offset":9420,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":9420,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":548,"record_offset":10472,"parent_record_offset":10344,"ancestor_record_offset":9420,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$5::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76D7
// 首条位置: {"module":1586,"inlinee":30423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1586,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":6912,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/01586.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1589,"record_offset":696,"rva":18312624,"va":5387021744}
// 编译实例 RVAs: 0x1176db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14272151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387021744,"raw_hex":"e8693926ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387021842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14272273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387021744,"raw_hex":"e8ef3826ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387021980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387021744,"raw_hex":"e83779f300","size":5,"soft_fail":false,"target_va":5402978144,"va":5387021860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*72(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387021744,"raw_hex":"ff5348","size":3,"soft_fail":false,"va":5387021794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1589,"record_offset":992,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1589,"record_offset":1432,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1589,"record_offset":1344,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1589,"record_offset":960,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":70015,"source_line":262,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1589,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":696,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1589,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":696,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1589,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":696,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1589,"record_offset":1188,"parent_record_offset":1088,"ancestor_record_offset":696,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1589,"record_offset":1572,"parent_record_offset":1480,"ancestor_record_offset":696,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1589,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":696,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1589,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":696,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1589,"record_offset":1056,"parent_record_offset":1024,"ancestor_record_offset":696,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1589,"record_offset":1304,"parent_record_offset":1056,"ancestor_record_offset":696,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1589,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":696,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1589,"record_offset":1024,"parent_record_offset":992,"ancestor_record_offset":696,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1589,"record_offset":1132,"parent_record_offset":1088,"ancestor_record_offset":696,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1589,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":696,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1589,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":696,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1589,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1589,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":696,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":11920,"rva":7506224,"va":5376215344}
// 编译实例 RVAs: 0x728930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141176db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10806367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215344,"raw_hex":"e85fe4a400","size":5,"soft_fail":false,"target_va":5387021744,"va":5376215372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::adapters::try_process
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10806670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215344,"raw_hex":"e88ee5a400","size":5,"soft_fail":false,"target_va":5387022128,"va":5376215453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":548,"record_offset":12136,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":548,"record_offset":12192,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":12076,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":12108,"parent_record_offset":11920,"ancestor_record_offset":11920,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":548,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":11920,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":548,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":11920,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":548,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":11920,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1117F
// 首条位置: {"module":1589,"inlinee":70015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3007,"record_offset":23148,"rva":26306752,"va":5395015872}
// 编译实例 RVAs: 0x19168c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25404352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8405c7cfe","size":5,"soft_fail":false,"target_va":5369614224,"va":5395018571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140100190
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25262533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e83b867efe","size":5,"soft_fail":false,"target_va":5369758096,"va":5395020624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23368040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8986e9bfe","size":5,"soft_fail":false,"target_va":5371652896,"va":5395020931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037e560
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22649108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8ec66a6fe","size":5,"soft_fail":false,"target_va":5372372320,"va":5395021423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22393223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8794eaafe","size":5,"soft_fail":false,"target_va":5372625024,"va":5395018242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22267071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8413bacfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5395016762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22268169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8f736acfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395017876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7978101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e88b4386ff","size":5,"soft_fail":false,"target_va":5387039456,"va":5395017552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141916540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8c6f2ffff","size":5,"soft_fail":false,"target_va":5395014976,"va":5395018357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7884218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8ba4d7800","size":5,"soft_fail":false,"target_va":5402900112,"va":5395015889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a1750
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7906547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8f3a47800","size":5,"soft_fail":false,"target_va":5402924880,"va":5395018328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8574549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e855d68200","size":5,"soft_fail":false,"target_va":5403593344,"va":5395018790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8577327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e82fe18200","size":5,"soft_fail":false,"target_va":5403593360,"va":5395016028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8577288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e808e18200","size":5,"soft_fail":false,"target_va":5403593392,"va":5395016099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7960712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e888787900","size":5,"soft_fail":false,"target_va":5402978144,"va":5395017427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17492825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8a714f5fe","size":5,"soft_fail":false,"target_va":5377527120,"va":5395019940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7951402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8d6ab86ff","size":5,"soft_fail":false,"target_va":5387066208,"va":5395017605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::from_bytes_until_nul
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17457655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8099ef5fe","size":5,"soft_fail":false,"target_va":5377559008,"va":5395016658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::to_str
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17457151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e801a0f5fe","size":5,"soft_fail":false,"target_va":5377559552,"va":5395016698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7960254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8be767900","size":5,"soft_fail":false,"target_va":5402980128,"va":5395019869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7960373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e835777900","size":5,"soft_fail":false,"target_va":5402980304,"va":5395019926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17664048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8d077f2fe","size":5,"soft_fail":false,"target_va":5377353776,"va":5395017819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17651994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395015872,"raw_hex":"e8e6a6f2fe","size":5,"soft_fail":false,"target_va":5377366224,"va":5395018213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*104(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5395015872,"raw_hex":"ff5068","size":3,"soft_fail":false,"va":5395015993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":29844,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":25980,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":3007,"record_offset":29932,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":11660,"source_line":691,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::enumerate_device_extension_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":26120,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98710,"source_line":504,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_memory_properties2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":29532,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30408,"source_line":238,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":23416,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":31877,"source_line":456,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_properties2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":24032,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98708,"source_line":161,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_queue_family_properties
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":25280,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98709,"source_line":467,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties2::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":29328,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30406,"source_line":11393,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceMemoryProperties::memory_heaps_as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":29588,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30388,"source_line":1555,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceProperties2::push_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23620,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":70253,"source_line":11108,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2404::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":28096,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30404,"source_line":27480,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$652::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23528,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98703,"source_line":11084,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$677::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":28124,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98713,"source_line":11369,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$700::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23500,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98701,"source_line":11601,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$747::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23472,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":98699,"source_line":12023,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3007,"record_offset":27156,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3007,"record_offset":27184,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3007,"record_offset":29752,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":26036,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":26416,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":26148,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":28152,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":26388,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3007,"record_offset":27212,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3007,"record_offset":27072,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::parse_driver_version
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":25204,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30427,"source_line":448,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::physical_device::string_from_c_array
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":24060,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":30426,"source_line":393,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":3007,"record_offset":28188,"parent_record_offset":23148,"ancestor_record_offset":23148,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24412,"parent_record_offset":24388,"ancestor_record_offset":23148,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24436,"parent_record_offset":24412,"ancestor_record_offset":23148,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":25928,"parent_record_offset":25904,"ancestor_record_offset":23148,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":30028,"parent_record_offset":29996,"ancestor_record_offset":23148,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24388,"parent_record_offset":24352,"ancestor_record_offset":23148,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":25904,"parent_record_offset":25812,"ancestor_record_offset":23148,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3007,"record_offset":29996,"parent_record_offset":29964,"ancestor_record_offset":23148,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":25840,"parent_record_offset":25812,"ancestor_record_offset":23148,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24352,"parent_record_offset":24324,"ancestor_record_offset":23148,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":23148,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24508,"parent_record_offset":24324,"ancestor_record_offset":23148,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":25812,"parent_record_offset":25788,"ancestor_record_offset":23148,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24260,"parent_record_offset":24232,"ancestor_record_offset":23148,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24232,"parent_record_offset":24204,"ancestor_record_offset":23148,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":23148,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24204,"parent_record_offset":24176,"ancestor_record_offset":23148,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24148,"parent_record_offset":24120,"ancestor_record_offset":23148,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3007,"record_offset":24120,"parent_record_offset":24060,"ancestor_record_offset":23148,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":29964,"parent_record_offset":29932,"ancestor_record_offset":23148,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":30996,"parent_record_offset":30972,"ancestor_record_offset":23148,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":25392,"parent_record_offset":25312,"ancestor_record_offset":23148,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":24296,"parent_record_offset":24260,"ancestor_record_offset":23148,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":26060,"parent_record_offset":26036,"ancestor_record_offset":23148,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::instance::impl$3::get_physical_device_queue_family_properties::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":3007,"record_offset":25364,"parent_record_offset":25312,"ancestor_record_offset":23148,"inlinee":70131,"source_line":471,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::prelude::read_into_uninitialized_vector
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":25312,"parent_record_offset":25280,"ancestor_record_offset":23148,"inlinee":70138,"source_line":45,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$15::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23556,"parent_record_offset":23528,"ancestor_record_offset":23148,"inlinee":98705,"source_line":889,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$345::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":23584,"parent_record_offset":23556,"ancestor_record_offset":23148,"inlinee":98707,"source_line":6825,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":3007,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":23148,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3007,"record_offset":25520,"parent_record_offset":25476,"ancestor_record_offset":23148,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3007,"record_offset":25544,"parent_record_offset":25520,"ancestor_record_offset":23148,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":3007,"record_offset":29616,"parent_record_offset":29588,"ancestor_record_offset":23148,"inlinee":8000,"source_line":347,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3007,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":23148,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3007,"record_offset":27620,"parent_record_offset":27596,"ancestor_record_offset":23148,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3007,"record_offset":27596,"parent_record_offset":27540,"ancestor_record_offset":23148,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3007,"record_offset":26296,"parent_record_offset":26184,"ancestor_record_offset":23148,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3007,"record_offset":29124,"parent_record_offset":29096,"ancestor_record_offset":23148,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3007,"record_offset":28484,"parent_record_offset":28460,"ancestor_record_offset":23148,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3007,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":23148,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3007,"record_offset":26324,"parent_record_offset":26184,"ancestor_record_offset":23148,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3007,"record_offset":24572,"parent_record_offset":24548,"ancestor_record_offset":23148,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":3007,"record_offset":23676,"parent_record_offset":23652,"ancestor_record_offset":23148,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3007,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":23148,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3007,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23148,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3007,"record_offset":29788,"parent_record_offset":29752,"ancestor_record_offset":23148,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3007,"record_offset":26084,"parent_record_offset":26060,"ancestor_record_offset":23148,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":27904,"parent_record_offset":27812,"ancestor_record_offset":23148,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":23652,"parent_record_offset":23620,"ancestor_record_offset":23148,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23148,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3007,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":23148,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3007,"record_offset":27644,"parent_record_offset":27620,"ancestor_record_offset":23148,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3007,"record_offset":25868,"parent_record_offset":25840,"ancestor_record_offset":23148,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3007,"record_offset":28684,"parent_record_offset":28660,"ancestor_record_offset":23148,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3007,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":23148,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3007,"record_offset":24548,"parent_record_offset":24260,"ancestor_record_offset":23148,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":25764,"parent_record_offset":25312,"ancestor_record_offset":23148,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":26004,"parent_record_offset":25980,"ancestor_record_offset":23148,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":29028,"parent_record_offset":29004,"ancestor_record_offset":23148,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":28920,"parent_record_offset":28896,"ancestor_record_offset":23148,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":24464,"parent_record_offset":24436,"ancestor_record_offset":23148,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":27668,"parent_record_offset":27644,"ancestor_record_offset":23148,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":24092,"parent_record_offset":24060,"ancestor_record_offset":23148,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3007,"record_offset":29208,"parent_record_offset":29180,"ancestor_record_offset":23148,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3007,"record_offset":29236,"parent_record_offset":29208,"ancestor_record_offset":23148,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":29644,"parent_record_offset":29616,"ancestor_record_offset":23148,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":29700,"parent_record_offset":29672,"ancestor_record_offset":23148,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3007,"record_offset":29672,"parent_record_offset":29644,"ancestor_record_offset":23148,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3007,"record_offset":29152,"parent_record_offset":29124,"ancestor_record_offset":23148,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":23148,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":31032,"parent_record_offset":30996,"ancestor_record_offset":23148,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":23148,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":23148,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":23148,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":31056,"parent_record_offset":31032,"ancestor_record_offset":23148,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":31088,"parent_record_offset":30996,"ancestor_record_offset":23148,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3007,"record_offset":29096,"parent_record_offset":29068,"ancestor_record_offset":23148,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":28244,"parent_record_offset":28216,"ancestor_record_offset":23148,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":28272,"parent_record_offset":28244,"ancestor_record_offset":23148,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":27844,"parent_record_offset":27812,"ancestor_record_offset":23148,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":29068,"parent_record_offset":28848,"ancestor_record_offset":23148,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":27812,"parent_record_offset":27784,"ancestor_record_offset":23148,"inlinee":18288,"source_line":6499,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":28304,"parent_record_offset":28272,"ancestor_record_offset":23148,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28980,"parent_record_offset":28956,"ancestor_record_offset":23148,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28956,"parent_record_offset":28848,"ancestor_record_offset":23148,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28896,"parent_record_offset":28872,"ancestor_record_offset":23148,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28816,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28872,"parent_record_offset":28848,"ancestor_record_offset":23148,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28356,"parent_record_offset":28332,"ancestor_record_offset":23148,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28332,"parent_record_offset":28272,"ancestor_record_offset":23148,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":27868,"parent_record_offset":27844,"ancestor_record_offset":23148,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28380,"parent_record_offset":28356,"ancestor_record_offset":23148,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3007,"record_offset":28628,"parent_record_offset":28600,"ancestor_record_offset":23148,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3007,"record_offset":28660,"parent_record_offset":28628,"ancestor_record_offset":23148,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3007,"record_offset":28600,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28432,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":23148,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":28848,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3007,"record_offset":28460,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3007,"record_offset":28544,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3007,"record_offset":28724,"parent_record_offset":28380,"ancestor_record_offset":23148,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":3007,"record_offset":28216,"parent_record_offset":28188,"ancestor_record_offset":23148,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$11::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":3007,"record_offset":27784,"parent_record_offset":27756,"ancestor_record_offset":23148,"inlinee":18286,"source_line":1354,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3007,"record_offset":26352,"parent_record_offset":26184,"ancestor_record_offset":23148,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3007,"record_offset":26212,"parent_record_offset":26184,"ancestor_record_offset":23148,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":3007,"record_offset":26184,"parent_record_offset":26148,"ancestor_record_offset":23148,"inlinee":98711,"source_line":325,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":3007,"record_offset":27728,"parent_record_offset":27276,"ancestor_record_offset":23148,"inlinee":18282,"source_line":373,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":3007,"record_offset":27276,"parent_record_offset":27240,"ancestor_record_offset":23148,"inlinee":18281,"source_line":1027,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":3007,"record_offset":27756,"parent_record_offset":27276,"ancestor_record_offset":23148,"inlinee":18284,"source_line":1072,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3007,"record_offset":27344,"parent_record_offset":27320,"ancestor_record_offset":23148,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3007,"record_offset":27420,"parent_record_offset":27392,"ancestor_record_offset":23148,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3007,"record_offset":27540,"parent_record_offset":27392,"ancestor_record_offset":23148,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3007,"record_offset":27320,"parent_record_offset":27276,"ancestor_record_offset":23148,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3007,"record_offset":27444,"parent_record_offset":27420,"ancestor_record_offset":23148,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3007,"record_offset":27472,"parent_record_offset":27444,"ancestor_record_offset":23148,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3007,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":23148,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3007,"record_offset":27368,"parent_record_offset":27344,"ancestor_record_offset":23148,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18197
// 首条位置: {"module":3007,"inlinee":98711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3007,"record_offset":26296,"parent_record_offset":26184,"ancestor_record_offset":26184,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3007,"record_offset":26324,"parent_record_offset":26184,"ancestor_record_offset":26184,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3007,"record_offset":26352,"parent_record_offset":26184,"ancestor_record_offset":26184,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3007,"record_offset":26212,"parent_record_offset":26184,"ancestor_record_offset":26184,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":26184,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":26184,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18189
// 首条位置: {"module":3007,"inlinee":98697}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3007,"record_offset":19824,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":3007,"record_offset":19736,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":20332,"parent_record_offset":19696,"ancestor_record_offset":19696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":20116,"parent_record_offset":20092,"ancestor_record_offset":19696,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":20140,"parent_record_offset":20116,"ancestor_record_offset":19696,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":20476,"parent_record_offset":20452,"ancestor_record_offset":19696,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":20092,"parent_record_offset":20056,"ancestor_record_offset":19696,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":20452,"parent_record_offset":20428,"ancestor_record_offset":19696,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":20056,"parent_record_offset":20028,"ancestor_record_offset":19696,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":19696,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":20212,"parent_record_offset":20028,"ancestor_record_offset":19696,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":19696,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":19696,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":19936,"parent_record_offset":19908,"ancestor_record_offset":19696,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":19880,"parent_record_offset":19852,"ancestor_record_offset":19696,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":19908,"parent_record_offset":19880,"ancestor_record_offset":19696,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":19852,"parent_record_offset":19824,"ancestor_record_offset":19696,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":20000,"parent_record_offset":19964,"ancestor_record_offset":19696,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":3007,"record_offset":19764,"parent_record_offset":19736,"ancestor_record_offset":19696,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":3007,"record_offset":19788,"parent_record_offset":19764,"ancestor_record_offset":19696,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3007,"record_offset":20276,"parent_record_offset":20252,"ancestor_record_offset":19696,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3007,"record_offset":20252,"parent_record_offset":19964,"ancestor_record_offset":19696,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":19696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":20168,"parent_record_offset":20140,"ancestor_record_offset":19696,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::string_from_c_array
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 393
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76DA
// 首条位置: {"module":3007,"inlinee":30426}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3007,"record_offset":24120,"parent_record_offset":24060,"ancestor_record_offset":24060,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":24092,"parent_record_offset":24060,"ancestor_record_offset":24060,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24412,"parent_record_offset":24388,"ancestor_record_offset":24060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24436,"parent_record_offset":24412,"ancestor_record_offset":24060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":24388,"parent_record_offset":24352,"ancestor_record_offset":24060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24352,"parent_record_offset":24324,"ancestor_record_offset":24060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":24060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":24508,"parent_record_offset":24324,"ancestor_record_offset":24060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24260,"parent_record_offset":24232,"ancestor_record_offset":24060,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24232,"parent_record_offset":24204,"ancestor_record_offset":24060,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":24060,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24204,"parent_record_offset":24176,"ancestor_record_offset":24060,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3007,"record_offset":24148,"parent_record_offset":24120,"ancestor_record_offset":24060,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":24296,"parent_record_offset":24260,"ancestor_record_offset":24060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3007,"record_offset":24572,"parent_record_offset":24548,"ancestor_record_offset":24060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3007,"record_offset":24548,"parent_record_offset":24260,"ancestor_record_offset":24060,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":24464,"parent_record_offset":24436,"ancestor_record_offset":24060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support
// 记录形式: compiled_procedure; instances=1
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":248,"rva":7510720,"va":5376219840}
// 编译实例 RVAs: 0x729ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14117ab60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: physical_device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10817632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219840,"raw_hex":"e86010a500","size":5,"soft_fail":false,"target_va":5387037536,"va":5376219899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":404,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":553,"record_offset":484,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":553,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":553,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":248,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":553,"record_offset":452,"parent_record_offset":404,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 411
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111B8
// 首条位置: {"module":1593,"inlinee":70072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1593,"record_offset":13800,"parent_record_offset":13752,"ancestor_record_offset":13752,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1593,"record_offset":14092,"parent_record_offset":14032,"ancestor_record_offset":13752,"inlinee":30981,"source_line":12,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1593,"record_offset":14064,"parent_record_offset":14032,"ancestor_record_offset":13752,"inlinee":70076,"source_line":32,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":14124,"parent_record_offset":14092,"ancestor_record_offset":13752,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":13752,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1593,"record_offset":13828,"parent_record_offset":13800,"ancestor_record_offset":13752,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1593,"record_offset":13972,"parent_record_offset":13856,"ancestor_record_offset":13752,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1593,"record_offset":14004,"parent_record_offset":13972,"ancestor_record_offset":13752,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1593,"record_offset":13856,"parent_record_offset":13828,"ancestor_record_offset":13752,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1593,"record_offset":13936,"parent_record_offset":13892,"ancestor_record_offset":13752,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1593,"record_offset":14184,"parent_record_offset":14032,"ancestor_record_offset":13752,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1593,"record_offset":13892,"parent_record_offset":13856,"ancestor_record_offset":13752,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/physical_device.rs
//     evidence={"module":1593,"record_offset":14032,"parent_record_offset":14004,"ancestor_record_offset":13752,"inlinee":70074,"source_line":417,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111BA
// 首条位置: {"module":1593,"inlinee":70074}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1593,"record_offset":14092,"parent_record_offset":14032,"ancestor_record_offset":14032,"inlinee":30981,"source_line":12,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::QueueFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1593,"record_offset":14064,"parent_record_offset":14032,"ancestor_record_offset":14032,"inlinee":70076,"source_line":32,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1593,"record_offset":14184,"parent_record_offset":14032,"ancestor_record_offset":14032,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":14124,"parent_record_offset":14092,"ancestor_record_offset":14032,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":14032,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::physical_device::parse_driver_version
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76DB
// 首条位置: {"module":3007,"inlinee":30427}
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
// 完整结构及原始类型记录: physical_device.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::DeviceDriverInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::physical_device::DeviceDriverInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::physical_device::DeviceDriverInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::DeviceDriverVersion
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::PhysicalDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 40
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::slice::iter::Iter", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["alloc::string::String", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["alloc::string::String", "alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["alloc::string::String", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::string::String,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::string::String", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$50::fmt<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "constructor": "alloc::sync::impl$50::fmt", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >", "constructor": "core::result::impl$25::from_iter::closure_env$0", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "ash::vk::enums::Result", "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "constructor": "core::iter::adapters::try_process", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>", "rhapsody::raw::physical_device::PhysicalDevice", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >", "core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >", "alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_drop::InPlaceDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::physical_device::PhysicalDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice>", "constructor": "alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "rhapsody::raw::physical_device::PhysicalDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rhapsody::raw::physical_device::PhysicalDevice,rhapsody::raw::physical_device::PhysicalDevice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::physical_device::PhysicalDevice", "core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::init::AvailableGraphicsDevice,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::init::AvailableGraphicsDevice", "core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >", "rhapsody::raw::init::impl$7::available_devices::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::QueueFamily
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::option::impl$49::fmt<rhapsody::raw::physical_device::QueueFamily>", "constructor": "core::option::impl$49::fmt", "arguments": ["rhapsody::raw::physical_device::QueueFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<rhapsody::raw::physical_device::QueueFamily>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["rhapsody::raw::physical_device::QueueFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::QueueFamily,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<ash::vk::definitions::QueueFamilyProperties,alloc::alloc::Global> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::physical_device::QueueFamily", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<alloc::vec::into_iter::IntoIter<ash::vk::definitions::QueueFamilyProperties,alloc::alloc::Global> >,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::physical_device::QueueFamilyIndex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::physical_device::QueueFamilyIndex>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::physical_device::QueueFamilyIndex"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: physical_device.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: physical_device.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
// 信息 ID: 1b39d994ad66edb49983b356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c8e3a -> 0x727d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
// 信息 ID: a257ae36f9d304af2744dc45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x2b10f5 -> 0x728150 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x2b156f -> 0x728150 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x2b181f -> 0x728150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
// 信息 ID: 9b16b1a8256b5bcee7c0f4a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73d01a -> 0x728050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73d397 -> 0x728050 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=1680; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
// 信息 ID: 04eb3bf29d1a08c08c2660ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x2b159b -> 0x7283f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x2b182a -> 0x7283f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes
// 信息 ID: ca6500ef8e3ed3a5c36f9754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x2b11ee -> 0x7282f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x2b12cc -> 0x7282f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=2196; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::name
// 信息 ID: 34a7f8710e6ad92c7a4ee553
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<alloc::string::String,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<alloc::string::String,alloc::vec::impl$19::extend_trusted::closure_env$0<alloc::string::String,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::new::closure_env$2<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > > > > > @ 0x2b350 -> 0x727f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::create_device @ 0x73304e -> 0x727f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > > @ 0x1914c5f -> 0x727f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
// 信息 ID: 6323ff93b041d362cf2e83e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget @ 0x741a01 -> 0x728590 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices
// 信息 ID: 3adc4b92aea2702103571b96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > @ 0x346820 -> 0x728930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$0
// 信息 ID: 30f1cd330fb82b41ad4d99cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0 <- rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; module=1589, inline_record=960; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>; module=1589, inline_record=1372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1
// 信息 ID: a36557fb64794ab9c9e72bbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,rhapsody::raw::physical_device::PhysicalDevice,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> >,core::result::impl$25::from_iter::closure_env$0<rhapsody::raw::physical_device::PhysicalDevice,ash::vk::enums::Result,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1> >,alloc::vec::Vec<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 3 条。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > > @ 0x117a66b -> 0x19168c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > > @ 0x117a6e3 -> 0x19168c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > > @ 0x117a87c -> 0x19168c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$1>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,ash::vk::enums::Result> > > > @ 0x117a8dc -> 0x19168c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$1
// 信息 ID: 2f459fae22ffd87a3135a16d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1; module=3007, inline_record=26184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure$2
// 信息 ID: 3de02a50d7b0eef471ead088
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::physical_device::enumerate_physical_devices", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,tuple$<> >,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2>,hashbrown::set::impl$11::extend::closure_env$0<alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::ExtensionProperties>,rhapsody::raw::physical_device::enumerate_physical_devices::closure$1::closure_env$2> > > > > >; module=3007, inline_record=19696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::enumerate_vk_data<u32,ash::vk::definitions::PhysicalDevice,rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0>
// 信息 ID: 23fa54a192a03e315f722875
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::physical_device::enumerate_vk_data -> ["u32", "ash::vk::definitions::PhysicalDevice", "rhapsody::raw::physical_device::enumerate_physical_devices::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::physical_device::enumerate_physical_devices @ 0x72894c -> 0x1176db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$13::fmt
// 信息 ID: ff8aeddda10669b3b6c5dde6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$16::fmt
// 信息 ID: 03ddcbbf3d7bf6c401fad274
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::physical_device::QueueFamily>; module=1589, inline_record=400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$17::clone
// 信息 ID: 95b0e6ea464986aca1c5bc01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=35164; 不是独立调用边。
// [PDB 内联宿主] alloc::slice::impl$9::to_owned<rhapsody::raw::init::AvailableGraphicsDevice>; module=778, inline_record=1524; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,rhapsody::raw::init::impl$7::order_and_filter_devices::closure_env$1>; module=1593, inline_record=7496; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=1356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$18::fmt
// 信息 ID: 7346b0d24db11d675fa10616
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$1::fmt
// 信息 ID: 27de6a52e1d041c53750efc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$21::fmt
// 信息 ID: e4915b8a6fcccfa571769d65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$3::fmt
// 信息 ID: 7a56e22a021cfb53ccdd6c70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c8e07 -> 0x727d30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::render_config::impl$1::from @ 0x5133a4 -> 0x727d30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::physical_device::impl$1::fmt @ 0x727a34 -> 0x727d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes::closure$0
// 信息 ID: 8e624f5092a89c1527643386
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::device_local_memory_budget_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes; module=548, inline_record=6624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::device_local_memory_bytes::closure$0
// 信息 ID: 98a6bd59f70778f50097833b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::device_local_memory_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes; module=548, inline_record=5736; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=2072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes::closure$0
// 信息 ID: 2d2c18b0a57acf05f3f36d7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::host_memory_budget_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes; module=548, inline_record=8724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$4::host_memory_bytes::closure$0
// 信息 ID: 6cece37c875668531e30742b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$4::host_memory_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::PhysicalDevice::host_memory_bytes; module=548, inline_record=7844; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::init::AvailableGraphicsDevice,core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::init::AvailableGraphicsDevice,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> >,rhapsody::raw::init::impl$7::available_devices::closure_env$0> > > >; module=3007, inline_record=2380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$5::fmt
// 信息 ID: f49eac4fca7915fdc84dc0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$50::fmt<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>; module=1586, inline_record=6912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support
// 信息 ID: 1145ad4ce0561fa084833b4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::new<ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> >,ref$<alloc::sync::Arc<winit::window::Window,alloc::alloc::Global> > > @ 0x3468a1 -> 0x729ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0
// 信息 ID: 7bfddbaa61e6605a3fcdad9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$6::with_presentation_support", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >; module=1593, inline_record=13752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::impl$6::with_presentation_support::closure$0::closure$0
// 信息 ID: 77a5467b091e42b3b2e3f33c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::physical_device::impl$6::with_presentation_support", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::physical_device::PhysicalDevice,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global>,rhapsody::raw::physical_device::impl$6::with_presentation_support::closure_env$0> >; module=1593, inline_record=14032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::parse_driver_version
// 信息 ID: d7fa45d48c7d388346107edf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1; module=3007, inline_record=25204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::physical_device::string_from_c_array
// 信息 ID: bfa6bb696cde1dafc93ae54f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1; module=3007, inline_record=24060; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::physical_device::enumerate_physical_devices::closure$1; module=3007, inline_record=24632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
