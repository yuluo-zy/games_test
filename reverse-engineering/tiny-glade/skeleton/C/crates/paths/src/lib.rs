// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/paths/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b8b93cb0ebdbe18ea43e706773f5fc91affac98bd5221a0933495b7d3ca565be
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 721, 2110, 3601
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
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
// 首条位置: {"module":721,"record_offset":9504,"rva":14430448,"va":5383139568}
// 编译实例 RVAs: 0xdc30f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33481987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383139568,"raw_hex":"e803e5fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383139608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9608,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9632,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
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
// 首条位置: {"module":721,"record_offset":9348,"rva":14430400,"va":5383139520}
// 编译实例 RVAs: 0xdc30c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33482040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383139520,"raw_hex":"e838e5fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383139555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9448,"parent_record_offset":9348,"ancestor_record_offset":9348,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9472,"parent_record_offset":9348,"ancestor_record_offset":9348,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::beta_saves
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":721,"record_offset":260,"rva":14426816,"va":5383135936}
// 编译实例 RVAs: 0xdc22c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33485619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383135936,"raw_hex":"e833f3fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383135976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":368,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":392,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
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
// 首条位置: {"module":721,"record_offset":428,"rva":14426864,"va":5383135984}
// 编译实例 RVAs: 0xdc22f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33485571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383135984,"raw_hex":"e803f3fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383136024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":536,"parent_record_offset":428,"ancestor_record_offset":428,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":560,"parent_record_offset":428,"ancestor_record_offset":428,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
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
// 首条位置: {"module":721,"record_offset":9188,"rva":14430352,"va":5383139472}
// 编译实例 RVAs: 0xdc3090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33482083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383139472,"raw_hex":"e863e5fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383139512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9288,"parent_record_offset":9188,"ancestor_record_offset":9188,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9312,"parent_record_offset":9188,"ancestor_record_offset":9188,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
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
// 首条位置: {"module":721,"record_offset":596,"rva":14426912,"va":5383136032}
// 编译实例 RVAs: 0xdc2320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33485523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136032,"raw_hex":"e8d3f2fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383136072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":704,"parent_record_offset":596,"ancestor_record_offset":596,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":728,"parent_record_offset":596,"ancestor_record_offset":596,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::installed
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":721,"record_offset":9668,"rva":14430496,"va":5383139616}
// 编译实例 RVAs: 0xdc3120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33481939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383139616,"raw_hex":"e8d3e4fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383139656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9772,"parent_record_offset":9668,"ancestor_record_offset":9668,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":9796,"parent_record_offset":9668,"ancestor_record_offset":9668,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":721,"record_offset":6956,"rva":14429664,"va":5383138784}
// 编译实例 RVAs: 0xdc2de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33482771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138784,"raw_hex":"e813e8fe01","size":5,"soft_fail":false,"target_va":5416621600,"va":5383138824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":7076,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":7100,"parent_record_offset":6956,"ancestor_record_offset":6956,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::find_standard_app_root_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":721,"record_offset":7136,"rva":14429712,"va":5383138832}
// 编译实例 RVAs: 0xdc2e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1520122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e8fa311700","size":5,"soft_fail":false,"target_va":5384659264,"va":5383139137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1517252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e8c4261700","size":5,"soft_fail":false,"target_va":5384656240,"va":5383138983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-7460309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e82b2a8eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383139008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hca161b97ec51bae5 (.llvm.11696610635359222835)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33714309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e885700202","size":5,"soft_fail":false,"target_va":5416853536,"va":5383139222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] known_folders::win::get_known_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"known-folders","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16863769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e819520101","size":5,"soft_fail":false,"target_va":5400002640,"va":5383138866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1573239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383138832,"raw_hex":"e877011800","size":5,"soft_fail":false,"target_va":5384712144,"va":5383138900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":721,"record_offset":7304,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":721,"record_offset":7820,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":721,"record_offset":7764,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":721,"record_offset":8504,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":721,"record_offset":7276,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":10542,"source_line":222,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":721,"record_offset":7792,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":7136,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":7136,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":7136,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":7136,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":7944,"parent_record_offset":7916,"ancestor_record_offset":7136,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7136,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7136,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":721,"record_offset":7388,"parent_record_offset":7344,"ancestor_record_offset":7136,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":721,"record_offset":7344,"parent_record_offset":7304,"ancestor_record_offset":7136,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":721,"record_offset":7420,"parent_record_offset":7344,"ancestor_record_offset":7136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2110,"record_offset":260,"rva":31292032,"va":5400001152}
// 编译实例 RVAs: 0x1dd7a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15311602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e80e5d16ff","size":5,"soft_fail":false,"target_va":5384689616,"va":5400001213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15347574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e88ad015ff","size":5,"soft_fail":false,"target_va":5384653728,"va":5400001297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24322635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e8b5dd8cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400001318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24322571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e8f5dd8cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5400001270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31279116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e8f4b722fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400001303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142707350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9631873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e881f89200","size":5,"soft_fail":false,"target_va":5409633104,"va":5400001226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16619390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400001152,"raw_hex":"e87e97fd00","size":5,"soft_fail":false,"target_va":5416620769,"va":5400001374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2110,"record_offset":920,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2110,"record_offset":512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2110,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2110,"record_offset":400,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2110,"record_offset":892,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":992,"parent_record_offset":960,"ancestor_record_offset":260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2110,"record_offset":748,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2110,"record_offset":960,"parent_record_offset":920,"ancestor_record_offset":260,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2110,"record_offset":716,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2110,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2110,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2110,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2110,"record_offset":452,"parent_record_offset":400,"ancestor_record_offset":260,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2110,"record_offset":424,"parent_record_offset":400,"ancestor_record_offset":260,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/02110.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::ensure_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F8F9
// 首条位置: {"module":3601,"inlinee":129273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3601,"record_offset":516,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03601.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3601,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":496,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03601.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3601,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":496,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: paths::initialize_paths
// 记录形式: compiled_procedure; instances=1
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":721,"record_offset":764,"rva":14426960,"va":5383136080}
// 编译实例 RVAs: 0xdc2350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1654754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8e23f1900","size":5,"soft_fail":false,"target_va":5384792224,"va":5383137465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1522167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8f7391700","size":5,"soft_fail":false,"target_va":5384659264,"va":5383137092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1520055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8b7311700","size":5,"soft_fail":false,"target_va":5384656240,"va":5383136180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1517589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e815281700","size":5,"soft_fail":false,"target_va":5384653728,"va":5383136134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7458668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e894308eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383137351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-7457512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e818358eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383136211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14415149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8d30a24ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383137336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13799291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e885702dff","size":5,"soft_fail":false,"target_va":5369338848,"va":5383138134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dd7a80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16864637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e87d550101","size":5,"soft_fail":false,"target_va":5400001152,"va":5383136510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33482934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8b6e8fe01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383137830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1658525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e89d4e1900","size":5,"soft_fail":false,"target_va":5384794864,"va":5383136334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6aebf40f8a28dcaf (.llvm.11696610635359222835)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33716120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e898770202","size":5,"soft_fail":false,"target_va":5416853312,"va":5383137187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33484346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e83aeefe01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383137889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8260a0000","size":5,"soft_fail":false,"target_va":5383138832,"va":5383136229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1574326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e8b6051800","size":5,"soft_fail":false,"target_va":5384711376,"va":5383137045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1583659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383136080,"raw_hex":"e82b2a1800","size":5,"soft_fail":false,"target_va":5384721168,"va":5383137504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":721,"record_offset":3268,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":721,"record_offset":1340,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":721,"record_offset":2364,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":721,"record_offset":1424,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":5196,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":5224,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":11351,"source_line":1943,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":721,"record_offset":968,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":721,"record_offset":1308,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":721,"record_offset":940,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":721,"record_offset":4404,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":21764,"source_line":2480,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":721,"record_offset":912,"parent_record_offset":764,"ancestor_record_offset":764,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":764,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":764,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":764,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":764,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":764,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":764,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":764,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":721,"record_offset":1164,"parent_record_offset":1112,"ancestor_record_offset":764,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":721,"record_offset":3312,"parent_record_offset":3268,"ancestor_record_offset":764,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":721,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":764,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":764,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":764,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":721,"record_offset":1088,"parent_record_offset":1068,"ancestor_record_offset":764,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":721,"record_offset":2444,"parent_record_offset":2404,"ancestor_record_offset":764,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":721,"record_offset":2404,"parent_record_offset":2364,"ancestor_record_offset":764,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":721,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":764,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":721,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":764,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":721,"record_offset":988,"parent_record_offset":968,"ancestor_record_offset":764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":721,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":764,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00721.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":721,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":764,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00721.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>

// ------------------------------------------------------------------
// 函数: paths::backup_saves
// 信息 ID: 4bc142ca4a0e642ce2862c28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::SessionId::backup_folder_path @ 0xb05f23 -> 0xdc22f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::beta_saves
// 信息 ID: 4d2a6af58d1f9e20be143026
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0xa0addf -> 0xdc22c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::import::ImportMenu::read @ 0xa817d3 -> 0xdc22c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::import::source_root @ 0xa82ee0 -> 0xdc22c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::import::source_folder @ 0xa82f13 -> 0xdc22c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::config
// 信息 ID: ad9d76842328c6b9fbde1a2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_settings_file @ 0x198f16 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x47a4d6 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_settings_file @ 0x47ab51 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::write_game_config_to_disk @ 0x47acc3 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::gather_screenshot_requests @ 0x4ba9de -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::start_recording @ 0x5afcd1 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68e60f -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::load_game_profile @ 0xa28340 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::profile_file_path @ 0xa28be8 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0xa28c40 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::game_profile::write_profile_via_tmp @ 0xa28ea7 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::shelve_mod @ 0xd8f14d -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::discover_mods @ 0xd8fc03 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::rediscover_mods @ 0xd93eb4 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::partless_mod_folders @ 0xd94845 -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::mods_root @ 0xd961bc -> 0xdc30f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::deleted_saves
// 信息 ID: 0547fb6e22c8b0eca88d1e20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::delete_session @ 0xb051fb -> 0xdc2320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::delete_session @ 0xb05391 -> 0xdc2320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path::closure$0
// 信息 ID: a0a787c72e326883f378d938
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "paths::ensure_path", "captures": "unknown"}
// [闭包环境类型引用] paths::ensure_path::closure_env$0 <- anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,std::io::error::Error,alloc::string::String,paths::ensure_path::closure_env$0<std::path::PathBuf> >; module=3601, inline_record=496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::ensure_path<std::path::PathBuf>
// 信息 ID: daeca7f86df9b218fc32acb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] paths::ensure_path -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] paths::initialize_paths @ 0xdc24fe -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc2563 -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc25c8 -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc2631 -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc269a -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc26e8 -> 0x1dd7a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::find_standard_app_root_path
// 信息 ID: 3355d5e3d6c1cdb4afa34c83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x18529c -> 0xdc2e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::main @ 0x64154d -> 0xdc2e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_last_used_locale @ 0xdc1bee -> 0xdc2e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] paths::initialize_paths @ 0xdc23e5 -> 0xdc2e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::cache_root @ 0x1d3eb40 -> 0xdc2e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::initialize_paths
// 信息 ID: 990d9fcd2fc4700c967d60a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x641169 -> 0xdc2350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::installed
// 信息 ID: 624d961c06e991ec759968a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_app_config @ 0x47a593 -> 0xdc3120 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_config::update_vulkan_allowlist_defaults @ 0x47b067 -> 0xdc3120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::saves
// 信息 ID: 285be4046f38fcd58b93735f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e91223b766d36a8ae6a7788f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::preactivate_boot_replacement_mods @ 0x908db7 -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0xa0adc9 -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::refresh_session_list @ 0xb05a29 -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::load_saved_session_list @ 0xb05acf -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::session::SessionId::save_folder_path @ 0xb05d53 -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)> @ 0x17cd13f -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)> > @ 0x1941e9f -> 0xdc30c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::standard_app_root_path
// 信息 ID: 11d8086f08af108fa01f608f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x15ae73 -> 0xdc2de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::discover_mods @ 0xd8fe06 -> 0xdc2de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd91938 -> 0xdc2de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd91d2e -> 0xdc2de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale @ 0xdc0fd8 -> 0xdc2de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: paths::tmp
// 信息 ID: 37f75e06691a7bee3c1d9979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_freecam_save::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x189011 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_freecam_save::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x1895a2 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_camera_snapshot_ui::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x18ae58 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_camera_snapshot_ui::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x18af33 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_camera_snapshot_ui::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x18b2ea -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_camera_snapshot_ui::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x18b3c5 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_log_file @ 0x198aba -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file @ 0x1991aa -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::aftermath::impl$0::default @ 0x199ec0 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::aftermath::impl$1::dumped @ 0x19aadb -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::main @ 0x64308b -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68dea6 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68e25d -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::author_preview_path @ 0xb003d7 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xc11e68 -> 0xdc3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::debug::sheep_controls::sheep_controls::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xc1204d -> 0xdc3090 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
