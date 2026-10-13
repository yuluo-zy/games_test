// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/import_export.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 40cc377adecd84e3059594c66ba45c015936f995d151c8663751ee2a17d9cd88
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: import_export.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 550, 557, 574
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::import
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79DA
// 首条位置: {"module":574,"inlinee":31194}
// 编译实例 RVAs: 0x7480f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140729720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-125727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376344304,"raw_hex":"e8e114feff","size":5,"soft_fail":false,"target_va":5376218912,"va":5376344634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5038854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376344304,"raw_hex":"e8fa1cb3ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5376344449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":574,"record_offset":6992,"parent_record_offset":6860,"ancestor_record_offset":6860,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":574,"record_offset":7148,"parent_record_offset":6860,"ancestor_record_offset":6860,"inlinee":30511,"source_line":87,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":574,"record_offset":7256,"parent_record_offset":7224,"ancestor_record_offset":6860,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":7224,"parent_record_offset":7184,"ancestor_record_offset":6860,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":7184,"parent_record_offset":7148,"ancestor_record_offset":6860,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":7288,"parent_record_offset":7184,"ancestor_record_offset":6860,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":7080,"parent_record_offset":7044,"ancestor_record_offset":6860,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":6860,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":574,"record_offset":7020,"parent_record_offset":6992,"ancestor_record_offset":6860,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":574,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":6860,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$0::export
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":574,"record_offset":7460,"rva":7635536,"va":5376344656}
// 编译实例 RVAs: 0x748250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5037027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376344656,"raw_hex":"e81d24b3ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5376344686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":7588,"parent_record_offset":7460,"ancestor_record_offset":7460,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":574,"record_offset":7656,"parent_record_offset":7624,"ancestor_record_offset":7460,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":7624,"parent_record_offset":7588,"ancestor_record_offset":7460,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":7688,"parent_record_offset":7588,"ancestor_record_offset":7460,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$1::import
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":2252,"rva":7532016,"va":5376241136}
// 编译实例 RVAs: 0x72edf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140729720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241136,"raw_hex":"e824a8ffff","size":5,"soft_fail":false,"target_va":5376218912,"va":5376241399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4935641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241136,"raw_hex":"e827b0b4ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5376241236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":2384,"parent_record_offset":2252,"ancestor_record_offset":2252,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":557,"record_offset":2508,"parent_record_offset":2252,"ancestor_record_offset":2252,"inlinee":30511,"source_line":87,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":557,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":2252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":2584,"parent_record_offset":2544,"ancestor_record_offset":2252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":2252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":2648,"parent_record_offset":2544,"ancestor_record_offset":2252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":557,"record_offset":2444,"parent_record_offset":2408,"ancestor_record_offset":2252,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":557,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":2252,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":2252,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$1::export
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":2820,"rva":7532304,"va":5376241424}
// 编译实例 RVAs: 0x72ef10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4933808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241424,"raw_hex":"e850b7b4ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5376241467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":2948,"parent_record_offset":2820,"ancestor_record_offset":2820,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":557,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":2820,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":2984,"parent_record_offset":2948,"ancestor_record_offset":2820,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":3048,"parent_record_offset":2948,"ancestor_record_offset":2820,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::import_export::impl$2::into_sampled_rg_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":574,"record_offset":7724,"rva":7635648,"va":5376344768}
// 编译实例 RVAs: 0x7482c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140729720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: import_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-126191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376344768,"raw_hex":"e81113feff","size":5,"soft_fail":false,"target_va":5376218912,"va":5376345098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5039316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376344768,"raw_hex":"e82c1bb3ff","size":5,"soft_fail":false,"target_va":5371305600,"va":5376344911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::import_export::impl$0::import
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/import_export.rs
//     evidence={"module":574,"record_offset":7868,"parent_record_offset":7724,"ancestor_record_offset":7724,"inlinee":31194,"source_line":24,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":574,"record_offset":8172,"parent_record_offset":8140,"ancestor_record_offset":7724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":8140,"parent_record_offset":8100,"ancestor_record_offset":7724,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":574,"record_offset":8100,"parent_record_offset":8064,"ancestor_record_offset":7724,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":574,"record_offset":8204,"parent_record_offset":8100,"ancestor_record_offset":7724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":7996,"parent_record_offset":7960,"ancestor_record_offset":7724,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":574,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7724,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":574,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":7724,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":574,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":7724,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":574,"record_offset":7908,"parent_record_offset":7868,"ancestor_record_offset":7724,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":574,"record_offset":8064,"parent_record_offset":7868,"ancestor_record_offset":7724,"inlinee":30511,"source_line":87,"archive":"_pdb/A/modules/00574.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::export
// 信息 ID: 2603e240a358314c1e1ad014
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources> @ 0xdba92 -> 0x748250 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15bdab -> 0x748250 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffffb -> 0x748250 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x303760 -> 0x748250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg @ 0x5629aa -> 0x748250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$0::import
// 信息 ID: f6664dbcec9a98c17261737f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0xdaca3 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0xdae65 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xdb263 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xdb425 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15b4af -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg @ 0x56287d -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x5ab6a3 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String> @ 0x5ab865 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5abc63 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5abe25 -> 0x7480f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$2::into_sampled_rg_texture; module=574, inline_record=7868; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$1::export
// 信息 ID: e5c91d401ae2bc27f000dba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources> @ 0xdbb2f -> 0x72ef10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15bb2e -> 0x72ef10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x3c2348 -> 0x72ef10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg @ 0x668f9b -> 0x72ef10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::export_from_rg<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> @ 0xf3f122 -> 0x72ef10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$1::import
// 信息 ID: 5e166be81a7556911e8869b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 81 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xda6cd -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xda861 -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x10745c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x10773c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x107a1c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x107cfc -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15ba32 -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::capture_image @ 0x3c2201 -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ab0cd -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ab261 -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::water_mask::WaterMaskSsbo> @ 0xd3295c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::garden_mask::GardenMaskSsbo> @ 0xd32c3c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::color_id::WallColorIdSsbo> @ 0xd32f1c -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo> @ 0xd331fc -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::water::RippleSourceSsbo> @ 0xd334dc -> 0x72edf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer> @ 0xd337bc -> 0x72edf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::import_export::impl$2::into_sampled_rg_texture
// 信息 ID: 8ef1ba6b90dec926ef678b43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x63d7de -> 0x7482c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
