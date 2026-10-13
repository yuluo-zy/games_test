// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/paths/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e96b459fcb26b356a630c587ee39497ad770aa821afe60743d3ad6e4a976c1a5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 539, 1552, 2993
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: paths::config
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":244,"rva":7444784,"va":5376153904}
// 编译实例 RVAs: 0x719930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376153904,"raw_hex":"e8e3509901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376153944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":348,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":372,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":408,"rva":7444832,"va":5376153952}
// 编译实例 RVAs: 0x719960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376153952,"raw_hex":"e8b8509901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376153987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":508,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":532,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::tmp
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":564,"rva":7444880,"va":5376154000}
// 编译实例 RVAs: 0x719990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154000,"raw_hex":"e883509901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376154040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":664,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":688,"parent_record_offset":564,"ancestor_record_offset":564,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::deleted_saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":724,"rva":7444928,"va":5376154048}
// 编译实例 RVAs: 0x7199c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154048,"raw_hex":"e853509901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376154088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":832,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":856,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::installed
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":892,"rva":7444976,"va":5376154096}
// 编译实例 RVAs: 0x7199f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154096,"raw_hex":"e823509901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376154136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":996,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":1020,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":1056,"rva":7445024,"va":5376154144}
// 编译实例 RVAs: 0x719a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26824691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154144,"raw_hex":"e8f34f9901","size":5,"soft_fail":false,"target_va":5402978880,"va":5376154184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":1176,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":1200,"parent_record_offset":1056,"ancestor_record_offset":1056,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::find_standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":1236,"rva":7445072,"va":5376154192}
// 编译实例 RVAs: 0x719a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3404664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e8880cccff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376154371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-983347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e8cdfef0ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376154398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h95c574544334ca48 (.llvm.12705168316278483607)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27056542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e89ed99c01","size":5,"soft_fail":false,"target_va":5403211136,"va":5376154589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] known_folders::win::get_known_folder_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10827781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e80538a500","size":5,"soft_fail":false,"target_va":5386982016,"va":5376154230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1241045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e8d5ef1200","size":5,"soft_fail":false,"target_va":5377395552,"va":5376154502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1245520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e850011300","size":5,"soft_fail":false,"target_va":5377399792,"va":5376154267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1285424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154192,"raw_hex":"e8309d1300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376154347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":539,"record_offset":1408,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":1836,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":539,"record_offset":1780,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":539,"record_offset":2508,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":539,"record_offset":1380,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":16002,"source_line":229,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":1808,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":2036,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1236,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1236,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":1236,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1236,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":539,"record_offset":1488,"parent_record_offset":1448,"ancestor_record_offset":1236,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":539,"record_offset":1448,"parent_record_offset":1408,"ancestor_record_offset":1236,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":1520,"parent_record_offset":1448,"ancestor_record_offset":1236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1236,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1236,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1236,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1236,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1552,"record_offset":244,"rva":18274768,"va":5386983888}
// 编译实例 RVAs: 0x116d9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14234365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e803cd26ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386984056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14234300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e844cd26ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386984007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11812868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e8fcbf4bff","size":5,"soft_fail":false,"target_va":5375171056,"va":5386983919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9547543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e8e9506eff","size":5,"soft_fail":false,"target_va":5377436496,"va":5386984034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141913a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8019999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e81f607a00","size":5,"soft_fail":false,"target_va":5395003968,"va":5386983964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15994019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e8a30cf400","size":5,"soft_fail":false,"target_va":5402978144,"va":5386984120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9575205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386983888,"raw_hex":"e8dbe46dff","size":5,"soft_fail":false,"target_va":5377408752,"va":5386983952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1552,"record_offset":1008,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1552,"record_offset":672,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1552,"record_offset":644,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1552,"record_offset":560,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1552,"record_offset":1500,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1552,"record_offset":980,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1552,"record_offset":392,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1552,"record_offset":896,"parent_record_offset":832,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1552,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1552,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1552,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1552,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1552,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":244,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1552,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":244,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1552,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":244,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1552,"record_offset":612,"parent_record_offset":560,"ancestor_record_offset":244,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1552,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":244,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1552,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1552,"record_offset":416,"parent_record_offset":392,"ancestor_record_offset":244,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1809E
// 首条位置: {"module":2993,"inlinee":98462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2993,"record_offset":500,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02993.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2993,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":480,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02993.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2993,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":480,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02993.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::initialize_paths
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":539,"record_offset":3040,"rva":7445712,"va":5376154832}
// 编译实例 RVAs: 0x719cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3842716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8645dc5ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5376156919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3406251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e85506ccff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376155942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3405251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e83d0accff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376154958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=7
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-983846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8dafcf0ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376154897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1281602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8428e1300","size":5,"soft_fail":false,"target_va":5377436496,"va":5376154889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14116d9d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10828574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e81e3ba500","size":5,"soft_fail":false,"target_va":5386983888,"va":5376155309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26821559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8b7439901","size":5,"soft_fail":false,"target_va":5402978144,"va":5376156580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1374737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e811fa1400","size":5,"soft_fail":false,"target_va":5377529888,"va":5376155146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::had84cd1ec1367ca5 (.llvm.12705168316278483607)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27055776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8a0d69c01","size":5,"soft_fail":false,"target_va":5403211600,"va":5376155819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26822244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e864469901","size":5,"soft_fail":false,"target_va":5402978944,"va":5376156695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8effcffff","size":5,"soft_fail":false,"target_va":5376154192,"va":5376154972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1239822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e80eeb1200","size":5,"soft_fail":false,"target_va":5377395552,"va":5376155725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1243463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e847f91200","size":5,"soft_fail":false,"target_va":5377399136,"va":5376155668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1262287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8cf421300","size":5,"soft_fail":false,"target_va":5377418528,"va":5376156236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t1284843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8eb9a1300","size":5,"soft_fail":false,"target_va":5377439776,"va":5376154928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1359796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376154832,"raw_hex":"e8b4bf1400","size":5,"soft_fail":false,"target_va":5377515952,"va":5376156151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":539,"record_offset":5732,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":539,"record_offset":3844,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":539,"record_offset":4980,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":539,"record_offset":7716,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":539,"record_offset":7020,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":7604,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":7632,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":3396,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":539,"record_offset":3672,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":3364,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":6964,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":16049,"source_line":2137,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":3224,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$56::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":3196,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":4727,"source_line":1902,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":3040,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":3040,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":3040,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":5768,"parent_record_offset":5732,"ancestor_record_offset":3040,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":5800,"parent_record_offset":5768,"ancestor_record_offset":3040,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":539,"record_offset":3588,"parent_record_offset":3532,"ancestor_record_offset":3040,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":539,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3040,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":3040,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":539,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3040,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":539,"record_offset":5056,"parent_record_offset":5020,"ancestor_record_offset":3040,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":539,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":3040,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":539,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":3040,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":539,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":3040,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":3416,"parent_record_offset":3396,"ancestor_record_offset":3040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":5876,"parent_record_offset":5848,"ancestor_record_offset":3040,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":539,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":3040,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":3040,"inlinee":6742,"source_line":865,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":3040,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":3040,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":539,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3040,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":539,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":3040,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00539.jsonl.gz"}
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
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: paths::config
// 信息 ID: 50d6a7a35ddf238b34c9d6b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x159345 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x325cc6 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_game_config_to_disk @ 0x326343 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_settings_file @ 0x326b51 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_settings_file @ 0x3c19f6 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::profile_file_path @ 0x5b49f8 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0x5b4a50 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0x5b505f -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0x5b5230 -> 0x719930 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::photomode_buttons::photomode_buttons @ 0x1b5d5e8 -> 0x719930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::deleted_saves
// 信息 ID: 3a3023f31d9ba85353072f93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::delete_session @ 0x54aa9f -> 0x7199c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path::closure$0
// 信息 ID: 4c22042976ca3fa89a0d21d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "paths::ensure_path", "captures": "unknown"}
// [闭包环境类型引用] paths::ensure_path::closure_env$0 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; module=2993, inline_record=480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path<std::path::PathBuf>
// 信息 ID: b5b782fc1759754542634f76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] paths::ensure_path -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] paths::initialize_paths @ 0x719ead -> 0x116d9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0x719f23 -> 0x116d9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0x719f99 -> 0x116d9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0x719fe5 -> 0x116d9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::find_standard_app_root_path
// 信息 ID: d084988717bfb91527fe227a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_last_used_locale @ 0x7192fe -> 0x719a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0x719d5c -> 0x719a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::initialize_paths
// 信息 ID: 3b749add8e608de905afd141
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x399917 -> 0x719cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::installed
// 信息 ID: 1d34133e50efa57b9efea77b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x325d8e -> 0x7199f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::update_vulkan_allowlist_defaults @ 0x326671 -> 0x7199f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::saves
// 信息 ID: b2eb94b04d2de287699b3e21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::SessionId::save_folder_path @ 0x54a073 -> 0x719960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::refresh_session_list @ 0x54a8ae -> 0x719960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::standard_app_root_path
// 信息 ID: 6b93cb23aea2f81e25105590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x2f5437 -> 0x719a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale @ 0x71876a -> 0x719a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::tmp
// 信息 ID: 968d4c2256fc44fb42845ba5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::logging::set_up_logging @ 0x2ee74d -> 0x719990 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_log_file @ 0x3c0e67 -> 0x719990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
