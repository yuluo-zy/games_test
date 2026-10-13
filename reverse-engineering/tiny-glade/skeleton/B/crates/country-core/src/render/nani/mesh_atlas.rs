// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/nani/mesh_atlas.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d8b03a61e9e33b3dfd47c4cc5cf0f161f7758065d1d7860e25a64cc3f953bc90
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh_atlas.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 544, 546, 570, 1585, 1610
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79E1
// 首条位置: {"module":544,"inlinee":31201}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":49044,"parent_record_offset":49016,"ancestor_record_offset":49016,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":49292,"parent_record_offset":49264,"ancestor_record_offset":49016,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":49344,"parent_record_offset":49320,"ancestor_record_offset":49016,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":544,"record_offset":49320,"parent_record_offset":49264,"ancestor_record_offset":49016,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":544,"record_offset":49368,"parent_record_offset":49344,"ancestor_record_offset":49016,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":49232,"parent_record_offset":49196,"ancestor_record_offset":49016,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":49392,"parent_record_offset":49368,"ancestor_record_offset":49016,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":49080,"parent_record_offset":49044,"ancestor_record_offset":49016,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":49136,"parent_record_offset":49108,"ancestor_record_offset":49016,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":49264,"parent_record_offset":49196,"ancestor_record_offset":49016,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":49108,"parent_record_offset":49080,"ancestor_record_offset":49016,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":49196,"parent_record_offset":49164,"ancestor_record_offset":49016,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":49164,"parent_record_offset":49136,"ancestor_record_offset":49016,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$6::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79DF
// 首条位置: {"module":544,"inlinee":31199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$9::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A7F
// 首条位置: {"module":546,"inlinee":31359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":34492,"rva":8598096,"va":5377307216}
// 编译实例 RVAs: 0x833250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":34628,"rva":10707456,"va":5379416576}
// 编译实例 RVAs: 0xa36200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b18f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4739570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e80eaeb7ff","size":5,"soft_fail":false,"target_va":5374679280,"va":5379418845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4737064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8d8b7b7ff","size":5,"soft_fail":false,"target_va":5374679664,"va":5379416723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c3490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4667705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8c7c6b8ff","size":5,"soft_fail":false,"target_va":5374751888,"va":5379419588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4336160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8e0d5bdff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379417643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4336887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e809d3bdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379418386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1761377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e89f1fe5ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379419020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4c880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1137062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8a6591100","size":5,"soft_fail":false,"target_va":5380556928,"va":5379419861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b72ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1296584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8c8c81300","size":5,"soft_fail":false,"target_va":5380714192,"va":5379417603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b75360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1303228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8bce21300","size":5,"soft_fail":false,"target_va":5380723552,"va":5379420319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412ff3a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9210868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8f48b8c00","size":5,"soft_fail":false,"target_va":5388628896,"va":5379418023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152ca90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11492910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e82e5eaf00","size":5,"soft_fail":false,"target_va":5390912144,"va":5379419229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31922934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8f61ae701","size":5,"soft_fail":false,"target_va":5411341280,"va":5379418341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32428916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e874d3ee01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379418231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32429494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8b6d5ee01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379417669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3421871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8af363400","size":5,"soft_fail":false,"target_va":5382840208,"va":5379418332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3840084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8ac67c5ff","size":5,"soft_fail":false,"target_va":5375579088,"va":5379419167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31927504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8d02ce701","size":5,"soft_fail":false,"target_va":5411347040,"va":5379419531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32121143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e83721ea01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379419815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32124561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8912eea01","size":5,"soft_fail":false,"target_va":5411541808,"va":5379417242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32122557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8bd26ea01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379419502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32122421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e83526ea01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379419478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::VertexAttributeValues::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-189969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379416576,"raw_hex":"e8ef19fdff","size":5,"soft_fail":false,"target_va":5379228736,"va":5379418700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":544,"record_offset":43552,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":42720,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":6034,"source_line":238,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":46388,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":38736,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":544,"record_offset":43080,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":15681,"source_line":17,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":544,"record_offset":47140,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":544,"record_offset":37472,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":544,"record_offset":42164,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":46608,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":43636,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":34904,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":30873,"source_line":43,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":34848,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":28830,"source_line":35,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":37788,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":30886,"source_line":212,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":37404,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":30848,"source_line":291,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":39808,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":30855,"source_line":11,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":46580,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":41736,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$45::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":44108,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":19936,"source_line":2352,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":45696,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":36220,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":544,"record_offset":37760,"parent_record_offset":34628,"ancestor_record_offset":34628,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":40488,"parent_record_offset":40464,"ancestor_record_offset":34628,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":40512,"parent_record_offset":40488,"ancestor_record_offset":34628,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":41572,"parent_record_offset":41548,"ancestor_record_offset":34628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":40464,"parent_record_offset":40292,"ancestor_record_offset":34628,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":34628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":544,"record_offset":43600,"parent_record_offset":43576,"ancestor_record_offset":34628,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":39052,"parent_record_offset":39024,"ancestor_record_offset":34628,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":38888,"parent_record_offset":38860,"ancestor_record_offset":34628,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":40232,"parent_record_offset":40204,"ancestor_record_offset":34628,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":38972,"parent_record_offset":38948,"ancestor_record_offset":34628,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":43736,"parent_record_offset":43708,"ancestor_record_offset":34628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":41524,"parent_record_offset":41500,"ancestor_record_offset":34628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":38948,"parent_record_offset":38916,"ancestor_record_offset":34628,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":39108,"parent_record_offset":39080,"ancestor_record_offset":34628,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":39080,"parent_record_offset":39052,"ancestor_record_offset":34628,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":38916,"parent_record_offset":38888,"ancestor_record_offset":34628,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":40292,"parent_record_offset":40260,"ancestor_record_offset":34628,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":40260,"parent_record_offset":40232,"ancestor_record_offset":34628,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":34628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":40324,"parent_record_offset":40292,"ancestor_record_offset":34628,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":40168,"parent_record_offset":40140,"ancestor_record_offset":34628,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":40140,"parent_record_offset":39964,"ancestor_record_offset":34628,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":42756,"parent_record_offset":42720,"ancestor_record_offset":34628,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":544,"record_offset":43396,"parent_record_offset":43368,"ancestor_record_offset":34628,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":39024,"parent_record_offset":38808,"ancestor_record_offset":34628,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":40024,"parent_record_offset":40000,"ancestor_record_offset":34628,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":40000,"parent_record_offset":39964,"ancestor_record_offset":34628,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":38808,"parent_record_offset":38772,"ancestor_record_offset":34628,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":34872,"parent_record_offset":34848,"ancestor_record_offset":34628,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":38860,"parent_record_offset":38808,"ancestor_record_offset":34628,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":40204,"parent_record_offset":40168,"ancestor_record_offset":34628,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":39936,"parent_record_offset":39808,"ancestor_record_offset":34628,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":42188,"parent_record_offset":42164,"ancestor_record_offset":34628,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":39428,"parent_record_offset":39384,"ancestor_record_offset":34628,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":39964,"parent_record_offset":39936,"ancestor_record_offset":34628,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":544,"record_offset":43424,"parent_record_offset":43396,"ancestor_record_offset":34628,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":544,"record_offset":39528,"parent_record_offset":39428,"ancestor_record_offset":34628,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":544,"record_offset":39768,"parent_record_offset":38808,"ancestor_record_offset":34628,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":544,"record_offset":39708,"parent_record_offset":39680,"ancestor_record_offset":34628,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":544,"record_offset":38772,"parent_record_offset":38736,"ancestor_record_offset":34628,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":544,"record_offset":40348,"parent_record_offset":40324,"ancestor_record_offset":34628,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":544,"record_offset":40372,"parent_record_offset":40348,"ancestor_record_offset":34628,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":544,"record_offset":43368,"parent_record_offset":43340,"ancestor_record_offset":34628,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":544,"record_offset":43248,"parent_record_offset":43216,"ancestor_record_offset":34628,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":544,"record_offset":35860,"parent_record_offset":35832,"ancestor_record_offset":34628,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":544,"record_offset":43340,"parent_record_offset":43152,"ancestor_record_offset":34628,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":38140,"parent_record_offset":38116,"ancestor_record_offset":34628,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":35360,"parent_record_offset":35336,"ancestor_record_offset":34628,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":35444,"parent_record_offset":35420,"ancestor_record_offset":34628,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":544,"record_offset":37620,"parent_record_offset":37592,"ancestor_record_offset":34628,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":544,"record_offset":37440,"parent_record_offset":37404,"ancestor_record_offset":34628,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":35384,"parent_record_offset":35360,"ancestor_record_offset":34628,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":36036,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":41216,"parent_record_offset":41192,"ancestor_record_offset":34628,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":544,"record_offset":39188,"parent_record_offset":39152,"ancestor_record_offset":34628,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":544,"record_offset":37568,"parent_record_offset":37496,"ancestor_record_offset":34628,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":544,"record_offset":43276,"parent_record_offset":43216,"ancestor_record_offset":34628,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":544,"record_offset":43216,"parent_record_offset":43188,"ancestor_record_offset":34628,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":544,"record_offset":43188,"parent_record_offset":43152,"ancestor_record_offset":34628,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":544,"record_offset":42684,"parent_record_offset":42660,"ancestor_record_offset":34628,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":544,"record_offset":39152,"parent_record_offset":38808,"ancestor_record_offset":34628,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":544,"record_offset":39384,"parent_record_offset":39304,"ancestor_record_offset":34628,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":40396,"parent_record_offset":40372,"ancestor_record_offset":34628,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":40420,"parent_record_offset":40396,"ancestor_record_offset":34628,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":39568,"parent_record_offset":39224,"ancestor_record_offset":34628,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":544,"record_offset":43768,"parent_record_offset":43736,"ancestor_record_offset":34628,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":34628,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":35564,"parent_record_offset":35536,"ancestor_record_offset":34628,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":46632,"parent_record_offset":46608,"ancestor_record_offset":34628,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":38704,"parent_record_offset":37788,"ancestor_record_offset":34628,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":43576,"parent_record_offset":43552,"ancestor_record_offset":34628,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":40636,"parent_record_offset":39808,"ancestor_record_offset":34628,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":41960,"parent_record_offset":41924,"ancestor_record_offset":34628,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":40564,"parent_record_offset":40168,"ancestor_record_offset":34628,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":39596,"parent_record_offset":39224,"ancestor_record_offset":34628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":544,"record_offset":35712,"parent_record_offset":35688,"ancestor_record_offset":34628,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":544,"record_offset":37540,"parent_record_offset":37496,"ancestor_record_offset":34628,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":39480,"parent_record_offset":39428,"ancestor_record_offset":34628,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::SlicePartialEq::not_equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":544,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":34628,"inlinee":15684,"source_line":47,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":544,"record_offset":35944,"parent_record_offset":35916,"ancestor_record_offset":34628,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":544,"record_offset":43104,"parent_record_offset":43080,"ancestor_record_offset":34628,"inlinee":15682,"source_line":18,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":544,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":34628,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":544,"record_offset":35972,"parent_record_offset":35944,"ancestor_record_offset":34628,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":544,"record_offset":37496,"parent_record_offset":37472,"ancestor_record_offset":34628,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":544,"record_offset":42792,"parent_record_offset":42756,"ancestor_record_offset":34628,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":544,"record_offset":35888,"parent_record_offset":35860,"ancestor_record_offset":34628,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":544,"record_offset":37656,"parent_record_offset":37568,"ancestor_record_offset":34628,"inlinee":31204,"source_line":82,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":544,"record_offset":37592,"parent_record_offset":37568,"ancestor_record_offset":34628,"inlinee":31203,"source_line":83,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$3::add::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":544,"record_offset":39336,"parent_record_offset":39304,"ancestor_record_offset":34628,"inlinee":31205,"source_line":102,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":544,"record_offset":37708,"parent_record_offset":37680,"ancestor_record_offset":34628,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":544,"record_offset":37680,"parent_record_offset":37656,"ancestor_record_offset":34628,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":35504,"parent_record_offset":35476,"ancestor_record_offset":34628,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":35536,"parent_record_offset":35504,"ancestor_record_offset":34628,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":46012,"parent_record_offset":45840,"ancestor_record_offset":34628,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":35476,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":35336,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":38116,"parent_record_offset":37944,"ancestor_record_offset":34628,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":36064,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":38196,"parent_record_offset":38172,"ancestor_record_offset":34628,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":38172,"parent_record_offset":37944,"ancestor_record_offset":34628,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":35420,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":544,"record_offset":35308,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":544,"record_offset":35832,"parent_record_offset":35804,"ancestor_record_offset":34628,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":34984,"parent_record_offset":34944,"ancestor_record_offset":34628,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":544,"record_offset":36284,"parent_record_offset":36248,"ancestor_record_offset":34628,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":35024,"parent_record_offset":34984,"ancestor_record_offset":34628,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":38436,"parent_record_offset":38412,"ancestor_record_offset":34628,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":44156,"parent_record_offset":44132,"ancestor_record_offset":34628,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":35804,"parent_record_offset":35636,"ancestor_record_offset":34628,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":39900,"parent_record_offset":39876,"ancestor_record_offset":34628,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":37876,"parent_record_offset":37844,"ancestor_record_offset":34628,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":45728,"parent_record_offset":45696,"ancestor_record_offset":34628,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":544,"record_offset":35128,"parent_record_offset":35024,"ancestor_record_offset":34628,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":46296,"parent_record_offset":45728,"ancestor_record_offset":34628,"inlinee":11978,"source_line":543,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35744,"parent_record_offset":35660,"ancestor_record_offset":34628,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35688,"parent_record_offset":35660,"ancestor_record_offset":34628,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":38256,"parent_record_offset":37944,"ancestor_record_offset":34628,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":36160,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":38540,"parent_record_offset":38484,"ancestor_record_offset":34628,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":37944,"parent_record_offset":37908,"ancestor_record_offset":34628,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35660,"parent_record_offset":35636,"ancestor_record_offset":34628,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35188,"parent_record_offset":35156,"ancestor_record_offset":34628,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35156,"parent_record_offset":35024,"ancestor_record_offset":34628,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35072,"parent_record_offset":35024,"ancestor_record_offset":34628,"inlinee":10143,"source_line":1263,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":38460,"parent_record_offset":38436,"ancestor_record_offset":34628,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35096,"parent_record_offset":35072,"ancestor_record_offset":34628,"inlinee":10144,"source_line":1257,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":41896,"parent_record_offset":41836,"ancestor_record_offset":34628,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":38512,"parent_record_offset":38484,"ancestor_record_offset":34628,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35220,"parent_record_offset":35188,"ancestor_record_offset":34628,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":38484,"parent_record_offset":38460,"ancestor_record_offset":34628,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35280,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":41692,"parent_record_offset":41668,"ancestor_record_offset":34628,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":37908,"parent_record_offset":37876,"ancestor_record_offset":34628,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":544,"record_offset":35636,"parent_record_offset":35220,"ancestor_record_offset":34628,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":34944,"parent_record_offset":34904,"ancestor_record_offset":34628,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":36248,"parent_record_offset":36220,"ancestor_record_offset":34628,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":38412,"parent_record_offset":38388,"ancestor_record_offset":34628,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":44132,"parent_record_offset":44108,"ancestor_record_offset":34628,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":38388,"parent_record_offset":37788,"ancestor_record_offset":34628,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":37844,"parent_record_offset":37812,"ancestor_record_offset":34628,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":39876,"parent_record_offset":39808,"ancestor_record_offset":34628,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":37812,"parent_record_offset":37788,"ancestor_record_offset":34628,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":544,"record_offset":40712,"parent_record_offset":40676,"ancestor_record_offset":34628,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":544,"record_offset":40676,"parent_record_offset":40636,"ancestor_record_offset":34628,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":544,"record_offset":40748,"parent_record_offset":40712,"ancestor_record_offset":34628,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":544,"record_offset":40780,"parent_record_offset":40748,"ancestor_record_offset":34628,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79E4
// 首条位置: {"module":544,"inlinee":31204}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":544,"record_offset":37680,"parent_record_offset":37656,"ancestor_record_offset":37656,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":544,"record_offset":37708,"parent_record_offset":37680,"ancestor_record_offset":37656,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79E3
// 首条位置: {"module":544,"inlinee":31203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":544,"record_offset":37620,"parent_record_offset":37592,"ancestor_record_offset":37592,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::add::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79E5
// 首条位置: {"module":544,"inlinee":31205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79FC
// 首条位置: {"module":570,"inlinee":31228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":12460,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":570,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":30887,"source_line":219,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":570,"record_offset":12020,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":11988,"parent_record_offset":11900,"ancestor_record_offset":11868,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":570,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":11868,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":12096,"parent_record_offset":12068,"ancestor_record_offset":11868,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":11868,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":570,"record_offset":12312,"parent_record_offset":12252,"ancestor_record_offset":11868,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":570,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11868,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11868,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":11868,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":11868,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12252,"parent_record_offset":12192,"ancestor_record_offset":11868,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12224,"parent_record_offset":12192,"ancestor_record_offset":11868,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12284,"parent_record_offset":12252,"ancestor_record_offset":11868,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12400,"parent_record_offset":12192,"ancestor_record_offset":11868,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":570,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11868,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":570,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":11868,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":570,"record_offset":11932,"parent_record_offset":11900,"ancestor_record_offset":11868,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$10::clone_behavior
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":84700,"rva":9072,"va":5368718192}
// 编译实例 RVAs: 0x2370
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
// 完整结构及原始类型记录: mesh_atlas.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlas
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::mesh_atlas::MeshAtlas>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlas"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlasPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "tuple$", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh_atlas.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh_atlas.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 信息 ID: 9a8f39a2f83bb11165243c17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0xa37463 -> 0xa36200 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName> @ 0xa379f3 -> 0xa36200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 信息 ID: 2213ee1199b66075c4b507ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas; module=570, inline_record=11868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 信息 ID: de70e5a937b66a95049e691c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 106 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$2,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xb796c -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > | core::ops::function::FnOnce::call_once<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$0<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >,tuple$<rapier3d::geometry::collider_components::ColliderHandle,ref$<rapier3d::geometry::collider::Collider> > > @ 0x254ce5 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,ref_mut$<rapier3d::pipeline::query_pipeline::impl$7::intersections_with_shape::closure_env$0<country_core::systems::collision::world::impl$14::iter_shape_overlaps::closure_env$1<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> > > > > @ 0x3c12b0 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh> @ 0x4f5511 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh> @ 0x4f554b -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh> @ 0x4f556b -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] slotmap::basic::SlotMap::insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh> @ 0x4f55da -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x7666f8 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities @ 0x7711c7 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities @ 0x770cad -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78dad6 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78e059 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78de00 -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78e32b -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78dc1a -> 0x833250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::generate_lanterns @ 0x78e1c2 -> 0x833250 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$0::default
// 信息 ID: e76f4c82895adc242407a78c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >; module=544, inline_record=49016; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>; module=544, inline_record=54368; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,hashbrown::map::impl$82::extend::closure_env$0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0> > > >; module=1610, inline_record=85516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$10::clone_behavior
// 信息 ID: 1ca4523bf79609c2a705799a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 183 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::look_at_camera::LookAtCamera> @ 0x3bdf7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x81034 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x814f2 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::Sheep> @ 0x81f47 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::SheepAudio> @ 0x825d7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xba779 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeed7f -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeee3a -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfc149 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfe760 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierAnimation> @ 0x157a17 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> @ 0x1580a7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$3::init_state<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> > @ 0x25bfe7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> @ 0x2b1887 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | 其余 9 个同地址别名见共享索引 @ 0x32c4f0 -> 0x2370 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::prefab::PrefabInstance> @ 0x32cbb7 -> 0x2370 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$0
// 信息 ID: 9037c89ec077076530c820b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=544, inline_record=37656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$1
// 信息 ID: 5e6f35e366f5c563bb2acd24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=544, inline_record=37592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::add::closure$2
// 信息 ID: 525fcc3c3771090123abb498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$3::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=544, inline_record=39336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$6::default
// 信息 ID: 8ed2b8d8c61933b5b2be3447
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$9::fmt
// 信息 ID: 50757c747ed77e414778dd04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>; module=546, inline_record=7192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
