// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/file.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 33796feed13109b745be5cb113114be2fc5cdf5fe1c1d353dd3bf2caeb320632
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 765, 1678, 2082
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::file::date_time_string_for_filename
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":1628,"rva":13198992,"va":5381908112}
// 编译实例 RVAs: 0xc96690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7007905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e85f1195ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5381908636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6826831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e8b1d497ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381908330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29634468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e8a42fc401","size":5,"soft_fail":false,"target_va":5411542880,"va":5381908407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::format_description::parse::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t102887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e8e7910100","size":5,"soft_fail":false,"target_va":5382011104,"va":5381908212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::formatting::formattable::sealed::Sealed::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13283558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e8e6b0ca00","size":5,"soft_fail":false,"target_va":5395191840,"va":5381908277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t103756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e84c950100","size":5,"soft_fail":false,"target_va":5382011904,"va":5381908143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t103830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908112,"raw_hex":"e896950100","size":5,"soft_fail":false,"target_va":5382012016,"va":5381908181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":2248,"parent_record_offset":1628,"ancestor_record_offset":1628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":765,"record_offset":1852,"parent_record_offset":1628,"ancestor_record_offset":1628,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":765,"record_offset":2152,"parent_record_offset":1628,"ancestor_record_offset":1628,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] utils::local_time::now_appox_local
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":765,"record_offset":1768,"parent_record_offset":1628,"ancestor_record_offset":1628,"inlinee":44778,"source_line":20,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":2068,"parent_record_offset":2012,"ancestor_record_offset":1628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":765,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":1628,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":765,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":1628,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":1892,"parent_record_offset":1852,"ancestor_record_offset":1628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] utils::local_time::now_appox_local::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":765,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":1628,"inlinee":44773,"source_line":22,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::generate_backup_file_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":2724,"rva":13199536,"va":5381908656}
// 编译实例 RVAs: 0xc968b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6827416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e868d297ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381908915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t796011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e86b250c00","size":5,"soft_fail":false,"target_va":5382704752,"va":5381908736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t931351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e817360e00","size":5,"soft_fail":false,"target_va":5382840208,"va":5381908852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29632823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e83729c401","size":5,"soft_fail":false,"target_va":5411541808,"va":5381908980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_with_file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t835440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e870bf0c00","size":5,"soft_fail":false,"target_va":5382744336,"va":5381908891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t832787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e813b50c00","size":5,"soft_fail":false,"target_va":5382741504,"va":5381908712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908656,"raw_hex":"e8aefdffff","size":5,"soft_fail":false,"target_va":5381908112,"va":5381908701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":765,"record_offset":2960,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":765,"record_offset":3044,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":765,"record_offset":2868,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":765,"record_offset":3792,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":3564,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_file_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":765,"record_offset":3072,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":21960,"source_line":2716,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":2724,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":3256,"parent_record_offset":3200,"ancestor_record_offset":2724,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":765,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2724,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":3228,"parent_record_offset":3200,"ancestor_record_offset":2724,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2724,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2724,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":765,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":2724,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":765,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2724,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":3104,"parent_record_offset":3072,"ancestor_record_offset":2724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":765,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2724,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::copy_file_to_backup
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":4052,"rva":13200000,"va":5381909120}
// 编译实例 RVAs: 0xc96a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t735089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e871370b00","size":5,"soft_fail":false,"target_va":5382644528,"va":5381909434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6827959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e849d097ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381909458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6393273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e847729eff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381909300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t918482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e8d2030e00","size":5,"soft_fail":false,"target_va":5382827728,"va":5381909241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10878926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e8ceffa500","size":5,"soft_fail":false,"target_va":5392788160,"va":5381909229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141942280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13284958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e85eb6ca00","size":5,"soft_fail":false,"target_va":5395194496,"va":5381909533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t869476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e864440d00","size":5,"soft_fail":false,"target_va":5382778992,"va":5381909511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::generate_backup_file_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381909120,"raw_hex":"e8eafdffff","size":5,"soft_fail":false,"target_va":5381908656,"va":5381909185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":765,"record_offset":4272,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":4620,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":765,"record_offset":4444,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":765,"record_offset":4188,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":765,"record_offset":5244,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":12455,"source_line":2523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":765,"record_offset":4304,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":4052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":765,"record_offset":4844,"parent_record_offset":4788,"ancestor_record_offset":4052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":4052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":765,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":4052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":765,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":4052,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":765,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":4052,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":765,"record_offset":4528,"parent_record_offset":4468,"ancestor_record_offset":4052,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":765,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":765,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":4052,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":765,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":4052,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":765,"record_offset":4556,"parent_record_offset":4468,"ancestor_record_offset":4052,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":765,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":4052,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":765,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":4052,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":765,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":4052,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":765,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":4052,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":765,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":4052,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":765,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":4052,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":765,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":4052,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":765,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":4052,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00765.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::copy_file_to_backup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1615C
// 首条位置: {"module":2082,"inlinee":90460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2082,"record_offset":496,"parent_record_offset":476,"ancestor_record_offset":476,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02082.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2082,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":476,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02082.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2082,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":476,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02082.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::impl$0::tail
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1678,"record_offset":252,"rva":24079040,"va":5392788160}
// 编译实例 RVAs: 0x16f6ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..path..Components$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10054679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e8e99366ff","size":5,"soft_fail":false,"target_va":5382733760,"va":5392788434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..path..PathBuf$u20$as$u20$core..iter..traits..collect..FromIterator$LT$P$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10309269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e8954e9d00","size":5,"soft_fail":false,"target_va":5403097984,"va":5392788710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17706797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e8d3d0f1fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392788280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17707358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e8a2cef1fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392788857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10309182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e83e4e9d00","size":5,"soft_fail":false,"target_va":5403097824,"va":5392788637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18752174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e8ae221e01","size":5,"soft_fail":false,"target_va":5411540963,"va":5392788784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10043240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788160,"raw_hex":"e898c066ff","size":5,"soft_fail":false,"target_va":5382745088,"va":5392788323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":1492,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":844,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":396,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":13374,"source_line":576,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1678,"record_offset":816,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1678,"record_offset":1620,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1678,"record_offset":1676,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1678,"record_offset":788,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":51565,"source_line":2866,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] std::path::PathBuf::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1678,"record_offset":1648,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7542,"source_line":1190,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$17::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1678,"record_offset":1368,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":51567,"source_line":865,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1678,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":252,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1678,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":252,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1678,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1678,"record_offset":684,"parent_record_offset":512,"ancestor_record_offset":252,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1678,"record_offset":1840,"parent_record_offset":1748,"ancestor_record_offset":252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":1240,"parent_record_offset":1200,"ancestor_record_offset":252,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":1200,"parent_record_offset":844,"ancestor_record_offset":252,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":1068,"parent_record_offset":900,"ancestor_record_offset":252,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":900,"parent_record_offset":844,"ancestor_record_offset":252,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":424,"parent_record_offset":396,"ancestor_record_offset":252,"inlinee":13375,"source_line":632,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1678,"record_offset":936,"parent_record_offset":900,"ancestor_record_offset":252,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1004,"parent_record_offset":964,"ancestor_record_offset":252,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1108,"parent_record_offset":1068,"ancestor_record_offset":252,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":452,"parent_record_offset":424,"ancestor_record_offset":252,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":252,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":252,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":252,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":252,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":252,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":512,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":252,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":252,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1678,"record_offset":544,"parent_record_offset":512,"ancestor_record_offset":252,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1678,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":252,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1678,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":252,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1678,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":252,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1678,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":252,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1678,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":252,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1678,"record_offset":964,"parent_record_offset":936,"ancestor_record_offset":252,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1678,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":252,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1678,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":252,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1678,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01678.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1678,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":51569,"source_line":544,"archive":"_pdb/B/modules/01678.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::file::copy_file_to_backup
// 信息 ID: 5216f74d5fca037fe9176218
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x146f678 -> 0xc96a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::copy_file_to_backup::closure$0
// 信息 ID: 0316f5ca5ade1cfd2fbc6aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::file::copy_file_to_backup", "captures": "unknown"}
// [闭包环境类型引用] utils::file::copy_file_to_backup::closure_env$0 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::copy_file_to_backup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::copy_file_to_backup::closure_env$0>; module=2082, inline_record=476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::date_time_string_for_filename
// 信息 ID: 745759eb682dbf28990b5503
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbc5b -> 0xc96690 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_freecam_save::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x141d0d -> 0xc96690 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x31b38e -> 0xc96690 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x3b00f5 -> 0xc96690 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::generate_backup_file_path @ 0xc968dd -> 0xc96690 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc41bd -> 0xc96690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::generate_backup_file_path
// 信息 ID: 5a0aede4dfaf00fe6771fcb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xc96ac1 -> 0xc968b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::impl$0::tail
// 信息 ID: e1a415de89f4f32b4124ad35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x8f7be2 -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xc96aed -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xc96b11 -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc46cb -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc46f2 -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x17a7117 -> 0x16f6ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
