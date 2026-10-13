// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/file.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d6db47c480aed8efa341db1286e9461e196896e09e0f47aa52d5b8031995430e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 617, 1710
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"PDB_registry_path_only","version":"0.3.36"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::file::date_time_string_for_filename
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":617,"record_offset":1424,"rva":8235888,"va":5376945008}
// 编译实例 RVAs: 0x7dab70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4631329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e8df54b9ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5376945532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4195520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e840fbbfff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376945227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26034819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e883428d01","size":5,"soft_fail":false,"target_va":5402980128,"va":5376945304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::format_description::parse::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10539543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e817d2a000","size":5,"soft_fail":false,"target_va":5387484656,"va":5376945108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::formattable::sealed::Sealed::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10540006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e8e6d3a000","size":5,"soft_fail":false,"target_va":5387485184,"va":5376945173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t43372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e86ca90000","size":5,"soft_fail":false,"target_va":5376988416,"va":5376945039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t43372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945008,"raw_hex":"e86ca90000","size":5,"soft_fail":false,"target_va":5376988464,"va":5376945087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":2020,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":617,"record_offset":1648,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":617,"record_offset":1924,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":12428,"source_line":1271,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] utils::local_time::now_appox_local
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":617,"record_offset":1564,"parent_record_offset":1424,"ancestor_record_offset":1424,"inlinee":36037,"source_line":18,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1424,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":1844,"parent_record_offset":1788,"ancestor_record_offset":1424,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":1424,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1424,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":617,"record_offset":2136,"parent_record_offset":2104,"ancestor_record_offset":1424,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":617,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":1424,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":1688,"parent_record_offset":1648,"ancestor_record_offset":1424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] utils::local_time::now_appox_local::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":617,"record_offset":1616,"parent_record_offset":1588,"ancestor_record_offset":1424,"inlinee":36034,"source_line":20,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::generate_backup_file_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":617,"record_offset":2464,"rva":8236432,"va":5376945552}
// 编译实例 RVAs: 0x7dad90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4196097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e8fff8bfff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376945804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t459371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e86b020700","size":5,"soft_fail":false,"target_va":5377405008,"va":5376945632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t584134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e8c6e90800","size":5,"soft_fail":false,"target_va":5377529888,"va":5376945749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26033038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e88e3b8d01","size":5,"soft_fail":false,"target_va":5402978912,"va":5376945869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_with_file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t494247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e8a78a0700","size":5,"soft_fail":false,"target_va":5377440032,"va":5376945780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t491667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e893800700","size":5,"soft_fail":false,"target_va":5377437280,"va":5376945608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376945552,"raw_hex":"e8aefdffff","size":5,"soft_fail":false,"target_va":5376945008,"va":5376945597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":617,"record_offset":2724,"parent_record_offset":2464,"ancestor_record_offset":2464,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":617,"record_offset":2604,"parent_record_offset":2464,"ancestor_record_offset":2464,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":617,"record_offset":3448,"parent_record_offset":2464,"ancestor_record_offset":2464,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":3244,"parent_record_offset":2464,"ancestor_record_offset":2464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_file_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":617,"record_offset":2808,"parent_record_offset":2464,"ancestor_record_offset":2464,"inlinee":36115,"source_line":2579,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2464,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":2968,"parent_record_offset":2912,"ancestor_record_offset":2464,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":617,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":2464,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2464,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2464,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":617,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":2464,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":617,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":2464,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":2464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":617,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":2464,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":617,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2464,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::rename_file_to_backup
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":617,"record_offset":3684,"rva":8236896,"va":5376946016}
// 编译实例 RVAs: 0x7daf60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4196571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e825f7bfff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376946278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1775163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e8c5e9e4ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5376946214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e8780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10540853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e835d7a000","size":5,"soft_fail":false,"target_va":5387487104,"va":5376946246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t407573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e815380600","size":5,"soft_fail":false,"target_va":5377353776,"va":5376946198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t528948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e834120800","size":5,"soft_fail":false,"target_va":5377475184,"va":5376946231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::generate_backup_file_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946016,"raw_hex":"e8f6fdffff","size":5,"soft_fail":false,"target_va":5376945552,"va":5376946069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":617,"record_offset":3908,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":617,"record_offset":3936,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":4216,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":617,"record_offset":3964,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":617,"record_offset":3824,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":617,"record_offset":3992,"parent_record_offset":3684,"ancestor_record_offset":3684,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":3684,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":617,"record_offset":4436,"parent_record_offset":4380,"ancestor_record_offset":3684,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":4408,"parent_record_offset":4380,"ancestor_record_offset":3684,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":617,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":3684,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":617,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3684,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":617,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3684,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":617,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3684,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":617,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3684,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":617,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3684,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":617,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3684,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":617,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3684,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":617,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3684,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":617,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":3684,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":617,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3684,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00617.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::rename_file_to_backup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11DAF
// 首条位置: {"module":1710,"inlinee":73135}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1710,"record_offset":488,"parent_record_offset":468,"ancestor_record_offset":468,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01710.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1710,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":468,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1710,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":468,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01710.jsonl.gz"}
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
// 完整结构及原始类型记录: file.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: file.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: file.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::file::date_time_string_for_filename
// 信息 ID: 8f185aed073eb4eca66795fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x159386 -> 0x7dab70 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278b0d -> 0x7dab70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_camera_snapshot_ui::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x34be11 -> 0x7dab70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_freecam_save::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x34d41e -> 0x7dab70 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::generate_backup_file_path @ 0x7dadbd -> 0x7dab70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::generate_backup_file_path
// 信息 ID: 4d25b14ecbe7dcbafef7fb65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::file::rename_file_to_backup @ 0x7daf95 -> 0x7dad90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::rename_file_to_backup
// 信息 ID: 3f14e03891afccb2ccbbc3fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x7913 -> 0x7daf60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x17f9ced -> 0x7daf60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::rename_file_to_backup::closure$0
// 信息 ID: 74b9e53fb606c5474f27f629
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::file::rename_file_to_backup", "captures": "unknown"}
// [闭包环境类型引用] utils::file::rename_file_to_backup::closure_env$0 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::rename_file_to_backup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::rename_file_to_backup::closure_env$0>; module=1710, inline_record=468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
