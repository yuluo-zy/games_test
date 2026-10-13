// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/vk-sync-rs/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a9fab48f91ed16188b819d61ee746379ca4eba220f7d8473f811453f775bf2f5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 105, 219, 533, 700, 720
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: [{"name":"vk-sync-fork","source":"local_or_path","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::impl$1::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":219,"record_offset":57816,"rva":5454336,"va":5374163456}
// 编译实例 RVAs: 0x533a00, 0x9dc5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t8728937","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374163456,"raw_hex":"e969318500","size":5,"soft_fail":false,"target_va":5382892432,"va":5374163490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EBA
// 首条位置: {"module":105,"inlinee":11962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":260,"rva":12650432,"va":5381359552}
// 编译实例 RVAs: 0xc107c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381359552,"raw_hex":"e8b0050000","size":5,"soft_fail":false,"target_va":5381361104,"va":5381359643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":720,"record_offset":752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":40336,"source_line":422,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":39096,"source_line":45,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":720,"record_offset":464,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":40334,"source_line":1476,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":39098,"source_line":39,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":720,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":720,"record_offset":428,"parent_record_offset":388,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":600,"parent_record_offset":492,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_buffer_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 416
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":784,"rva":12650816,"va":5381359936}
// 编译实例 RVAs: 0xc10940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381359936,"raw_hex":"e810040000","size":5,"soft_fail":false,"target_va":5381361104,"va":5381360059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":720,"record_offset":1284,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":40336,"source_line":422,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1220,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":39096,"source_line":45,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":920,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":720,"record_offset":996,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":40334,"source_line":1476,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":784,"inlinee":39098,"source_line":39,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":720,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":784,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":720,"record_offset":960,"parent_record_offset":920,"ancestor_record_offset":784,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":784,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":1132,"parent_record_offset":1024,"ancestor_record_offset":784,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_image_memory_barrier
// 记录形式: compiled_procedure; instances=1
// 源码行: 529
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":1316,"rva":12651248,"va":5381360368}
// 编译实例 RVAs: 0xc10af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30180998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381360368,"raw_hex":"e88686cc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381361061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381360368,"raw_hex":"e86e020000","size":5,"soft_fail":false,"target_va":5381361104,"va":5381360477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$269::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1776,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":40344,"source_line":45,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$669::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":720,"record_offset":1860,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":40336,"source_line":422,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$677::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1720,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":39096,"source_line":45,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":720,"record_offset":1460,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::AccessType::is_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":720,"record_offset":1564,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":40334,"source_line":1476,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":720,"record_offset":1832,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":11962,"source_line":21,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$268::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":1316,"inlinee":40346,"source_line":39,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$676::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":720,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1316,"inlinee":39098,"source_line":39,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":720,"record_offset":1520,"parent_record_offset":1460,"ancestor_record_offset":1316,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::get_access_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 713
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":1892,"rva":12651984,"va":5381361104}
// 编译实例 RVAs: 0xc10dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: vk_sync_fork::AccessType::is_write
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 1476, 1477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9D8E
// 首条位置: {"module":720,"inlinee":40334}
// 编译实例 RVAs: 0xc10e10
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
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >

// ------------------------------------------------------------------
// 函数: vk_sync_fork::AccessType::is_write
// 信息 ID: 9e9f207736fe00a9b9733291
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 47 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155439 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x15563d -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1557d7 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1556e0 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155db5 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155fd2 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1561b4 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1560b5 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2af785 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2af9a2 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afb84 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afa85 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x363de5 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x364002 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x3641e4 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x3640e5 -> 0xc10e10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] vk_sync_fork::get_memory_barrier; module=720, inline_record=464; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_buffer_memory_barrier; module=720, inline_record=996; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_image_memory_barrier; module=720, inline_record=1564; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_access_info
// 信息 ID: 681110b9de721b85897ce1d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5307ee -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports @ 0xbd45d6 -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0xc1081b -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0xc10868 -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_memory_barrier @ 0xc108cb -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0xc109bb -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0xc10a09 -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_buffer_memory_barrier @ 0xc10a6b -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0xc10b5d -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0xc10c4d -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0xc10bdf -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::get_image_memory_barrier @ 0xc10cd9 -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > > @ 0x18b479b -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding @ 0x205f2fe -> 0xc10dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_buffer_memory_barrier
// 信息 ID: 2813d8cd7ec6baba2b9dc757
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::barrier::impl$5::buffer_barrier @ 0xbd6891 -> 0xc10940 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x18aff56 -> 0xc10940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_image_memory_barrier
// 信息 ID: 3ebbd786ad06977c63f491ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::barrier::impl$5::image_barrier @ 0xbd6722 -> 0xc10af0 (direct_call); 同地址别名未裁定。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x18b003e -> 0xc10af0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::get_memory_barrier
// 信息 ID: 54d0bc1086d5779892c641d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] vk_sync_fork::cmd::pipeline_barrier @ 0x18afea6 -> 0xc107c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::impl$1::fmt
// 信息 ID: 8186f32e35f98371aaa35461
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$74::fmt<vk_sync_fork::AccessType> @ 0x5338f3 -> 0x533a00 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: vk_sync_fork::impl$5::eq
// 信息 ID: 185cc2cf89340d958a6d8da2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=19816; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=219, inline_record=61124; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=533, inline_record=17212; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with; module=700, inline_record=1072; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with; module=700, inline_record=1124; 不是独立调用边。
// [PDB 内联宿主] vk_sync_fork::get_image_memory_barrier; module=720, inline_record=1832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
