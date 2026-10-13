// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/managed/resources.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b8f31d3120c47ea4f0905c36652e66d2af0efe643b4d0676325b95c83da3fda2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resources.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 52, 139, 199, 549, 557, 565, 566, 568, 569, 570, 571, 572, 1332, 1367, 1464, 1497, 1591, 1592, 1601, 1617, 1622, 1624, 2824, 3007, 3025
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"PDB_registry_path_only","version":"1.7.1"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76F0
// 首条位置: {"module":568,"inlinee":30448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76E2
// 首条位置: {"module":549,"inlinee":30434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7845
// 首条位置: {"module":566,"inlinee":30789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":566,"record_offset":896,"parent_record_offset":668,"ancestor_record_offset":668,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":784,"parent_record_offset":668,"ancestor_record_offset":668,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":840,"parent_record_offset":668,"ancestor_record_offset":668,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":728,"parent_record_offset":668,"ancestor_record_offset":668,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":566,"record_offset":936,"parent_record_offset":896,"ancestor_record_offset":668,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":1020,"parent_record_offset":896,"ancestor_record_offset":668,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":668,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":668,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":668,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":668,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::buffer::ManagedBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":1896,"rva":18314000,"va":5387023120}
// 编译实例 RVAs: 0x1177310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t69716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023120,"raw_hex":"e854100100","size":5,"soft_fail":false,"target_va":5387093072,"va":5387023351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10660464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023120,"raw_hex":"e890555dff","size":5,"soft_fail":false,"target_va":5376362912,"va":5387023371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15955705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023120,"raw_hex":"e8f976f300","size":5,"soft_fail":false,"target_va":5402979152,"va":5387023442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15955921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023120,"raw_hex":"e8d177f300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387023418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":2268,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":2156,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2212,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2104,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":2308,"parent_record_offset":2268,"ancestor_record_offset":1896,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":2392,"parent_record_offset":2268,"ancestor_record_offset":1896,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1896,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1896,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1896,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2124,"parent_record_offset":2104,"ancestor_record_offset":1896,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":248,"rva":18313376,"va":5387022496}
// 编译实例 RVAs: 0x11770a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t69751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022496,"raw_hex":"e877100100","size":5,"soft_fail":false,"target_va":5387092352,"va":5387022596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10659709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022496,"raw_hex":"e883585dff","size":5,"soft_fail":false,"target_va":5376362912,"va":5387022616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022496,"raw_hex":"e8ef79f300","size":5,"soft_fail":false,"target_va":5402979152,"va":5387022684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022496,"raw_hex":"e8c77af300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387022660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":648,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":536,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":592,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":484,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":688,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":772,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":248,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":248,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":504,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":1352,"rva":18313808,"va":5387022928}
// 编译实例 RVAs: 0x1177250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t68592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022928,"raw_hex":"e8f00b0100","size":5,"soft_fail":false,"target_va":5387091616,"va":5387023019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10660132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022928,"raw_hex":"e8dc565dff","size":5,"soft_fail":false,"target_va":5376362912,"va":5387023039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022928,"raw_hex":"e84778f300","size":5,"soft_fail":false,"target_va":5402979152,"va":5387023108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022928,"raw_hex":"e81f79f300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387023084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":1736,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1624,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1680,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1572,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":1776,"parent_record_offset":1736,"ancestor_record_offset":1352,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1860,"parent_record_offset":1736,"ancestor_record_offset":1352,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1352,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":1352,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1352,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1592,"parent_record_offset":1572,"ancestor_record_offset":1352,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":808,"rva":18313584,"va":5387022704}
// 编译实例 RVAs: 0x1177170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t69160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022704,"raw_hex":"e8280e0100","size":5,"soft_fail":false,"target_va":5387091984,"va":5387022819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10659932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022704,"raw_hex":"e8a4575dff","size":5,"soft_fail":false,"target_va":5376362912,"va":5387022839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022704,"raw_hex":"e80f79f300","size":5,"soft_fail":false,"target_va":5402979152,"va":5387022908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15956455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387022704,"raw_hex":"e8e779f300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387022884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":1192,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1080,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1136,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1028,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":1591,"record_offset":1232,"parent_record_offset":1192,"ancestor_record_offset":808,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1316,"parent_record_offset":1192,"ancestor_record_offset":808,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1591,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":808,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":808,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":808,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":1048,"parent_record_offset":1028,"ancestor_record_offset":808,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResourceStore::release
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x784C
// 首条位置: {"module":566,"inlinee":30796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5356,"parent_record_offset":5316,"ancestor_record_offset":5316,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5496,"parent_record_offset":5316,"ancestor_record_offset":5316,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5440,"parent_record_offset":5316,"ancestor_record_offset":5316,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5412,"parent_record_offset":5316,"ancestor_record_offset":5316,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":5316,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":5316,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":248,"rva":7506352,"va":5376215472}
// 编译实例 RVAs: 0x7289b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3465970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e80e1dcbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376215661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e8b72f0000","size":5,"soft_fail":false,"target_va":5376228064,"va":5376215844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bb70
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e883300000","size":5,"soft_fail":false,"target_va":5376228208,"va":5376215784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bc00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e8cb300000","size":5,"soft_fail":false,"target_va":5376228352,"va":5376215856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072be30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e813330000","size":5,"soft_fail":false,"target_va":5376228912,"va":5376215832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e8af330000","size":5,"soft_fail":false,"target_va":5376229056,"va":5376215820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bf50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e857340000","size":5,"soft_fail":false,"target_va":5376229200,"va":5376215796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27377865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e8c9c0a101","size":5,"soft_fail":false,"target_va":5403593392,"va":5376215522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26762386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215472,"raw_hex":"e8925c9801","size":5,"soft_fail":false,"target_va":5402978144,"va":5376215753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":549,"record_offset":588,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$1::default
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":549,"record_offset":392,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":30434,"source_line":62,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":248,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":248,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":248,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":248,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":549,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":248,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":248,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::owning_device
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76F2
// 首条位置: {"module":571,"inlinee":30450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::throw_out_managed_resource
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76E6
// 首条位置: {"module":549,"inlinee":30438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferSeqTracker::current
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":549,"record_offset":1600,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":30442,"source_line":68,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::DisposedRawResourceHandle::resource
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":549,"record_offset":1572,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":30440,"source_line":54,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1540,"inlinee":19665,"source_line":2413,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1540,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111A6
// 首条位置: {"module":1592,"inlinee":70054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
// 记录形式: compiled_procedure; instances=1
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":808,"rva":7506752,"va":5376215872}
// 编译实例 RVAs: 0x728b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3466879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e88119cbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376216586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c7c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e895390000","size":5,"soft_fail":false,"target_va":5376231360,"va":5376216614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140741d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8de8e0100","size":5,"soft_fail":false,"target_va":5376318720,"va":5376216605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10810987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e86bf6a400","size":5,"soft_fail":false,"target_va":5387027536,"va":5376216544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411786f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10811700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e834f9a400","size":5,"soft_fail":false,"target_va":5387028208,"va":5376216503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10812648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8e8fca400","size":5,"soft_fail":false,"target_va":5387028880,"va":5376216227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10813167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8effea400","size":5,"soft_fail":false,"target_va":5387029552,"va":5376216380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10813757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e83d01a500","size":5,"soft_fail":false,"target_va":5387030224,"va":5376216462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141179170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10814549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e85504a500","size":5,"soft_fail":false,"target_va":5387030896,"va":5376216342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141179410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10815142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8a606a500","size":5,"soft_fail":false,"target_va":5387031568,"va":5376216421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26762488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8f85c9801","size":5,"soft_fail":false,"target_va":5402979152,"va":5376216659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26752911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e88f379801","size":5,"soft_fail":false,"target_va":5402968992,"va":5376216076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26754642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376215872,"raw_hex":"e8523e9801","size":5,"soft_fail":false,"target_va":5402970800,"va":5376216153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":1484,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":549,"record_offset":1688,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":549,"record_offset":1192,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":1308,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":1020,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":549,"record_offset":992,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":30436,"source_line":104,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::throw_out_managed_resource
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":549,"record_offset":1540,"parent_record_offset":808,"ancestor_record_offset":808,"inlinee":30438,"source_line":177,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":808,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":2852,"parent_record_offset":2760,"ancestor_record_offset":808,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":2788,"parent_record_offset":2760,"ancestor_record_offset":808,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":808,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":549,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":808,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":549,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":808,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":549,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":808,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":549,"record_offset":2816,"parent_record_offset":2788,"ancestor_record_offset":808,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":549,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":808,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":549,"record_offset":1720,"parent_record_offset":1688,"ancestor_record_offset":808,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":808,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":1272,"parent_record_offset":1216,"ancestor_record_offset":808,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":549,"record_offset":1128,"parent_record_offset":1044,"ancestor_record_offset":808,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":549,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":808,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":808,"inlinee":19665,"source_line":2413,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1388,"parent_record_offset":1356,"ancestor_record_offset":808,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":808,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":808,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":808,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":808,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":549,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":808,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":808,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":549,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":808,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::current
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":549,"record_offset":1600,"parent_record_offset":1540,"ancestor_record_offset":808,"inlinee":30442,"source_line":68,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::DisposedRawResourceHandle::resource
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":549,"record_offset":1572,"parent_record_offset":1540,"ancestor_record_offset":808,"inlinee":30440,"source_line":54,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked
// 记录形式: inlinee_declaration; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1119E
// 首条位置: {"module":1617,"inlinee":70046}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":2428,"rva":18314336,"va":5387023456}
// 编译实例 RVAs: 0x1177460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16202754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023456,"raw_hex":"e8023cf700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387023721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15955091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023456,"raw_hex":"e89374f300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387023848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":2948,"parent_record_offset":2428,"ancestor_record_offset":2428,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":2692,"parent_record_offset":2428,"ancestor_record_offset":2428,"inlinee":7646,"source_line":328,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1591,"record_offset":2912,"parent_record_offset":2820,"ancestor_record_offset":2428,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2856,"parent_record_offset":2820,"ancestor_record_offset":2428,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2428,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1591,"record_offset":2820,"parent_record_offset":2692,"ancestor_record_offset":2428,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":2744,"parent_record_offset":2692,"ancestor_record_offset":2428,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":2792,"parent_record_offset":2692,"ancestor_record_offset":2428,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":2768,"parent_record_offset":2692,"ancestor_record_offset":2428,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::create
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B7D
// 首条位置: {"module":139,"inlinee":11133}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":139,"record_offset":16468,"parent_record_offset":16428,"ancestor_record_offset":16428,"inlinee":11062,"source_line":302,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$11::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":85368,"parent_record_offset":85336,"ancestor_record_offset":85336,"inlinee":31062,"source_line":418,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$12::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":63456,"parent_record_offset":63420,"ancestor_record_offset":63420,"inlinee":31012,"source_line":428,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$6::allocate
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1624,"record_offset":12284,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":30825,"source_line":368,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$7::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":10168,"parent_record_offset":10132,"ancestor_record_offset":10132,"inlinee":30916,"source_line":378,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$8::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78464,"parent_record_offset":78420,"ancestor_record_offset":78420,"inlinee":31051,"source_line":388,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$9::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":65688,"parent_record_offset":65656,"ancestor_record_offset":65656,"inlinee":31025,"source_line":398,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78776,"parent_record_offset":78728,"ancestor_record_offset":78420,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78728,"parent_record_offset":78496,"ancestor_record_offset":78420,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78860,"parent_record_offset":78728,"ancestor_record_offset":78420,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78616,"parent_record_offset":78496,"ancestor_record_offset":78420,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78640,"parent_record_offset":78616,"ancestor_record_offset":78420,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78672,"parent_record_offset":78496,"ancestor_record_offset":78420,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78560,"parent_record_offset":78496,"ancestor_record_offset":78420,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78800,"parent_record_offset":78776,"ancestor_record_offset":78420,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78696,"parent_record_offset":78672,"ancestor_record_offset":78420,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78584,"parent_record_offset":78560,"ancestor_record_offset":78420,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78496,"parent_record_offset":78464,"ancestor_record_offset":78420,"inlinee":30789,"source_line":71,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::get
// 记录形式: inline_instance, inlinee_declaration; instances=53
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DEA
// 首条位置: {"module":52,"inlinee":7658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=33
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6380,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=33
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6436,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=33
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6408,"parent_record_offset":6336,"ancestor_record_offset":6336,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::get_tracked
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DDE
// 首条位置: {"module":52,"inlinee":7646}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1591,"record_offset":2820,"parent_record_offset":2692,"ancestor_record_offset":2692,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5020,"parent_record_offset":4976,"ancestor_record_offset":4976,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5076,"parent_record_offset":4976,"ancestor_record_offset":4976,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5048,"parent_record_offset":4976,"ancestor_record_offset":4976,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1591,"record_offset":2912,"parent_record_offset":2820,"ancestor_record_offset":2692,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2856,"parent_record_offset":2820,"ancestor_record_offset":2692,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2692,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::buffer::ManagedBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":3500,"rva":18315024,"va":5387024144}
// 编译实例 RVAs: 0x1177710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16202186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024144,"raw_hex":"e8ca39f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387024289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15954523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024144,"raw_hex":"e85b72f300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387024416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1591,"record_offset":3776,"parent_record_offset":3500,"ancestor_record_offset":3500,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3700,"parent_record_offset":3500,"ancestor_record_offset":3500,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3748,"parent_record_offset":3500,"ancestor_record_offset":3500,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3724,"parent_record_offset":3500,"ancestor_record_offset":3500,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1591,"record_offset":3864,"parent_record_offset":3776,"ancestor_record_offset":3500,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":3808,"parent_record_offset":3776,"ancestor_record_offset":3500,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":3500,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":3076,"rva":18314736,"va":5387023856}
// 编译实例 RVAs: 0x11775f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16202474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023856,"raw_hex":"e8ea3af700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387024001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15954811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387023856,"raw_hex":"e87b73f300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387024128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1591,"record_offset":3376,"parent_record_offset":3076,"ancestor_record_offset":3076,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3300,"parent_record_offset":3076,"ancestor_record_offset":3076,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3348,"parent_record_offset":3076,"ancestor_record_offset":3076,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":3324,"parent_record_offset":3076,"ancestor_record_offset":3076,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1591,"record_offset":3464,"parent_record_offset":3376,"ancestor_record_offset":3076,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":3076,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":3076,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":3900,"rva":18315312,"va":5387024432}
// 编译实例 RVAs: 0x1177830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16201898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024432,"raw_hex":"e8aa38f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387024577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15954235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024432,"raw_hex":"e83b71f300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387024704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1591,"record_offset":4184,"parent_record_offset":3900,"ancestor_record_offset":3900,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4108,"parent_record_offset":3900,"ancestor_record_offset":3900,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4156,"parent_record_offset":3900,"ancestor_record_offset":3900,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4132,"parent_record_offset":3900,"ancestor_record_offset":3900,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1591,"record_offset":4272,"parent_record_offset":4184,"ancestor_record_offset":3900,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":4216,"parent_record_offset":4184,"ancestor_record_offset":3900,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3900,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":4308,"rva":18315600,"va":5387024720}
// 编译实例 RVAs: 0x1177950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ced40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15371532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024720,"raw_hex":"e8f47215ff","size":5,"soft_fail":false,"target_va":5371653440,"va":5387024967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072cb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10793021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024720,"raw_hex":"e8c34f5bff","size":5,"soft_fail":false,"target_va":5376232224,"va":5387025240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411814c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024720,"raw_hex":"e8e49a0000","size":5,"soft_fail":false,"target_va":5387064512,"va":5387024855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024720,"raw_hex":"e821dfffff","size":5,"soft_fail":false,"target_va":5387016528,"va":5387024938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16201400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387024720,"raw_hex":"e8b836f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387025075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1591,"record_offset":4592,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":4648,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4512,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4564,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":4540,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$7::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":5288,"parent_record_offset":4308,"ancestor_record_offset":4308,"inlinee":70023,"source_line":382,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1591,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":4308,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1591,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4308,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1591,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":4308,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":1591,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4308,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::impl$7::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":1591,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4308,"inlinee":13375,"source_line":327,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":1591,"record_offset":4832,"parent_record_offset":4744,"ancestor_record_offset":4308,"inlinee":9138,"source_line":86,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":1591,"record_offset":4884,"parent_record_offset":4856,"ancestor_record_offset":4308,"inlinee":9140,"source_line":121,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":1591,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4308,"inlinee":9148,"source_line":169,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":1591,"record_offset":4776,"parent_record_offset":4744,"ancestor_record_offset":4308,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1591,"record_offset":5056,"parent_record_offset":4832,"ancestor_record_offset":4308,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1591,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":4308,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1591,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4308,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1591,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":4308,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":4308,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4308,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5356,"parent_record_offset":5316,"ancestor_record_offset":4308,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5496,"parent_record_offset":5316,"ancestor_record_offset":4308,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5440,"parent_record_offset":5316,"ancestor_record_offset":4308,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5164,"parent_record_offset":5128,"ancestor_record_offset":4308,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":4308,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":4308,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4308,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5220,"parent_record_offset":5128,"ancestor_record_offset":4308,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5412,"parent_record_offset":5316,"ancestor_record_offset":4308,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":4308,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":7152,"rva":18316880,"va":5387026000}
// 编译实例 RVAs: 0x1177e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15721033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026000,"raw_hex":"e8b71d10ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5387026084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16200264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026000,"raw_hex":"e84832f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387026211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::descriptor_set::impl$1::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/descriptor_set.rs
//     evidence={"module":1591,"record_offset":7456,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":70035,"source_line":22,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7376,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7428,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7404,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$11::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":7660,"parent_record_offset":7152,"ancestor_record_offset":7152,"inlinee":70038,"source_line":422,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":7152,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":7560,"parent_record_offset":7528,"ancestor_record_offset":7152,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":7152,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":7616,"parent_record_offset":7528,"ancestor_record_offset":7152,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7728,"parent_record_offset":7688,"ancestor_record_offset":7152,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7868,"parent_record_offset":7688,"ancestor_record_offset":7152,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7812,"parent_record_offset":7688,"ancestor_record_offset":7152,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":7152,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":7152,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":7152,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$11::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":7152,"inlinee":70037,"source_line":251,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7784,"parent_record_offset":7688,"ancestor_record_offset":7152,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":7688,"parent_record_offset":7660,"ancestor_record_offset":7152,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":8548,"rva":18317648,"va":5387026768}
// 编译实例 RVAs: 0x1178150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15721795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026768,"raw_hex":"e8bd1a10ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5387026846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16199496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026768,"raw_hex":"e8482ff700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387026979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::framebuffer::impl$0::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/framebuffer.rs
//     evidence={"module":1591,"record_offset":8836,"parent_record_offset":8548,"ancestor_record_offset":8548,"inlinee":70040,"source_line":17,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8756,"parent_record_offset":8548,"ancestor_record_offset":8548,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8808,"parent_record_offset":8548,"ancestor_record_offset":8548,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8784,"parent_record_offset":8548,"ancestor_record_offset":8548,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$12::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9040,"parent_record_offset":8548,"ancestor_record_offset":8548,"inlinee":70043,"source_line":432,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":8548,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":8940,"parent_record_offset":8908,"ancestor_record_offset":8548,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":8548,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":8996,"parent_record_offset":8908,"ancestor_record_offset":8548,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9108,"parent_record_offset":9068,"ancestor_record_offset":8548,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9248,"parent_record_offset":9068,"ancestor_record_offset":8548,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9192,"parent_record_offset":9068,"ancestor_record_offset":8548,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":8548,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":8548,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":8548,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$12::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":8548,"inlinee":70042,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9164,"parent_record_offset":9068,"ancestor_record_offset":8548,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9068,"parent_record_offset":9040,"ancestor_record_offset":8548,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":5564,"rva":18316144,"va":5387025264}
// 编译实例 RVAs: 0x1177b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15720297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387025264,"raw_hex":"e8972010ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5387025348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16201000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387025264,"raw_hex":"e82835f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387025475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1591,"record_offset":5856,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5776,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5828,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5804,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::render_pass::impl$0::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/render_pass.rs
//     evidence={"module":1591,"record_offset":5912,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":70025,"source_line":19,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$9::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6116,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":70028,"source_line":402,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5564,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6016,"parent_record_offset":5984,"ancestor_record_offset":5564,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":5564,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":5564,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":6072,"parent_record_offset":5984,"ancestor_record_offset":5564,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6184,"parent_record_offset":6144,"ancestor_record_offset":5564,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6324,"parent_record_offset":6144,"ancestor_record_offset":5564,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6268,"parent_record_offset":6144,"ancestor_record_offset":5564,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":5564,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":5564,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":5564,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$9::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":5564,"inlinee":70027,"source_line":239,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":6240,"parent_record_offset":6144,"ancestor_record_offset":5564,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6144,"parent_record_offset":6116,"ancestor_record_offset":5564,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":6392,"rva":18316512,"va":5387025632}
// 编译实例 RVAs: 0x1177ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15720665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387025632,"raw_hex":"e8271f10ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5387025716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16200632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387025632,"raw_hex":"e8b833f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387025843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":6592,"parent_record_offset":6392,"ancestor_record_offset":6392,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":6644,"parent_record_offset":6392,"ancestor_record_offset":6392,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":6620,"parent_record_offset":6392,"ancestor_record_offset":6392,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$10::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6876,"parent_record_offset":6392,"ancestor_record_offset":6392,"inlinee":70033,"source_line":412,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::sampler::impl$0::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/sampler.rs
//     evidence={"module":1591,"record_offset":6672,"parent_record_offset":6392,"ancestor_record_offset":6392,"inlinee":70030,"source_line":17,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6392,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":6392,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6392,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":6832,"parent_record_offset":6744,"ancestor_record_offset":6392,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6944,"parent_record_offset":6904,"ancestor_record_offset":6392,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7084,"parent_record_offset":6904,"ancestor_record_offset":6392,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7028,"parent_record_offset":6904,"ancestor_record_offset":6392,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":6392,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6392,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":6392,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$10::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":6392,"inlinee":70032,"source_line":245,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7000,"parent_record_offset":6904,"ancestor_record_offset":6392,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":6392,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":9316,"rva":18318016,"va":5387027136}
// 编译实例 RVAs: 0x11782c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072d960
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10791383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027136,"raw_hex":"e829565bff","size":5,"soft_fail":false,"target_va":5376235872,"va":5387027250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118b3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t78002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027136,"raw_hex":"e8b2300100","size":5,"soft_fail":false,"target_va":5387105248,"va":5387027241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16566125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027136,"raw_hex":"e86dc7fc00","size":5,"soft_fail":false,"target_va":5403593360,"va":5387027230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16199126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027136,"raw_hex":"e8d62df700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387027349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1591,"record_offset":9600,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9520,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9572,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9548,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$8::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9656,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":70044,"source_line":392,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":9316,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9724,"parent_record_offset":9684,"ancestor_record_offset":9316,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9864,"parent_record_offset":9684,"ancestor_record_offset":9316,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9808,"parent_record_offset":9684,"ancestor_record_offset":9316,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":9316,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9316,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9780,"parent_record_offset":9684,"ancestor_record_offset":9316,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9684,"parent_record_offset":9656,"ancestor_record_offset":9316,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1591,"record_offset":7936,"rva":18317248,"va":5387026368}
// 编译实例 RVAs: 0x1177fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072ced0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10793319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026368,"raw_hex":"e8994e5bff","size":5,"soft_fail":false,"target_va":5376233168,"va":5387026482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140734860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10762190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026368,"raw_hex":"e832c85bff","size":5,"soft_fail":false,"target_va":5376264288,"va":5387026473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16566893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026368,"raw_hex":"e86dcafc00","size":5,"soft_fail":false,"target_va":5403593360,"va":5387026462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16199894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387026368,"raw_hex":"e8d630f700","size":5,"soft_fail":false,"target_va":5403226480,"va":5387026581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1591,"record_offset":8216,"parent_record_offset":7936,"ancestor_record_offset":7936,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8136,"parent_record_offset":7936,"ancestor_record_offset":7936,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8188,"parent_record_offset":7936,"ancestor_record_offset":7936,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8164,"parent_record_offset":7936,"ancestor_record_offset":7936,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$6::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":8272,"parent_record_offset":7936,"ancestor_record_offset":7936,"inlinee":30826,"source_line":372,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":7936,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8340,"parent_record_offset":8300,"ancestor_record_offset":7936,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8480,"parent_record_offset":8300,"ancestor_record_offset":7936,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8424,"parent_record_offset":8300,"ancestor_record_offset":7936,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":7936,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7936,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8396,"parent_record_offset":8300,"ancestor_record_offset":7936,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":7936,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$6::allocate
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7869
// 首条位置: {"module":566,"record_offset":532,"rva":7554928,"va":5376264048}
// 编译实例 RVAs: 0x734770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resources.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27329242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264048,"raw_hex":"e8da02a101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376264113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10828523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264048,"raw_hex":"e8eb3aa500","size":5,"soft_fail":false,"target_va":5387092704,"va":5376264176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t98711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264048,"raw_hex":"e897810100","size":5,"soft_fail":false,"target_va":5376362912,"va":5376264196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26714878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264048,"raw_hex":"e8fea29701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376264269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26715094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264048,"raw_hex":"e8d6a39701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376264245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":566,"record_offset":668,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":30789,"source_line":71,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":566,"record_offset":936,"parent_record_offset":896,"ancestor_record_offset":532,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":566,"record_offset":896,"parent_record_offset":668,"ancestor_record_offset":532,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":1020,"parent_record_offset":896,"ancestor_record_offset":532,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":784,"parent_record_offset":668,"ancestor_record_offset":532,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":532,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":840,"parent_record_offset":668,"ancestor_record_offset":532,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":728,"parent_record_offset":668,"ancestor_record_offset":532,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":532,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":532,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":532,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$6::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x786A
// 首条位置: {"module":1591,"inlinee":30826}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":8272,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8340,"parent_record_offset":8300,"ancestor_record_offset":8272,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8480,"parent_record_offset":8300,"ancestor_record_offset":8272,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8424,"parent_record_offset":8300,"ancestor_record_offset":8272,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":8272,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":8272,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":8396,"parent_record_offset":8300,"ancestor_record_offset":8272,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$7::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C4
// 首条位置: {"module":570,"inlinee":30916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$7::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11187
// 首条位置: {"module":1591,"inlinee":70023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":5288,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5356,"parent_record_offset":5316,"ancestor_record_offset":5288,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5496,"parent_record_offset":5316,"ancestor_record_offset":5288,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5440,"parent_record_offset":5316,"ancestor_record_offset":5288,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":5288,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":5288,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":5412,"parent_record_offset":5316,"ancestor_record_offset":5288,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$8::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794B
// 首条位置: {"module":570,"inlinee":31051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78496,"parent_record_offset":78464,"ancestor_record_offset":78464,"inlinee":30789,"source_line":71,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78776,"parent_record_offset":78728,"ancestor_record_offset":78464,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78728,"parent_record_offset":78496,"ancestor_record_offset":78464,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78860,"parent_record_offset":78728,"ancestor_record_offset":78464,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78616,"parent_record_offset":78496,"ancestor_record_offset":78464,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78640,"parent_record_offset":78616,"ancestor_record_offset":78464,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78672,"parent_record_offset":78496,"ancestor_record_offset":78464,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78560,"parent_record_offset":78496,"ancestor_record_offset":78464,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78800,"parent_record_offset":78776,"ancestor_record_offset":78464,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78696,"parent_record_offset":78672,"ancestor_record_offset":78464,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78584,"parent_record_offset":78560,"ancestor_record_offset":78464,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$8::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1119C
// 首条位置: {"module":1591,"inlinee":70044}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9684,"parent_record_offset":9656,"ancestor_record_offset":9656,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9724,"parent_record_offset":9684,"ancestor_record_offset":9656,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9864,"parent_record_offset":9684,"ancestor_record_offset":9656,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9808,"parent_record_offset":9684,"ancestor_record_offset":9656,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":9656,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9656,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9780,"parent_record_offset":9684,"ancestor_record_offset":9656,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$9::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7931
// 首条位置: {"module":570,"inlinee":31025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$9::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 402
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1118C
// 首条位置: {"module":1591,"inlinee":70028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6144,"parent_record_offset":6116,"ancestor_record_offset":6116,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6184,"parent_record_offset":6144,"ancestor_record_offset":6116,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6324,"parent_record_offset":6144,"ancestor_record_offset":6116,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6268,"parent_record_offset":6144,"ancestor_record_offset":6116,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":6116,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":6116,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":6240,"parent_record_offset":6144,"ancestor_record_offset":6116,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$10::allocate
// 记录形式: inlinee_declaration; instances=2
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7934
// 首条位置: {"module":570,"inlinee":31028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$10::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 412
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11191
// 首条位置: {"module":1591,"inlinee":70033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":6876,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6944,"parent_record_offset":6904,"ancestor_record_offset":6876,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7084,"parent_record_offset":6904,"ancestor_record_offset":6876,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7028,"parent_record_offset":6904,"ancestor_record_offset":6876,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":6876,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6876,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7000,"parent_record_offset":6904,"ancestor_record_offset":6876,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$11::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7956
// 首条位置: {"module":570,"inlinee":31062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$11::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11196
// 首条位置: {"module":1591,"inlinee":70038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":7688,"parent_record_offset":7660,"ancestor_record_offset":7660,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7728,"parent_record_offset":7688,"ancestor_record_offset":7660,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7868,"parent_record_offset":7688,"ancestor_record_offset":7660,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7812,"parent_record_offset":7688,"ancestor_record_offset":7660,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":7660,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":7660,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":7784,"parent_record_offset":7688,"ancestor_record_offset":7660,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$12::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7924
// 首条位置: {"module":570,"inlinee":31012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::resources::impl$12::release
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 432
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1119B
// 首条位置: {"module":1591,"inlinee":70043}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResourceStore::release
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1591,"record_offset":9068,"parent_record_offset":9040,"ancestor_record_offset":9040,"inlinee":30796,"source_line":119,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9108,"parent_record_offset":9068,"ancestor_record_offset":9040,"inlinee":30797,"source_line":2413,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9248,"parent_record_offset":9068,"ancestor_record_offset":9040,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9192,"parent_record_offset":9068,"ancestor_record_offset":9040,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":9040,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1591,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":9040,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedResourceChecksum::invalidate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1591,"record_offset":9164,"parent_record_offset":9068,"ancestor_record_offset":9040,"inlinee":30799,"source_line":27,"archive":"_pdb/A/modules/01591.jsonl.gz"}
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
// 完整结构及原始类型记录: resources.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::resources::AccessManagedResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked", "arguments": ["rhapsody::raw::managed::resources::ManagedResources", "rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::resources::ManagedResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::render_pass::ManagedRenderPass>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::render_pass::ManagedRenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::render_pass::ManagedRenderPass> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::render_pass::ManagedRenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::resources::ManagedResourceMeta
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::resources::ManagedResourceStore
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::sampler::ManagedSampler>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::sampler::ManagedSampler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::sampler::ManagedSampler> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::sampler::ManagedSampler>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::render_pass::ManagedRenderPass>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::render_pass::ManagedRenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::render_pass::ManagedRenderPass> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::render_pass::ManagedRenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::render_pass::ManagedRenderPass>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::render_pass::ManagedRenderPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore::allocate", "arguments": ["rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::resources::ManagedResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResources>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>", "constructor": "rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked", "arguments": ["rhapsody::raw::managed::resources::ManagedResources", "rhapsody::raw::managed::buffer::ManagedBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resources.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resources.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked
// 信息 ID: eed2f9d6aac05f744f89d47b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>
// 信息 ID: e0bdfd642ac7e8c2fbb793b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked -> ["rhapsody::raw::managed::resources::ManagedResources", "rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding @ 0x733c8c -> 0x1177460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding @ 0x733d5c -> 0x1177460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data @ 0x733e50 -> 0x1177460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::end_top_level_timestamp_query @ 0x736d57 -> 0x1177460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::get_vk_buffer @ 0x73a3bb -> 0x1177460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
// 信息 ID: bf451a210588354378f8393e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::handle::impl$4::track; module=568, inline_record=1088; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::acquire_next_texture; module=569, inline_record=4392; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::record_crash_marker; module=570, inline_record=19584; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=3708; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0; module=1624, inline_record=1192; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0; module=1624, inline_record=1424; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; module=1624, inline_record=5124; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; module=1624, inline_record=5412; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::allocate
// 信息 ID: c04ae093d622b3cad76c9eaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$6::allocate; module=566, inline_record=668; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_swapchain; module=570, inline_record=78496; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >; module=1624, inline_record=12324; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > >; module=3007, inline_record=14476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::buffer::ManagedBuffer>
// 信息 ID: 8ccdc706bf40166f6c97ded6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::ManagedResourceStore::allocate -> ["rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer_impl @ 0x7390f8 -> 0x1177310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 信息 ID: 722dba67a707b0284cfd4654
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::ManagedResourceStore::allocate -> ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set @ 0x744bd7 -> 0x11770a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>
// 信息 ID: 01c6fa745ade0ce6b6138fa8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::ManagedResourceStore::allocate -> ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_framebuffer @ 0x742971 -> 0x1177250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::allocate<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 信息 ID: 43407e175e11af2e6350da16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::ManagedResourceStore::allocate -> ["rhapsody::raw::managed::render_pass::ManagedRenderPass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_render_pass @ 0x742d4d -> 0x1177170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResourceStore::release
// 信息 ID: 19f6684ced1a6f9217714231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer>; module=1591, inline_record=5316; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1591, inline_record=6144; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1591, inline_record=6904; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1591, inline_record=7688; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>; module=1591, inline_record=8300; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1591, inline_record=9068; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>; module=1591, inline_record=9684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::new
// 信息 ID: a0c12fb10149988b06c234ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73eb4a -> 0x7289b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::owning_device
// 信息 ID: efb2fd23039eb1f7c9983ff1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::command_buffer::CommandBuffer::new; module=571, inline_record=1812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
// 信息 ID: bc1c7d26083fde792955352b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x26a4bd -> 0x728b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0x72e846 -> 0x728b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0x7418f6 -> 0x728b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0x7434b9 -> 0x728b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::ManagedResources::throw_out_managed_resource
// 信息 ID: 94294a4fa0a288c39c3fdd5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1540; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1752; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1900; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2048; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2196; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2344; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$10::allocate
// 信息 ID: 93b5ca5fcb8ebff254ae6877
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$10::release
// 信息 ID: ad30b4a9c5bbde7ff3437bea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1591, inline_record=6876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$11::allocate
// 信息 ID: 7f98c2295ffc8e42c668fb95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=85368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$11::release
// 信息 ID: e5536afa3122856c3fe5a83e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1591, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$12::allocate
// 信息 ID: 269de4aeed5875138d94957c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_framebuffer; module=570, inline_record=63456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$12::release
// 信息 ID: af93f8dfaabad14782e34204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1591, inline_record=9040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$1::default
// 信息 ID: 02b1b24baf2649dc55fce9b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=392; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=420; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=448; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=476; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=504; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=532; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::new; module=549, inline_record=560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
// 信息 ID: 38cce1a27ce2e495b7d5816e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 <- rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >; module=1592, inline_record=2140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >; module=1592, inline_record=2344; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >; module=1592, inline_record=4780; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >; module=1592, inline_record=4984; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; module=1592, inline_record=7396; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; module=1592, inline_record=7600; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; module=1592, inline_record=10020; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; module=1592, inline_record=10224; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::create
// 信息 ID: fc69e257419cdcc7439cc6d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > >; module=139, inline_record=16428; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >; module=139, inline_record=22508; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=570, inline_record=10132; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_framebuffer; module=570, inline_record=63420; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_render_pass; module=570, inline_record=65656; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_swapchain; module=570, inline_record=78420; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=85336; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > >; module=1332, inline_record=48904; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::get
// 信息 ID: 6774d6964b90fc483cb38d00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 33 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=6336; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=6600; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::Buffer::device_address; module=557, inline_record=756; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::Buffer::mapped_slice_mut; module=557, inline_record=1120; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::new; module=565, inline_record=3756; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate; module=565, inline_record=4388; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::on_focus; module=569, inline_record=1156; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::resize; module=569, inline_record=1988; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::get_tracked
// 信息 ID: 4a3a04e4050bb21b3c70e190
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=4976; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=5108; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=9844; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=9976; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>; module=1591, inline_record=2692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::buffer::ManagedBuffer>
// 信息 ID: fa306d9ed73cd3846afd782f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$4::get_tracked -> ["rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::bind_vertex_buffer_by_binding_point @ 0x1136edd -> 0x1177710 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::bind_index_buffer @ 0x1136f4b -> 0x1177710 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::impl$9::update_texture_impl::closure$2 @ 0x118e3d8 -> 0x1177710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 信息 ID: 321ee4bca6c18c68051ce7d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$4::get_tracked -> ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set @ 0x738c17 -> 0x11775f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 信息 ID: 0e9f44c8fda6bb4bf9d16adb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$4::get_tracked -> ["rhapsody::raw::managed::render_pass::ManagedRenderPass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x1136388 -> 0x1177830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer>
// 信息 ID: 4cc022662731ae60a4ddad4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> > @ 0x117902f -> 0x1177950 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> > @ 0x117907e -> 0x1177950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>
// 信息 ID: e9ef129816b80b4a73ba93fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> > @ 0x117884f -> 0x1177e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> > @ 0x117889e -> 0x1177e50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>
// 信息 ID: 6cd07e8bd23aa19f0813466e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> > @ 0x11785af -> 0x1178150 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> > @ 0x11785fe -> 0x1178150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>
// 信息 ID: a10e14b15ebdfc1e4dca8465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::render_pass::ManagedRenderPass"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> > @ 0x1178d8f -> 0x1177b70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> > @ 0x1178dde -> 0x1177b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>
// 信息 ID: f6518aa44f5d06571418d07e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::sampler::ManagedSampler"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> > @ 0x1178aef -> 0x1177ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> > @ 0x1178b3e -> 0x1177ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>
// 信息 ID: b917c77a3531fa1cab22c4f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::swapchain::ManagedSwapchain"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> > @ 0x117956f -> 0x11782c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> > @ 0x11795be -> 0x11782c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>
// 信息 ID: 3cf78e4c06bc7fba2723d01c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$5::dispose -> ["rhapsody::raw::managed::texture::ManagedTexture"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> > @ 0x11792cf -> 0x1177fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> > @ 0x117931e -> 0x1177fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$6::allocate
// 信息 ID: 7ab1fdcf86398665aa789a3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x26b449 -> 0x734770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x26c674 -> 0x734770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xe9e739 -> 0x734770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe9f964 -> 0x734770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1148d14 -> 0x734770 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >; module=1624, inline_record=12284; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > >; module=3007, inline_record=14436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$6::release
// 信息 ID: 41095b3491cc450cccb78139
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>; module=1591, inline_record=8272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$7::allocate
// 信息 ID: 0604f4089a2d1a1c68fd23bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=570, inline_record=10168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$7::release
// 信息 ID: 0487c1fceee077451b72f57c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::buffer::ManagedBuffer>; module=1591, inline_record=5288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$8::allocate
// 信息 ID: 1c551c25c66f65a85899423f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_swapchain; module=570, inline_record=78464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$8::release
// 信息 ID: e36e16fe3e85be2ad7f1bf49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>; module=1591, inline_record=9656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$9::allocate
// 信息 ID: 8ef060047f803cef4e9ab1dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_render_pass; module=570, inline_record=65688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::resources::impl$9::release
// 信息 ID: 0d3109ac3565e59a4ed2f49d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1591, inline_record=6116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
