// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/paths/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 604de0c876e6675a9e8ec58ba81c4234e70b074cd621e84894bb3fe2e6b8b1c1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 637, 1834, 3260
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use known_folders; // namespace candidate
//   package identities: [{"name":"known-folders","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: paths::config
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":252,"rva":12180272,"va":5380889392}
// 编译实例 RVAs: 0xb9db30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889392,"raw_hex":"e8b3b7d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":356,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":416,"rva":12180320,"va":5380889440}
// 编译实例 RVAs: 0xb9db60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889440,"raw_hex":"e888b7d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":516,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":540,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::backup_saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":572,"rva":12180368,"va":5380889488}
// 编译实例 RVAs: 0xb9db90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889488,"raw_hex":"e853b7d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":680,"parent_record_offset":572,"ancestor_record_offset":572,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":704,"parent_record_offset":572,"ancestor_record_offset":572,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::tmp
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":740,"rva":12180416,"va":5380889536}
// 编译实例 RVAs: 0xb9dbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889536,"raw_hex":"e823b7d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":840,"parent_record_offset":740,"ancestor_record_offset":740,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":864,"parent_record_offset":740,"ancestor_record_offset":740,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::deleted_saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":900,"rva":12180464,"va":5380889584}
// 编译实例 RVAs: 0xb9dbf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889584,"raw_hex":"e8f3b6d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1008,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1032,"parent_record_offset":900,"ancestor_record_offset":900,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::installed
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":1068,"rva":12180512,"va":5380889632}
// 编译实例 RVAs: 0xb9dc20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889632,"raw_hex":"e8c3b6d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1172,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1196,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":1232,"rva":12180560,"va":5380889680}
// 编译实例 RVAs: 0xb9dc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30652051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889680,"raw_hex":"e893b6d301","size":5,"soft_fail":false,"target_va":5411541776,"va":5380889720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1352,"parent_record_offset":1232,"ancestor_record_offset":1232,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":1376,"parent_record_offset":1232,"ancestor_record_offset":1232,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::find_standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":1412,"rva":12180608,"va":5380889728}
// 编译实例 RVAs: 0xb9dc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5808412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e8e45ea7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380889911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1937785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e879911d00","size":5,"soft_fail":false,"target_va":5382827728,"va":5380889938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6c7408209ab167bd (.llvm.14641344035423613259)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30826657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e8a160d601","size":5,"soft_fail":false,"target_va":5411716800,"va":5380890138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] known_folders::win::get_known_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"known-folders","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13287910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e8e6c1ca00","size":5,"soft_fail":false,"target_va":5394177680,"va":5380889765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1804603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e83b891b00","size":5,"soft_fail":false,"target_va":5382694656,"va":5380890048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1809509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e8659c1b00","size":5,"soft_fail":false,"target_va":5382699312,"va":5380889798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1854191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889728,"raw_hex":"e8ef4a1c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380889884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":637,"record_offset":1584,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":2044,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":637,"record_offset":1988,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":637,"record_offset":2756,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":637,"record_offset":1556,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":17230,"source_line":217,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":2016,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":2272,"parent_record_offset":1412,"ancestor_record_offset":1412,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1412,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1412,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1412,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1412,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":1412,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":637,"record_offset":1668,"parent_record_offset":1624,"ancestor_record_offset":1412,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":637,"record_offset":1624,"parent_record_offset":1584,"ancestor_record_offset":1412,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":1700,"parent_record_offset":1624,"ancestor_record_offset":1412,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1412,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":1412,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":1412,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1412,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1834,"record_offset":252,"rva":25469088,"va":5394178208}
// 编译实例 RVAs: 0x184a0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19096893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e8c39adcfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394178376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19096828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e8049bdcfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394178327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11437623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e8c97951ff","size":5,"soft_fail":false,"target_va":5382740736,"va":5394178354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11350516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e80cce52ff","size":5,"soft_fail":false,"target_va":5382827728,"va":5394178239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142131410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9335583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e81f738e00","size":5,"soft_fail":false,"target_va":5403513872,"va":5394178284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17362547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e873ee0801","size":5,"soft_fail":false,"target_va":5411540992,"va":5394178440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11468693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394178208,"raw_hex":"e86b0051ff","size":5,"soft_fail":false,"target_va":5382709584,"va":5394178272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1834,"record_offset":1032,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1834,"record_offset":680,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1834,"record_offset":652,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1834,"record_offset":568,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1834,"record_offset":1536,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1834,"record_offset":1004,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1834,"record_offset":400,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":252,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":252,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":1068,"parent_record_offset":1032,"ancestor_record_offset":252,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":1100,"parent_record_offset":1068,"ancestor_record_offset":252,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1834,"record_offset":916,"parent_record_offset":852,"ancestor_record_offset":252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1834,"record_offset":884,"parent_record_offset":852,"ancestor_record_offset":252,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1834,"record_offset":852,"parent_record_offset":824,"ancestor_record_offset":252,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1834,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":252,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1834,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1834,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":252,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1834,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":252,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1834,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":252,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1834,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":252,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1834,"record_offset":620,"parent_record_offset":568,"ancestor_record_offset":252,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1834,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":252,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1834,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":252,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1834,"record_offset":424,"parent_record_offset":400,"ancestor_record_offset":252,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01834.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B925
// 首条位置: {"module":3260,"inlinee":112933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3260,"record_offset":508,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03260.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3260,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":488,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03260.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3260,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":488,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03260.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::initialize_paths
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":637,"record_offset":3328,"rva":12181264,"va":5380890384}
// 编译实例 RVAs: 0xb9df10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5992124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e84491a4ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5380892855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5810336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e86057a7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380891819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5809011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e88d5ca7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380890510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1850290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8b23b1c00","size":5,"soft_fail":false,"target_va":5382740736,"va":5380890441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=9
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1937274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e87a8f1d00","size":5,"soft_fail":false,"target_va":5382827728,"va":5380890449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14184a0a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13287323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e89bbfca00","size":5,"soft_fail":false,"target_va":5394178208,"va":5380890880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30648475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e89ba8d301","size":5,"soft_fail":false,"target_va":5411540992,"va":5380892512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1949503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e83fbf1d00","size":5,"soft_fail":false,"target_va":5382840208,"va":5380890700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h4af4b2db7932f791 (.llvm.14641344035423613259)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30824882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8b259d601","size":5,"soft_fail":false,"target_va":5411716576,"va":5380891689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30649282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8c2abd301","size":5,"soft_fail":false,"target_va":5411541904,"va":5380892617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8dffcffff","size":5,"soft_fail":false,"target_va":5380889728,"va":5380890524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1803062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e836831b00","size":5,"soft_fail":false,"target_va":5382694656,"va":5380891589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1807033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8b9921b00","size":5,"soft_fail":false,"target_va":5382698576,"va":5380891538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1827757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8ade31b00","size":5,"soft_fail":false,"target_va":5382719888,"va":5380892126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t1853595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e89b481c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380890480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1936818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380890384,"raw_hex":"e8b28d1d00","size":5,"soft_fail":false,"target_va":5382828864,"va":5380892041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":637,"record_offset":6544,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":637,"record_offset":4148,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":637,"record_offset":5760,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":637,"record_offset":8624,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":637,"record_offset":7888,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":637,"record_offset":4232,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":8512,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":8540,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":3688,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":637,"record_offset":3976,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":3656,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":7832,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":18557,"source_line":2278,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":3516,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":3488,"parent_record_offset":3328,"ancestor_record_offset":3328,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":3328,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":3328,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3328,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":6580,"parent_record_offset":6544,"ancestor_record_offset":3328,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":3328,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":637,"record_offset":3888,"parent_record_offset":3832,"ancestor_record_offset":3328,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":637,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3328,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":3328,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":3328,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":637,"record_offset":3808,"parent_record_offset":3788,"ancestor_record_offset":3328,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":637,"record_offset":5844,"parent_record_offset":5804,"ancestor_record_offset":3328,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":637,"record_offset":5804,"parent_record_offset":5760,"ancestor_record_offset":3328,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":637,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":3328,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":637,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3328,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":3708,"parent_record_offset":3688,"ancestor_record_offset":3328,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":3328,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":637,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":3328,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":3328,"inlinee":6016,"source_line":906,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3328,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":3328,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":637,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3328,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00637.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":637,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":3328,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00637.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: paths::backup_saves
// 信息 ID: a351266e1413a08da5645030
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::SessionId::backup_folder_path @ 0xb461a3 -> 0xb9db90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::config
// 信息 ID: 3f00344661fa655624573d93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x31b34d -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x3539b6 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_game_config_to_disk @ 0x354263 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_settings_file @ 0x354d91 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_settings_file @ 0x3ae076 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::profile_file_path @ 0x916448 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0x9164a1 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0x917383 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0x9175b0 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::photomode_buttons::photomode_buttons @ 0x234cec2 -> 0xb9db30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::deleted_saves
// 信息 ID: 6ce874e4fe25d5fbde4b2f0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::delete_session @ 0xb46e4f -> 0xb9dbf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::delete_session @ 0xb46fcd -> 0xb9dbf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path::closure$0
// 信息 ID: a342050fb8415683424d6020
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "paths::ensure_path", "captures": "unknown"}
// [闭包环境类型引用] paths::ensure_path::closure_env$0 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; module=3260, inline_record=488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path<std::path::PathBuf>
// 信息 ID: 3e38e6a7c57b9089c217e080
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] paths::ensure_path -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] paths::initialize_paths @ 0xb9e100 -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9e17f -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9e1fe -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9e284 -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9e30a -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9e360 -> 0x184a0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::find_standard_app_root_path
// 信息 ID: f5352dfd66c251be524e6293
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_last_used_locale @ 0xb9d19e -> 0xb9dc80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xb9df9c -> 0xb9dc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::initialize_paths
// 信息 ID: 523da16a8b8daa2f1f124ce6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dc894 -> 0xb9df10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::installed
// 信息 ID: 6c21ee1256043464236682c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x353a81 -> 0xb9dc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::update_vulkan_allowlist_defaults @ 0x354677 -> 0xb9dc20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::saves
// 信息 ID: 6196cbe70d9ee0bc8a8fd88d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::SessionId::save_folder_path @ 0xb46023 -> 0xb9db60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::refresh_session_list @ 0xb46b3f -> 0xb9db60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::standard_app_root_path
// 信息 ID: 8db82f120d9ae75b9b8916ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x3d107b -> 0xb9dc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale @ 0xb9c32e -> 0xb9dc50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::tmp
// 信息 ID: f8aa1393a7b232385839a798
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::aftermath::impl$0::default @ 0x244480 -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::aftermath::impl$1::dumped @ 0x244f6e -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::logging::set_up_logging @ 0x2e4c6d -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_log_file @ 0x3acca7 -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file @ 0x3ae5a7 -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1ca41 -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x1e1ca84 -> 0xb9dbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
