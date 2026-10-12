// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/motomoto/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 31e083c1b504e0ba62ca597c4f56167d61c619ef272f30a86ff04e5608f138c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 449, 500, 1410, 1438, 1443, 1446
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"PDB_registry_path_only","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use motomoto; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: motomoto::impl$0::as_tag
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":248,"rva":1104432,"va":5369813552}
// 编译实例 RVAs: 0x10da30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$1::as_tag
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":348,"rva":17854704,"va":5386563824}
// 编译实例 RVAs: 0x11070f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17029495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386563824,"raw_hex":"e877d90301","size":5,"soft_fail":false,"target_va":5403593360,"va":5386563860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16415246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386563824,"raw_hex":"e80e7afa00","size":5,"soft_fail":false,"target_va":5402979152,"va":5386563901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] motomoto::tag_from_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1446,"record_offset":456,"parent_record_offset":348,"ancestor_record_offset":348,"inlinee":66895,"source_line":63,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1446,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":348,"inlinee":66896,"source_line":47,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_le_bytes_truncated
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10550
// 首条位置: {"module":1446,"inlinee":66896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1054F
// 首条位置: {"module":1446,"inlinee":66895}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1446,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":456,"inlinee":66896,"source_line":47,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::tag_from_str_trunc
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":528,"rva":17854800,"va":5386563920}
// 编译实例 RVAs: 0x1107150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17029392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386563920,"raw_hex":"e810d90301","size":5,"soft_fail":false,"target_va":5403593360,"va":5386563963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] motomoto::tag_from_le_bytes_truncated
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1446,"record_offset":640,"parent_record_offset":528,"ancestor_record_offset":528,"inlinee":66896,"source_line":47,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64CA
// 首条位置: {"module":1438,"inlinee":25802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":449,"record_offset":3976,"rva":6323408,"va":5375032528}
// 编译实例 RVAs: 0x607cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411070f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11531220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032528,"raw_hex":"e8d4f3af00","size":5,"soft_fail":false,"target_va":5386563824,"va":5375032599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28560715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032528,"raw_hex":"e84bcdb301","size":5,"soft_fail":false,"target_va":5403593392,"va":5375032672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27886229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032528,"raw_hex":"e89582a901","size":5,"soft_fail":false,"target_va":5402919072,"va":5375032838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":449,"record_offset":4300,"parent_record_offset":3976,"ancestor_record_offset":3976,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":449,"record_offset":4188,"parent_record_offset":3976,"ancestor_record_offset":3976,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":449,"record_offset":4272,"parent_record_offset":3976,"ancestor_record_offset":3976,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":3976,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":4664,"parent_record_offset":4632,"ancestor_record_offset":3976,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":4632,"parent_record_offset":4604,"ancestor_record_offset":3976,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":4516,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":4572,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":4604,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":4844,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":4732,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":449,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":3976,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":4484,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":3976,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":3976,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":3976,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3976,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":4240,"parent_record_offset":4188,"ancestor_record_offset":3976,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":449,"record_offset":4788,"parent_record_offset":4424,"ancestor_record_offset":3976,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":449,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":3976,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":449,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":3976,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":4356,"parent_record_offset":4328,"ancestor_record_offset":3976,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":4424,"parent_record_offset":4384,"ancestor_record_offset":3976,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":4384,"parent_record_offset":4356,"ancestor_record_offset":3976,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":4876,"parent_record_offset":4384,"ancestor_record_offset":3976,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":449,"record_offset":5736,"rva":6323776,"va":5375032896}
// 编译实例 RVAs: 0x607e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5219415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032896,"raw_hex":"e8a95bb0ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5375032962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28560352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032896,"raw_hex":"e8e0cbb301","size":5,"soft_fail":false,"target_va":5403593392,"va":5375033035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27885866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375032896,"raw_hex":"e82a81a901","size":5,"soft_fail":false,"target_va":5402919072,"va":5375033201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":449,"record_offset":6052,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":449,"record_offset":5940,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":449,"record_offset":6024,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":5736,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":6416,"parent_record_offset":6384,"ancestor_record_offset":5736,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":5736,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":6268,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":6324,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":6356,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":6596,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":6484,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":449,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":5736,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":6236,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":5736,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":5736,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":5736,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":5736,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":5992,"parent_record_offset":5940,"ancestor_record_offset":5736,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":449,"record_offset":6540,"parent_record_offset":6176,"ancestor_record_offset":5736,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":449,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":5736,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":449,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":5736,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":5736,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":6176,"parent_record_offset":6136,"ancestor_record_offset":5736,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":5736,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":6628,"parent_record_offset":6136,"ancestor_record_offset":5736,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$3::write
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64B8
// 首条位置: {"module":449,"inlinee":25784}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":468,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":468,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":468,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":449,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":468,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":652,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":708,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":736,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":1004,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":449,"record_offset":892,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":449,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":468,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":620,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":449,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":468,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":468,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":25336,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":449,"record_offset":948,"parent_record_offset":564,"ancestor_record_offset":468,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":449,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":468,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":564,"parent_record_offset":524,"ancestor_record_offset":468,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":524,"parent_record_offset":496,"ancestor_record_offset":468,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":449,"record_offset":1036,"parent_record_offset":524,"ancestor_record_offset":468,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::ChunkWriter::seek_back_and_write_length
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64CE
// 首条位置: {"module":500,"inlinee":25806}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":500,"record_offset":99184,"parent_record_offset":99140,"ancestor_record_offset":99140,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99420,"parent_record_offset":99396,"ancestor_record_offset":99140,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99540,"parent_record_offset":99512,"ancestor_record_offset":99140,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99512,"parent_record_offset":99484,"ancestor_record_offset":99140,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99396,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99452,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99484,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99720,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99608,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":500,"record_offset":99804,"parent_record_offset":99780,"ancestor_record_offset":99140,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":99364,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":99564,"parent_record_offset":99540,"ancestor_record_offset":99140,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":99632,"parent_record_offset":99608,"ancestor_record_offset":99140,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":99780,"parent_record_offset":99752,"ancestor_record_offset":99140,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":99664,"parent_record_offset":99304,"ancestor_record_offset":99140,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":500,"record_offset":99688,"parent_record_offset":99664,"ancestor_record_offset":99140,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":500,"record_offset":99212,"parent_record_offset":99184,"ancestor_record_offset":99140,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99240,"parent_record_offset":99212,"ancestor_record_offset":99140,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99304,"parent_record_offset":99268,"ancestor_record_offset":99140,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99268,"parent_record_offset":99240,"ancestor_record_offset":99140,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99752,"parent_record_offset":99268,"ancestor_record_offset":99140,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$5::drop
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D71
// 首条位置: {"module":500,"inlinee":28017}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":99888,"parent_record_offset":99108,"ancestor_record_offset":99108,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] motomoto::ChunkWriter::seek_back_and_write_length
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":500,"record_offset":99140,"parent_record_offset":99108,"ancestor_record_offset":99108,"inlinee":25806,"source_line":114,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99420,"parent_record_offset":99396,"ancestor_record_offset":99108,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99540,"parent_record_offset":99512,"ancestor_record_offset":99108,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":99512,"parent_record_offset":99484,"ancestor_record_offset":99108,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99396,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99452,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99484,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99720,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":99608,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":500,"record_offset":99184,"parent_record_offset":99140,"ancestor_record_offset":99108,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":500,"record_offset":99804,"parent_record_offset":99780,"ancestor_record_offset":99108,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":99364,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":99564,"parent_record_offset":99540,"ancestor_record_offset":99108,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":99632,"parent_record_offset":99608,"ancestor_record_offset":99108,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":99780,"parent_record_offset":99752,"ancestor_record_offset":99108,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":99664,"parent_record_offset":99304,"ancestor_record_offset":99108,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":500,"record_offset":99688,"parent_record_offset":99664,"ancestor_record_offset":99108,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":500,"record_offset":99212,"parent_record_offset":99184,"ancestor_record_offset":99108,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99240,"parent_record_offset":99212,"ancestor_record_offset":99108,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99304,"parent_record_offset":99268,"ancestor_record_offset":99108,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99268,"parent_record_offset":99240,"ancestor_record_offset":99108,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":500,"record_offset":99752,"parent_record_offset":99268,"ancestor_record_offset":99108,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Reader::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":684,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::Reader::iter
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":784,"rva":17854880,"va":5386564000}
// 编译实例 RVAs: 0x11071a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::impl$9::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1446,"record_offset":884,"rva":17854912,"va":5386564032}
// 编译实例 RVAs: 0x11071c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16416048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386564032,"raw_hex":"e8307dfa00","size":5,"soft_fail":false,"target_va":5402980304,"va":5386564251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16416079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386564032,"raw_hex":"e84f7dfa00","size":5,"soft_fail":false,"target_va":5402980320,"va":5386564236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::transpose
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1446,"record_offset":1884,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":45117,"source_line":1607,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_direct] motomoto::ChunkIterator::try_next
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1446,"record_offset":996,"parent_record_offset":884,"ancestor_record_offset":884,"inlinee":66898,"source_line":187,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1446,"record_offset":1040,"parent_record_offset":996,"ancestor_record_offset":884,"inlinee":45284,"source_line":321,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1446,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":884,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":884,"inlinee":24845,"source_line":543,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1704,"parent_record_offset":1672,"ancestor_record_offset":884,"inlinee":9242,"source_line":1975,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1672,"parent_record_offset":1640,"ancestor_record_offset":884,"inlinee":24846,"source_line":2050,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1446,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":884,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1446,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":884,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":884,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":884,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":884,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":884,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":884,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::remaining_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":884,"inlinee":24843,"source_line":235,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$3::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1616,"parent_record_offset":996,"ancestor_record_offset":884,"inlinee":24850,"source_line":287,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::read_exact
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":884,"inlinee":24848,"source_line":354,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1446,"record_offset":1232,"parent_record_offset":1064,"ancestor_record_offset":884,"inlinee":24844,"source_line":282,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: motomoto::ChunkIterator::try_next
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10552
// 首条位置: {"module":1446,"inlinee":66898}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1446,"record_offset":1040,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":45284,"source_line":321,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1748,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$3::seek
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1616,"parent_record_offset":996,"ancestor_record_offset":996,"inlinee":24850,"source_line":287,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1446,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":996,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":996,"inlinee":24845,"source_line":543,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1704,"parent_record_offset":1672,"ancestor_record_offset":996,"inlinee":9242,"source_line":1975,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1446,"record_offset":1672,"parent_record_offset":1640,"ancestor_record_offset":996,"inlinee":24846,"source_line":2050,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1446,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":996,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1446,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":996,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":996,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":996,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":996,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":996,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1446,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":996,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::remaining_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":996,"inlinee":24843,"source_line":235,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::read_exact
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1446,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":996,"inlinee":24848,"source_line":354,"archive":"_pdb/A/modules/01446.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1446,"record_offset":1232,"parent_record_offset":1064,"ancestor_record_offset":996,"inlinee":24844,"source_line":282,"archive":"_pdb/A/modules/01446.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::ChunkIterator
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::ChunkWriter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "constructor": "motomoto::ChunkWriter", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "constructor": "ref_mut$", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >", "constructor": "flate2::deflate::write::DeflateEncoder", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::io::Write::write_all<flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > > >", "constructor": "std::io::Write::write_all", "arguments": ["flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::deflate::write::DeflateEncoder::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >", "constructor": "flate2::deflate::write::DeflateEncoder::finish", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::io::Write::write_all<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "constructor": "std::io::Write::write_all", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::Serializer", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "core::ptr::drop_in_place", "arguments": ["serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["flate2::deflate::write::DeflateEncoder<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::zio::impl$5::write<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>", "constructor": "flate2::zio::impl$5::write", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "flate2::mem::Compress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "flate2::zio::Writer::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>", "constructor": "flate2::zio::Writer::finish", "arguments": ["ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >", "flate2::mem::Compress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::format_escaped_str<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::format_escaped_str", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "ref_mut$", "arguments": ["serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> > >", "constructor": "serde_json::value::ser::impl$0::serialize", "arguments": ["ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["ref_mut$<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "ref$<alloc::vec::Vec<enum2$<serde_json::value::Value>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>", "constructor": "serde_json::ser::Compound", "arguments": ["motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >", "serde_json::ser::CompactFormatter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "constructor": "enum2$", "arguments": ["serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >,alloc::string::String,enum2$<serde_json::value::Value> >", "constructor": "serde::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >", "alloc::string::String", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::Reader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] motomoto::Writer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >", "constructor": "motomoto::Writer::add_chunk", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>", "constructor": "motomoto::Writer::add_chunk", "arguments": ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "u64"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 75; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >,bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >", "bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_fade_in_on_start::MusicFadeIn>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: motomoto::ChunkIterator::try_next
// 信息 ID: f309d0cc9ac131174ec62708
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] motomoto::impl$9::next; module=1446, inline_record=996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::ChunkWriter::seek_back_and_write_length
// 信息 ID: 94b000c06ea71624bf270afe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >; module=500, inline_record=99140; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >; module=500, inline_record=200396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=30012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=30860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=35156; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=36028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Reader::iter
// 信息 ID: c2123b9a31acab95d8feee23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x10a8c4b -> 0x11071a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Reader::new
// 信息 ID: 88a62e05af8c2303f762c324
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> >
// 信息 ID: 931d47d5f5cbfbe2893938d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] motomoto::Writer::add_chunk -> ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x10a5699 -> 0x607cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x10a62e9 -> 0x607cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,u64>
// 信息 ID: f1b4ea878b7331645e29a80e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] motomoto::Writer::add_chunk -> ["std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x10a603b -> 0x607e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::Writer::new
// 信息 ID: faf684277b3a29506b914d75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=3800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$0::as_tag
// 信息 ID: b68ce1b3e411a44ae96e5173
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 213 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9f1 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfd8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5c8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec0f -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f19f -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f727 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbcb -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201a0 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x20688 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20c4d -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x210f8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a22d -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a309 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a3ce -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a499 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game @ 0x6c120 -> 0x10da30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$1::as_tag
// 信息 ID: cf282d212962ece52ea7af93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] motomoto::Writer::add_chunk<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > >,ref$<str$> > @ 0x607d17 -> 0x11070f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$3::write
// 信息 ID: 303dc1675f22367bc3005924
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::io::Write::write_all<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >; module=449, inline_record=468; 不是独立调用边。
// [PDB 内联宿主] flate2::zio::impl$5::write<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=1410, inline_record=107212; 不是独立调用边。
// [PDB 内联宿主] flate2::zio::impl$5::write<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=1410, inline_record=107912; 不是独立调用边。
// [PDB 内联宿主] flate2::zio::Writer::finish<ref_mut$<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >,flate2::mem::Compress>; module=1410, inline_record=110016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=25336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=26452; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=27512; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=28076; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$5::drop
// 信息 ID: e00bf17a62efff65b712faec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >; module=500, inline_record=99108; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<serde_json::ser::Serializer<motomoto::ChunkWriter<std::io::cursor::Cursor<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,serde_json::ser::CompactFormatter> >; module=500, inline_record=200364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=29980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=30828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=35124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=35996; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=36808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::impl$9::next
// 信息 ID: c24bd7782e81df4ec5092fe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure$0 @ 0xde9aa5 -> 0x11071c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x10a8c5b -> 0x11071c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x10a9cc7 -> 0x11071c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_le_bytes_truncated
// 信息 ID: f1715f62305622066e6f993d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] motomoto::impl$1::as_tag; module=1446, inline_record=484; 不是独立调用边。
// [PDB 内联宿主] motomoto::tag_from_str_trunc; module=1446, inline_record=640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_str
// 信息 ID: 3eff0f76847fb5e40d87ba17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] motomoto::impl$1::as_tag; module=1446, inline_record=456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: motomoto::tag_from_str_trunc
// 信息 ID: 479bc96e2b4eae55113d1464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::capture_game_snapshot @ 0x10a6021 -> 0x1107150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
