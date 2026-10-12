// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/file.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 33796feed13109b745be5cb113114be2fc5cdf5fe1c1d353dd3bf2caeb320632
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 760, 1791, 2183
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
// 首条位置: {"module":760,"record_offset":5912,"rva":14590944,"va":5383300064}
// 编译实例 RVAs: 0xdea3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-7621584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e830b48bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383300283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13961746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e8eef52aff","size":5,"soft_fail":false,"target_va":5369338848,"va":5383300589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e346c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17080886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e836a20401","size":5,"soft_fail":false,"target_va":5400381120,"va":5383300229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33321267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e83371fc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383300360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::format_description::parse::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17080471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e897a00401","size":5,"soft_fail":false,"target_va":5400380640,"va":5383300164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t86764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e8ec520100","size":5,"soft_fail":false,"target_va":5383386864,"va":5383300095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t86838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383300064,"raw_hex":"e836530100","size":5,"soft_fail":false,"target_va":5383386976,"va":5383300133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":6592,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":760,"record_offset":6140,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":760,"record_offset":6496,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] utils::local_time::now_appox_local
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":760,"record_offset":6052,"parent_record_offset":5912,"ancestor_record_offset":5912,"inlinee":48917,"source_line":20,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":5912,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":5912,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":6428,"parent_record_offset":6404,"ancestor_record_offset":5912,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":6356,"parent_record_offset":6300,"ancestor_record_offset":5912,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":6328,"parent_record_offset":6300,"ancestor_record_offset":5912,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":5912,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5912,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":760,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":5912,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":760,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5912,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":6180,"parent_record_offset":6140,"ancestor_record_offset":5912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] utils::local_time::now_appox_local::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/local_time.rs
//     evidence={"module":760,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":5912,"inlinee":48912,"source_line":22,"archive":"_pdb/C/modules/00760.jsonl.gz"}
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
// 首条位置: {"module":760,"record_offset":4316,"rva":14590512,"va":5383299632}
// 编译实例 RVAs: 0xdea230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_with_file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1356224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e8c0b11400","size":5,"soft_fail":false,"target_va":5384656064,"va":5383299835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1357330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e812b61400","size":5,"soft_fail":false,"target_va":5384657024,"va":5383299689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-7621161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e8d7b58bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383299860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1495064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e818d01600","size":5,"soft_fail":false,"target_va":5384794864,"va":5383299795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33321606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e88672fc01","size":5,"soft_fail":false,"target_va":5416621536,"va":5383299925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1530361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e8f9591700","size":5,"soft_fail":false,"target_va":5384830080,"va":5383299714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383299632,"raw_hex":"e87e010000","size":5,"soft_fail":false,"target_va":5383300064,"va":5383299677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":760,"record_offset":4596,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":760,"record_offset":4680,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":760,"record_offset":4460,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":760,"record_offset":5596,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":5312,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_file_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":760,"record_offset":4708,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":49065,"source_line":3038,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":4892,"parent_record_offset":4836,"ancestor_record_offset":4316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":760,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4316,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":4864,"parent_record_offset":4836,"ancestor_record_offset":4316,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":4316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":760,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":4316,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":760,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4316,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":4740,"parent_record_offset":4708,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":760,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":4316,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":760,"record_offset":4504,"parent_record_offset":4484,"ancestor_record_offset":4316,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":760,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":4316,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00760.jsonl.gz"}
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
// 首条位置: {"module":760,"record_offset":1664,"rva":14589824,"va":5383298944}
// 编译实例 RVAs: 0xde9f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1310173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e8ddfd1300","size":5,"soft_fail":false,"target_va":5384609408,"va":5383299230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-7620556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e834b88bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383299255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b48210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14017059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e823e2d500","size":5,"soft_fail":false,"target_va":5397316112,"va":5383299048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e32e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17075453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e8fd8c0401","size":5,"soft_fail":false,"target_va":5400374784,"va":5383299326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1424414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e81ebc1500","size":5,"soft_fail":false,"target_va":5384723728,"va":5383299309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::generate_backup_file_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383298944,"raw_hex":"e86f020000","size":5,"soft_fail":false,"target_va":5383299632,"va":5383299004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":760,"record_offset":1880,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":2048,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":760,"record_offset":1936,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":760,"record_offset":1796,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":760,"record_offset":2784,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":1664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":760,"record_offset":2272,"parent_record_offset":2216,"ancestor_record_offset":1664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":2244,"parent_record_offset":2216,"ancestor_record_offset":1664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":760,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":760,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":1664,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":760,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1664,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":760,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":760,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1664,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":760,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":1664,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":760,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1664,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":760,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1664,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":760,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1664,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00760.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::file::copy_file_to_backup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18231
// 首条位置: {"module":2183,"inlinee":98865}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2183,"record_offset":504,"parent_record_offset":484,"ancestor_record_offset":484,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2183,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":484,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02183.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2183,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":484,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02183.jsonl.gz"}
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
// 首条位置: {"module":1791,"record_offset":260,"rva":28606992,"va":5397316112}
// 编译实例 RVAs: 0x1b48210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Components as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12541083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e865a340ff","size":5,"soft_fail":false,"target_va":5384775360,"va":5397316438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12663161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e887c63eff","size":5,"soft_fail":false,"target_va":5384653152,"va":5397316308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..path..PathBuf$u20$as$u20$core..iter..traits..collect..FromIterator$LT$P$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11537593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e8b90cb000","size":5,"soft_fail":false,"target_va":5408854304,"va":5397316706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21637586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e82ed6b5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397316269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21638110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e822d4b5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397316809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28594063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e871b04bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397316250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10690612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e83420a300","size":5,"soft_fail":false,"target_va":5408007248,"va":5397316631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19304586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397316112,"raw_hex":"e88a902601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397316203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1624,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":912,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":404,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1791,"record_offset":884,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1791,"record_offset":1752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1791,"record_offset":1780,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1791,"record_offset":856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":60358,"source_line":3203,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] std::path::PathBuf::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1791,"record_offset":828,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17879,"source_line":1221,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$17::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1791,"record_offset":1500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":60360,"source_line":892,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":696,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1791,"record_offset":1944,"parent_record_offset":1852,"ancestor_record_offset":260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1368,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1328,"parent_record_offset":952,"ancestor_record_offset":260,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1176,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":952,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1008,"parent_record_offset":952,"ancestor_record_offset":260,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":432,"parent_record_offset":404,"ancestor_record_offset":260,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1791,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":260,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":260,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":260,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":260,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":260,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":520,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1791,"record_offset":552,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1791,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":260,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1791,"record_offset":656,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1791,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":260,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1791,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1791,"record_offset":1912,"parent_record_offset":1880,"ancestor_record_offset":260,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1791,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":260,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1791,"record_offset":1560,"parent_record_offset":1528,"ancestor_record_offset":260,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1791,"record_offset":1528,"parent_record_offset":1500,"ancestor_record_offset":260,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1791,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1791,"record_offset":1296,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01791.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Component::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1791,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":260,"inlinee":38918,"source_line":562,"archive":"_pdb/C/modules/01791.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::file::copy_file_to_backup
// 信息 ID: a3b5ab33e6a66ca5289faca0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0xb25a4e -> 0xde9f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::copy_file_to_backup::closure$0
// 信息 ID: d6da52b49186f510282e69d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::file::copy_file_to_backup", "captures": "unknown"}
// [闭包环境类型引用] utils::file::copy_file_to_backup::closure_env$0 <- anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::copy_file_to_backup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<u64,std::io::error::Error,alloc::string::String,utils::file::copy_file_to_backup::closure_env$0>; module=2183, inline_record=484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::date_time_string_for_filename
// 信息 ID: 5419507ce25341930835ed0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x18534f -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_freecam_save::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x18901f -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x4baa25 -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x543181 -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::start_recording @ 0x5afd35 -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::start_recording @ 0x5afeae -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68deb4 -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68e26b -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::generate_backup_file_path @ 0xdea25d -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a0355 -> 0xdea3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::generate_backup_file_path
// 信息 ID: 38dc68be3b7c3b413d68a41e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xde9fbc -> 0xdea230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::file::impl$0::tail
// 信息 ID: c9962500a50d7803cedefd23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0xa0bb34 -> 0x1b48210 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xde9fe8 -> 0x1b48210 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::file::copy_file_to_backup @ 0xdea00c -> 0x1b48210 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a078e -> 0x1b48210 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a07ac -> 0x1b48210 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfd3c2 -> 0x1b48210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
