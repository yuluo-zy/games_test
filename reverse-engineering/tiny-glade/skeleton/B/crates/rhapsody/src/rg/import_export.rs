// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/import_export.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ca21a8fb380fb3087b1b48465a687ca29447e442e4a8d0f217d81060dc1eddc8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: import_export.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 677, 679, 682
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::import_unowned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x987A
// 首条位置: {"module":679,"inlinee":39034}
// 编译实例 RVAs: 0xbd60a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381120160,"raw_hex":"e87f9a0000","size":5,"soft_fail":false,"target_va":5381160080,"va":5381120524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-485201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381120160,"raw_hex":"e8af98f8ff","size":5,"soft_fail":false,"target_va":5380635120,"va":5381120316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":679,"record_offset":7496,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":679,"record_offset":7676,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":38987,"source_line":68,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":7784,"parent_record_offset":7752,"ancestor_record_offset":7356,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":7356,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":7808,"parent_record_offset":7784,"ancestor_record_offset":7356,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":7752,"parent_record_offset":7712,"ancestor_record_offset":7356,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":7712,"parent_record_offset":7676,"ancestor_record_offset":7356,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":679,"record_offset":7872,"parent_record_offset":7712,"ancestor_record_offset":7356,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":679,"record_offset":7584,"parent_record_offset":7548,"ancestor_record_offset":7356,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":679,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":7356,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":679,"record_offset":7524,"parent_record_offset":7496,"ancestor_record_offset":7356,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":679,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":7356,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":679,"record_offset":7648,"parent_record_offset":7496,"ancestor_record_offset":7356,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::import_owned
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":679,"record_offset":8104,"rva":12411424,"va":5381120544}
// 编译实例 RVAs: 0xbd6220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381120544,"raw_hex":"e821990000","size":5,"soft_fail":false,"target_va":5381160080,"va":5381120874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-485586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381120544,"raw_hex":"e82e97f8ff","size":5,"soft_fail":false,"target_va":5380635120,"va":5381120701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":679,"record_offset":8240,"parent_record_offset":8104,"ancestor_record_offset":8104,"inlinee":38987,"source_line":68,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8348,"parent_record_offset":8316,"ancestor_record_offset":8104,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":8104,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":8104,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":8316,"parent_record_offset":8276,"ancestor_record_offset":8104,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":8276,"parent_record_offset":8240,"ancestor_record_offset":8104,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":679,"record_offset":8436,"parent_record_offset":8276,"ancestor_record_offset":8104,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::export
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":679,"record_offset":8668,"rva":12411776,"va":5381120896}
// 编译实例 RVAs: 0xbd6380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6764988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381120896,"raw_hex":"e844c698ff","size":5,"soft_fail":false,"target_va":5374355952,"va":5381120935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":8796,"parent_record_offset":8668,"ancestor_record_offset":8668,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8864,"parent_record_offset":8832,"ancestor_record_offset":8668,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":8668,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":8668,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":8832,"parent_record_offset":8796,"ancestor_record_offset":8668,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":679,"record_offset":8952,"parent_record_offset":8796,"ancestor_record_offset":8668,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::from_owned_rg_resource
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":679,"record_offset":8988,"rva":12411888,"va":5381121008}
// 编译实例 RVAs: 0xbd63f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428ee9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30508464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381121008,"raw_hex":"e8b085d101","size":5,"soft_fail":false,"target_va":5411629552,"va":5381121083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::rg::resource::RuntimeResource> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381121008,"raw_hex":"e889edffff","size":5,"soft_fail":false,"target_va":5381116400,"va":5381121122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$1::import_unowned
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":2576,"rva":12406720,"va":5381115840}
// 编译实例 RVAs: 0xbd4fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115840,"raw_hex":"e87fab0000","size":5,"soft_fail":false,"target_va":5381160080,"va":5381116172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-480865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115840,"raw_hex":"e89fa9f8ff","size":5,"soft_fail":false,"target_va":5380635120,"va":5381115980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":677,"record_offset":2716,"parent_record_offset":2576,"ancestor_record_offset":2576,"inlinee":38982,"source_line":20,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":677,"record_offset":2896,"parent_record_offset":2576,"ancestor_record_offset":2576,"inlinee":38987,"source_line":68,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3004,"parent_record_offset":2972,"ancestor_record_offset":2576,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2576,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2576,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":2972,"parent_record_offset":2932,"ancestor_record_offset":2576,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":2932,"parent_record_offset":2896,"ancestor_record_offset":2576,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":677,"record_offset":3092,"parent_record_offset":2932,"ancestor_record_offset":2576,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":677,"record_offset":2804,"parent_record_offset":2768,"ancestor_record_offset":2576,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":677,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2576,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":677,"record_offset":2868,"parent_record_offset":2716,"ancestor_record_offset":2576,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":2576,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2576,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$1::export
// 记录形式: compiled_procedure; instances=1
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":3324,"rva":12407072,"va":5381116192}
// 编译实例 RVAs: 0xbd5120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6760296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381116192,"raw_hex":"e898d898ff","size":5,"soft_fail":false,"target_va":5374355952,"va":5381116243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":3452,"parent_record_offset":3324,"ancestor_record_offset":3324,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3520,"parent_record_offset":3488,"ancestor_record_offset":3324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":677,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":677,"record_offset":3488,"parent_record_offset":3452,"ancestor_record_offset":3324,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":677,"record_offset":3608,"parent_record_offset":3452,"ancestor_record_offset":3324,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$2::into_sampled_rg_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":679,"record_offset":9124,"rva":12412016,"va":5381121136}
// 编译实例 RVAs: 0xbd6470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381121136,"raw_hex":"e8bf960000","size":5,"soft_fail":false,"target_va":5381160080,"va":5381121484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-486175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381121136,"raw_hex":"e8e194f8ff","size":5,"soft_fail":false,"target_va":5380635120,"va":5381121290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::import_export::impl$0::import_unowned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/import_export.rs
//     evidence={"module":679,"record_offset":9268,"parent_record_offset":9124,"ancestor_record_offset":9124,"inlinee":39034,"source_line":34,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":9124,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9124,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":679,"record_offset":9620,"parent_record_offset":9596,"ancestor_record_offset":9124,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":9564,"parent_record_offset":9524,"ancestor_record_offset":9124,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":679,"record_offset":9524,"parent_record_offset":9488,"ancestor_record_offset":9124,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":679,"record_offset":9684,"parent_record_offset":9524,"ancestor_record_offset":9124,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":679,"record_offset":9396,"parent_record_offset":9360,"ancestor_record_offset":9124,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":679,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":9124,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":679,"record_offset":9336,"parent_record_offset":9308,"ancestor_record_offset":9124,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":679,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":9124,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":679,"record_offset":9308,"parent_record_offset":9268,"ancestor_record_offset":9124,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":679,"record_offset":9460,"parent_record_offset":9308,"ancestor_record_offset":9124,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00679.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":679,"record_offset":9488,"parent_record_offset":9268,"ancestor_record_offset":9124,"inlinee":38987,"source_line":68,"archive":"_pdb/B/modules/00679.jsonl.gz"}
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
// 完整结构及原始类型记录: import_export.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: import_export.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: import_export.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::export
// 信息 ID: ff074aeb48534793f3ba2577
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1635be -> 0xbd6380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x1653ba -> 0xbd6380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166b4c -> 0xbd6380 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31dd71 -> 0xbd6380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources> @ 0x4f7d8b -> 0xbd6380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg @ 0x93cefc -> 0xbd6380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::from_owned_rg_resource
// 信息 ID: 0a36544581dff2a9c5e6ae96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9b67 -> 0xbd63f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::import_owned
// 信息 ID: 85f7f4f93bef35fd1d2c9629
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166aa6 -> 0xbd6220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::import_unowned
// 信息 ID: 1627ee756323f240b7cb5228
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166aca -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31d456 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x4f6c05 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x4f69f3 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f7205 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f6ff3 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg @ 0x93cdcc -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96cc45 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96ca33 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x96d245 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x96d033 -> 0xbd60a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$2::into_sampled_rg_texture; module=679, inline_record=9268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$1::export
// 信息 ID: ae2cdc7f5b5053ad5f8e3a8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31dade -> 0xbd5120 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources> @ 0x4f7e4e -> 0xbd5120 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x589ed2 -> 0xbd5120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg @ 0x91eccd -> 0xbd5120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::export_from_rg<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData> @ 0x15724de -> 0xbd5120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$1::from_owned_rg_resource::panic_cold_explicit
// 信息 ID: 9cc82e47c2c18e0a575395c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] samply_symbols::jitdump::JitDumpIndex::from_reader<samply_symbols::shared::FileContentsCursor<enum2$<wholesym::helper::FileContents> > > @ 0x6dc65b -> 0x28ee9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] linux_perf_data::jitdump::jitdump_reader::JitDumpReader::new_with_buffer_size<samply_symbols::shared::FileContentsCursor<enum2$<wholesym::helper::FileContents> > > @ 0x6de21c -> 0x28ee9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::import_export::impl$0::from_owned_rg_resource @ 0xbd643b -> 0x28ee9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] image::codecs::bmp::decoder::impl$7::read_palettized_pixel_data::closure$1<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x1b72dc5 -> 0x28ee9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] image::codecs::bmp::decoder::impl$7::read_palettized_pixel_data::closure$1<std::io::cursor::Cursor<ref$<slice2$<u8> > > > @ 0x1b7300a -> 0x28ee9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] namespace_unique_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$1::import_unowned
// 信息 ID: d9f3506577cce267e1d8cca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 89 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b32db -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b360b -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x2b393b -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo> @ 0x2b3c6b -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x2b3f9b -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo> @ 0x2b42cb -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x2b45fb -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31d9b1 -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f659c -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f63fc -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x589d86 -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96c5dc -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96c43c -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo> @ 0x12c768b -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo> @ 0x12c79bb -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::rt::RtTrianglesSsbo> @ 0x12c7ceb -> 0xbd4fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$2::into_sampled_rg_texture
// 信息 ID: e76b4c8cf1c099ce3c176ebb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a8e89 -> 0xbd6470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
