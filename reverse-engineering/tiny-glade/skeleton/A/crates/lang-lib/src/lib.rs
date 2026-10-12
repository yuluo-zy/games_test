// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/lang-lib/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d0150431b90b75524a2fc72b539bf80feb69bfc6ef2414c25fb0903e57d5a04b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 538, 1545
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"PDB_registry_path_only","version":"1.7.1"}]
// use core; // namespace candidate
//   package identities: []
// use lang_lib; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: lang_lib::lang_pack_path
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7344
// 首条位置: {"module":538,"inlinee":29508}
// 编译实例 RVAs: 0x718640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3399516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149056,"raw_hex":"e8a420ccff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376149223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1380722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149056,"raw_hex":"e872111500","size":5,"soft_fail":false,"target_va":5377529888,"va":5376149161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1290572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149056,"raw_hex":"e84cb11300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376149199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":538,"record_offset":356,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":436,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":600,"parent_record_offset":544,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":538,"record_offset":404,"parent_record_offset":380,"ancestor_record_offset":244,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":380,"parent_record_offset":356,"ancestor_record_offset":244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":472,"parent_record_offset":436,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":880,"rva":7440192,"va":5376149312}
// 编译实例 RVAs: 0x718740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3835252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e88c7ac5ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5376149455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3399730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e8ce1fccff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376149437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-978345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e85712f1ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376149396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e8ce000000","size":5,"soft_fail":false,"target_va":5376149552,"va":5376149341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e8b1120000","size":5,"soft_fail":false,"target_va":5376154144,"va":5376149354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1256889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e8b92d1300","size":5,"soft_fail":false,"target_va":5377406304,"va":5376149410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1290383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149312,"raw_hex":"e88fb01300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376149388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":1808,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":538,"record_offset":1024,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":996,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":880,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":1420,"parent_record_offset":1364,"ancestor_record_offset":880,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":880,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":880,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":1228,"parent_record_offset":1024,"ancestor_record_offset":880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":880,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":880,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":880,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":880,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":880,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":880,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":1840,"rva":7440432,"va":5376149552}
// 编译实例 RVAs: 0x718830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd7c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4569563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e82546baff","size":5,"soft_fail":false,"target_va":5371582400,"va":5376151958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3400757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8cb1bccff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376150448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3400074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8761eccff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376149781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-978742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8ca10f1ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376149793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14116cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10830036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8d440a500","size":5,"soft_fail":false,"target_va":5386980304,"va":5376150263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26826776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e818589901","size":5,"soft_fail":false,"target_va":5402978144,"va":5376151363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1380180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8540f1500","size":5,"soft_fail":false,"target_va":5377529888,"va":5376149703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10831298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8c245a500","size":5,"soft_fail":false,"target_va":5386981936,"va":5376150633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1213987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e823861200","size":5,"soft_fail":false,"target_va":5377364336,"va":5376150344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10830872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e81844a500","size":5,"soft_fail":false,"target_va":5386981472,"va":5376150595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26827450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8ba5a9901","size":5,"soft_fail":false,"target_va":5402978944,"va":5376151489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::try_from_yml
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10829960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e88840a500","size":5,"soft_fail":false,"target_va":5386979776,"va":5376149811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1203728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8105e1200","size":5,"soft_fail":false,"target_va":5377353776,"va":5376150043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1249129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e8690f1300","size":5,"soft_fail":false,"target_va":5377399792,"va":5376150658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1290020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376149552,"raw_hex":"e824af1300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376149751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":538,"record_offset":4212,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":538,"record_offset":4616,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":13889,"source_line":466,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":2772,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":538,"record_offset":2800,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":3584,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":6196,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":3552,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::lang_pack_path
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":538,"record_offset":1996,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":29508,"source_line":17,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":538,"record_offset":2828,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":538,"record_offset":2688,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":538,"record_offset":4972,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":16002,"source_line":229,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":2548,"parent_record_offset":1840,"ancestor_record_offset":1840,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":1840,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":1840,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1840,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":4284,"parent_record_offset":4248,"ancestor_record_offset":1840,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":4316,"parent_record_offset":4284,"ancestor_record_offset":1840,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":2268,"parent_record_offset":2212,"ancestor_record_offset":1840,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":538,"record_offset":4248,"parent_record_offset":4212,"ancestor_record_offset":1840,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":538,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":1840,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":538,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1840,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":1840,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1840,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":538,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":1840,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":538,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":1840,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":538,"record_offset":4760,"parent_record_offset":4616,"ancestor_record_offset":1840,"inlinee":9148,"source_line":169,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":538,"record_offset":4704,"parent_record_offset":4616,"ancestor_record_offset":1840,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":538,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":1840,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1840,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":2140,"parent_record_offset":2104,"ancestor_record_offset":1840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":1840,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":4648,"parent_record_offset":4616,"ancestor_record_offset":1840,"inlinee":12880,"source_line":1493,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":4876,"parent_record_offset":4840,"ancestor_record_offset":1840,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":1840,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1840,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":1840,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":1840,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":4932,"parent_record_offset":4840,"ancestor_record_offset":1840,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":1840,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] lang_lib::load_locale_impl::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":538,"record_offset":6236,"parent_record_offset":6196,"ancestor_record_offset":1840,"inlinee":29510,"source_line":56,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1840,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1840,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1840,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":2104,"parent_record_offset":1996,"ancestor_record_offset":1840,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1840,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10B1D
// 首条位置: {"module":1545,"inlinee":68381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1545,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":612,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":500,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":828,"parent_record_offset":772,"ancestor_record_offset":500,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1545,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":500,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1545,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":500,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1545,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":500,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1545,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":500,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7346
// 首条位置: {"module":538,"inlinee":29510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":538,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":6236,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":6296,"parent_record_offset":6236,"ancestor_record_offset":6236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":8876,"rva":7443168,"va":5376152288}
// 编译实例 RVAs: 0x7192e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3838277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8bb6ec5ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5376152480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3402840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8a813ccff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376152531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3402751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e80114ccff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376152458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-981328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8b006f1ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376152379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26825469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8fd529901","size":5,"soft_fail":false,"target_va":5402978144,"va":5376152670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1211985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8517e1200","size":5,"soft_fail":false,"target_va":5377364336,"va":5376152346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e831f4ffff","size":5,"soft_fail":false,"target_va":5376149552,"va":5376152570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e84d070000","size":5,"soft_fail":false,"target_va":5376154192,"va":5376152318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1253177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8391f1300","size":5,"soft_fail":false,"target_va":5377405616,"va":5376152434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1287361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152288,"raw_hex":"e8c1a41300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376152410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":538,"record_offset":11100,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":9084,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":11596,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":9004,"parent_record_offset":8876,"ancestor_record_offset":8876,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":11384,"parent_record_offset":11360,"ancestor_record_offset":8876,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":8876,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":8876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":11360,"parent_record_offset":11332,"ancestor_record_offset":8876,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":9736,"parent_record_offset":9680,"ancestor_record_offset":8876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":8876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":8876,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":8876,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":11480,"parent_record_offset":11304,"ancestor_record_offset":8876,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":8876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":538,"record_offset":11240,"parent_record_offset":11212,"ancestor_record_offset":8876,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":538,"record_offset":11212,"parent_record_offset":11184,"ancestor_record_offset":8876,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":538,"record_offset":11156,"parent_record_offset":11128,"ancestor_record_offset":8876,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":538,"record_offset":11184,"parent_record_offset":11156,"ancestor_record_offset":8876,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":538,"record_offset":11128,"parent_record_offset":11100,"ancestor_record_offset":8876,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":11276,"parent_record_offset":11240,"ancestor_record_offset":8876,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":538,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":8876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":538,"record_offset":11516,"parent_record_offset":11240,"ancestor_record_offset":8876,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":9032,"parent_record_offset":9004,"ancestor_record_offset":8876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":11436,"parent_record_offset":11408,"ancestor_record_offset":8876,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":10124,"parent_record_offset":9116,"ancestor_record_offset":8876,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] lang_lib::load_last_used_locale::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":538,"record_offset":9116,"parent_record_offset":9084,"ancestor_record_offset":8876,"inlinee":29512,"source_line":70,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":8876,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8876,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":8876,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":538,"record_offset":9320,"parent_record_offset":9116,"ancestor_record_offset":8876,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9288,"parent_record_offset":9116,"ancestor_record_offset":8876,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":8876,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9148,"parent_record_offset":9116,"ancestor_record_offset":8876,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9352,"parent_record_offset":9320,"ancestor_record_offset":8876,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7348
// 首条位置: {"module":538,"inlinee":29512}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":10192,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":10124,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":538,"record_offset":9320,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9288,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9148,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":9116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":9736,"parent_record_offset":9680,"ancestor_record_offset":9116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":9116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":538,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":9116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":9520,"parent_record_offset":9320,"ancestor_record_offset":9116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":9116,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":9116,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":538,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":9116,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":9116,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":9116,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":538,"record_offset":9352,"parent_record_offset":9320,"ancestor_record_offset":9116,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::lookup
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":12000,"rva":7443760,"va":5376152880}
// 编译实例 RVAs: 0x719530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6538790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e8da399cff","size":5,"soft_fail":false,"target_va":5369614224,"va":5376153009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14116bbb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10822408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e80823a500","size":5,"soft_fail":false,"target_va":5386976176,"va":5376153763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27440149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e815b4a201","size":5,"soft_fail":false,"target_va":5403593344,"va":5376153190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10828341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e8353aa500","size":5,"soft_fail":false,"target_va":5386981936,"va":5376153590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10828763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e8db3ba500","size":5,"soft_fail":false,"target_va":5386981728,"va":5376152960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10824762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e83a2ca500","size":5,"soft_fail":false,"target_va":5386978160,"va":5376153393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1200441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376152880,"raw_hex":"e839511200","size":5,"soft_fail":false,"target_va":5377353776,"va":5376153330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":538,"record_offset":13620,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":538,"record_offset":12132,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":13864,"source_line":453,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":13452,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":538,"record_offset":13480,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":12188,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13816,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::try_get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":538,"record_offset":12216,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":29514,"source_line":34,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":538,"record_offset":13508,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":538,"record_offset":13368,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":538,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":12000,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":538,"record_offset":14300,"parent_record_offset":13864,"ancestor_record_offset":12000,"inlinee":9124,"source_line":65,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":538,"record_offset":14244,"parent_record_offset":13864,"ancestor_record_offset":12000,"inlinee":9123,"source_line":166,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":538,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":12000,"inlinee":9121,"source_line":104,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":538,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":12000,"inlinee":9133,"source_line":186,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":12000,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":538,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":12000,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":538,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":12000,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":538,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":12000,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":538,"record_offset":14028,"parent_record_offset":13864,"ancestor_record_offset":12000,"inlinee":9125,"source_line":140,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":538,"record_offset":13936,"parent_record_offset":13908,"ancestor_record_offset":12000,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":538,"record_offset":12696,"parent_record_offset":12672,"ancestor_record_offset":12000,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":12000,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":13336,"parent_record_offset":12216,"ancestor_record_offset":12000,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":13908,"parent_record_offset":13864,"ancestor_record_offset":12000,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":12000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":12000,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12000,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13964,"parent_record_offset":13936,"ancestor_record_offset":12000,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":13992,"parent_record_offset":13936,"ancestor_record_offset":12000,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":538,"record_offset":14384,"parent_record_offset":14300,"ancestor_record_offset":12000,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":13220,"parent_record_offset":13192,"ancestor_record_offset":12000,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":13248,"parent_record_offset":13220,"ancestor_record_offset":12000,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":538,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":12000,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":12000,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14144,"parent_record_offset":14108,"ancestor_record_offset":12000,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":12000,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":12000,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":12000,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":12000,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":14200,"parent_record_offset":14108,"ancestor_record_offset":12000,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":538,"record_offset":13108,"parent_record_offset":13080,"ancestor_record_offset":12000,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":12000,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":12000,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":13080,"parent_record_offset":12860,"ancestor_record_offset":12000,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12320,"parent_record_offset":12288,"ancestor_record_offset":12000,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12000,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12968,"parent_record_offset":12860,"ancestor_record_offset":12000,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":12000,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12828,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":12000,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":12000,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12348,"parent_record_offset":12288,"ancestor_record_offset":12000,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":12000,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12640,"parent_record_offset":12612,"ancestor_record_offset":12000,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12672,"parent_record_offset":12640,"ancestor_record_offset":12000,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12612,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12444,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12860,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12472,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12556,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12736,"parent_record_offset":12396,"ancestor_record_offset":12000,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":12000,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00538.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: lang_lib::lang_pack_path
// 信息 ID: 8b9ee38635c387241128b5e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0> @ 0x116cc31 -> 0x718640 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=538, inline_record=1996; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=538, inline_record=2856; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=538, inline_record=5496; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale
// 信息 ID: 5ed51acdfdce911353dbbd0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x6596 -> 0x7192e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale::closure$0
// 信息 ID: b6d141784cc69565af8eaecf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_last_used_locale", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_last_used_locale; module=538, inline_record=9116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale
// 信息 ID: 9caf445240cb7ba193ee2b54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0> @ 0x7f3b1 -> 0x718740 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf0ab1 -> 0x718740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl
// 信息 ID: 5adcd7ad645e3153fcefa8db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale @ 0x71875d -> 0x718830 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_last_used_locale @ 0x7193fa -> 0x718830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$0
// 信息 ID: 6d0b49467336d7edeb3ad73d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::load_locale_impl::closure_env$0 <- anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; module=1545, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$1
// 信息 ID: 3a26df5a29b58bc4ae821fa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=538, inline_record=6236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::lookup
// 信息 ID: f0670188560e34270f21c9f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 147 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e12c -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e718 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e781 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e72b -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> @ 0xd6e40 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x2841ec -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::impl$0::fmt @ 0x2ef520 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2ef6ee -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2ef768 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2ef85b -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2ef8c8 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f01a0 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f01e2 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0222 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0262 -> 0x719530 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f02aa -> 0x719530 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
