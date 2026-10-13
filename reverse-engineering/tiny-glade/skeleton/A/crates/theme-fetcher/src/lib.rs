// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/theme-fetcher/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 22283c87b63a0b6501e1700818c6c280a09e419725e589381c594fb11b87baf5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 239, 258, 1838
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"PDB_registry_path_only","version":"1.7.1"}]
// use bytes; // namespace candidate
//   package identities: [{"name":"bytes","source":"PDB_registry_path_only","version":"1.6.0"}]
// use core; // namespace candidate
//   package identities: []
// use http; // namespace candidate
//   package identities: [{"name":"http","source":"PDB_registry_path_only","version":"0.2.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use reqwest; // namespace candidate
//   package identities: [{"name":"reqwest","source":"PDB_registry_path_only","version":"0.11.22"}]
// use std; // namespace candidate
//   package identities: []
// use theme_defs; // namespace candidate
//   package identities: []
// use theme_fetcher; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":239,"record_offset":25792,"rva":3796960,"va":5372506080}
// 编译实例 RVAs: 0x39efe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403647a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-239840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e82057fcff","size":5,"soft_fail":false,"target_va":5372266400,"va":5372506235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038d3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-73467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e805e1feff","size":5,"soft_fail":false,"target_va":5372433360,"va":5372506822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t243321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e879b60300","size":5,"soft_fail":false,"target_va":5372749696,"va":5372506370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t243481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e819b70300","size":5,"soft_fail":false,"target_va":5372749712,"va":5372506226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2664902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e8c6a92800","size":5,"soft_fail":false,"target_va":5375171056,"va":5372506149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30471533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e86df5d001","size":5,"soft_fail":false,"target_va":5402978144,"va":5372506606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-846494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e86215f3ff","size":5,"soft_fail":false,"target_va":5371660288,"va":5372506777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30443708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e8bc88d001","size":5,"soft_fail":false,"target_va":5402949904,"va":5372506191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<theme_server_proto::EventsResponse,alloc::string::String> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-263659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e815fafbff","size":5,"soft_fail":false,"target_va":5372243104,"va":5372506758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::Client::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1020823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e897930f00","size":5,"soft_fail":false,"target_va":5373527104,"va":5372506276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4902576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506080,"raw_hex":"e8b0ce4a00","size":5,"soft_fail":false,"target_va":5377408752,"va":5372506171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":239,"record_offset":26600,"parent_record_offset":25792,"ancestor_record_offset":25792,"inlinee":11660,"source_line":691,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":239,"record_offset":25944,"parent_record_offset":25792,"ancestor_record_offset":25792,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":239,"record_offset":26308,"parent_record_offset":25792,"ancestor_record_offset":25792,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":239,"record_offset":26280,"parent_record_offset":25792,"ancestor_record_offset":25792,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":239,"record_offset":26000,"parent_record_offset":25792,"ancestor_record_offset":25792,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":25792,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26788,"parent_record_offset":26764,"ancestor_record_offset":25792,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":25792,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26708,"parent_record_offset":26672,"ancestor_record_offset":25792,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26740,"parent_record_offset":26708,"ancestor_record_offset":25792,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":239,"record_offset":26516,"parent_record_offset":26460,"ancestor_record_offset":25792,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":239,"record_offset":26672,"parent_record_offset":26636,"ancestor_record_offset":25792,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":239,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":25792,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":239,"record_offset":26460,"parent_record_offset":26432,"ancestor_record_offset":25792,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":239,"record_offset":27292,"parent_record_offset":27256,"ancestor_record_offset":25792,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":239,"record_offset":26636,"parent_record_offset":26600,"ancestor_record_offset":25792,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":239,"record_offset":27256,"parent_record_offset":27232,"ancestor_record_offset":25792,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":239,"record_offset":26052,"parent_record_offset":26000,"ancestor_record_offset":25792,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$4::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":239,"record_offset":25968,"parent_record_offset":25944,"ancestor_record_offset":25792,"inlinee":5477,"source_line":768,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":239,"record_offset":26328,"parent_record_offset":26308,"ancestor_record_offset":25792,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":239,"record_offset":27324,"parent_record_offset":27292,"ancestor_record_offset":25792,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":239,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":25792,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":239,"record_offset":27352,"parent_record_offset":27292,"ancestor_record_offset":25792,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":239,"record_offset":27384,"parent_record_offset":27256,"ancestor_record_offset":25792,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":239,"record_offset":26148,"parent_record_offset":26124,"ancestor_record_offset":25792,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":239,"record_offset":26196,"parent_record_offset":26172,"ancestor_record_offset":25792,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":239,"record_offset":26172,"parent_record_offset":26148,"ancestor_record_offset":25792,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":239,"record_offset":26248,"parent_record_offset":26000,"ancestor_record_offset":25792,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":239,"record_offset":26024,"parent_record_offset":26000,"ancestor_record_offset":25792,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":239,"record_offset":26124,"parent_record_offset":26100,"ancestor_record_offset":25792,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":239,"record_offset":26100,"parent_record_offset":26076,"ancestor_record_offset":25792,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":239,"record_offset":26076,"parent_record_offset":26052,"ancestor_record_offset":25792,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::fetch_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x361E
// 首条位置: {"module":258,"inlinee":13854}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":15860,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":14104,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":12066,"source_line":1776,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$37::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":15004,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":8005,"source_line":2639,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":17884,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18144,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":13888,"source_line":461,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":14752,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":16112,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":15804,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":14780,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":17800,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":16592,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":16388,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":17512,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":14132,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":14168,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":17716,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14440,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":13860,"source_line":676,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":14808,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":14668,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":13800,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":13800,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":13800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14276,"parent_record_offset":14240,"ancestor_record_offset":13800,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14308,"parent_record_offset":14276,"ancestor_record_offset":13800,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":16520,"parent_record_offset":16464,"ancestor_record_offset":13800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":14240,"parent_record_offset":14204,"ancestor_record_offset":13800,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$69::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":14204,"parent_record_offset":14168,"ancestor_record_offset":13800,"inlinee":13856,"source_line":2271,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":13800,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":16492,"parent_record_offset":16464,"ancestor_record_offset":13800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":13800,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15324,"parent_record_offset":15296,"ancestor_record_offset":13800,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15564,"parent_record_offset":15324,"ancestor_record_offset":13800,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":16464,"parent_record_offset":16436,"ancestor_record_offset":13800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15256,"parent_record_offset":15220,"ancestor_record_offset":13800,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15220,"parent_record_offset":15184,"ancestor_record_offset":13800,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15148,"parent_record_offset":15112,"ancestor_record_offset":13800,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15184,"parent_record_offset":15148,"ancestor_record_offset":13800,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15112,"parent_record_offset":15076,"ancestor_record_offset":13800,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":258,"record_offset":15076,"parent_record_offset":15040,"ancestor_record_offset":13800,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":15040,"parent_record_offset":15004,"ancestor_record_offset":13800,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":13800,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":13800,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":13800,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":13800,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14564,"parent_record_offset":14468,"ancestor_record_offset":13800,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":17824,"parent_record_offset":17800,"ancestor_record_offset":13800,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":18944,"parent_record_offset":18920,"ancestor_record_offset":13800,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":13800,"inlinee":13889,"source_line":466,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":13800,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":18312,"parent_record_offset":18168,"ancestor_record_offset":13800,"inlinee":9140,"source_line":121,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":18256,"parent_record_offset":18168,"ancestor_record_offset":13800,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytes","version":"1.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":13800,"inlinee":13369,"source_line":546,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":15388,"parent_record_offset":15352,"ancestor_record_offset":13800,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":13800,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":258,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":13800,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":258,"record_offset":17848,"parent_record_offset":17824,"ancestor_record_offset":13800,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":18484,"parent_record_offset":18144,"ancestor_record_offset":13800,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":19336,"parent_record_offset":19308,"ancestor_record_offset":13800,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":13800,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":13800,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":15604,"parent_record_offset":15256,"ancestor_record_offset":13800,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":13800,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":13800,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":13800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":258,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":13800,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":14384,"parent_record_offset":14356,"ancestor_record_offset":13800,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":13800,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":13800,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14620,"parent_record_offset":14592,"ancestor_record_offset":13800,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":258,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":13800,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18200,"parent_record_offset":18168,"ancestor_record_offset":13800,"inlinee":12880,"source_line":1493,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":13800,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":14692,"parent_record_offset":14668,"ancestor_record_offset":13800,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":13800,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":13800,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":13800,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18624,"parent_record_offset":18532,"ancestor_record_offset":13800,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14468,"parent_record_offset":14440,"ancestor_record_offset":13800,"inlinee":13861,"source_line":683,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14508,"parent_record_offset":14468,"ancestor_record_offset":13800,"inlinee":13863,"source_line":3422,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::fetch_events::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12B1D
// 首条位置: {"module":1838,"inlinee":76573}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":1838,"record_offset":1200,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":11660,"source_line":691,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1838,"record_offset":1452,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":1140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":1140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":1140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":1140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1838,"record_offset":1584,"parent_record_offset":1528,"ancestor_record_offset":1140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1838,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":1140,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1838,"record_offset":1556,"parent_record_offset":1528,"ancestor_record_offset":1140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1838,"record_offset":1528,"parent_record_offset":1500,"ancestor_record_offset":1140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1838,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":1140,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1838,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":1140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1838,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":1140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01838.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::download_blob
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x362F
// 首条位置: {"module":258,"inlinee":13871}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":27188,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":30356,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$82::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":30724,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":13891,"source_line":2325,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] anyhow::Error::msg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28924,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":13722,"source_line":79,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":27020,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":31120,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":27048,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":28340,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":27916,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":27952,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":27076,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":26936,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27272,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":13873,"source_line":892,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":26764,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":26624,"parent_record_offset":26392,"ancestor_record_offset":26392,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":28196,"parent_record_offset":28172,"ancestor_record_offset":26392,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":28220,"parent_record_offset":28196,"ancestor_record_offset":26392,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27660,"parent_record_offset":27632,"ancestor_record_offset":26392,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":28140,"parent_record_offset":28104,"ancestor_record_offset":26392,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":28172,"parent_record_offset":28140,"ancestor_record_offset":26392,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27632,"parent_record_offset":27572,"ancestor_record_offset":26392,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":28104,"parent_record_offset":28068,"ancestor_record_offset":26392,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":29572,"parent_record_offset":29544,"ancestor_record_offset":26392,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":26392,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":27600,"parent_record_offset":27572,"ancestor_record_offset":26392,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":26392,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":30408,"parent_record_offset":30356,"ancestor_record_offset":26392,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":32116,"parent_record_offset":32076,"ancestor_record_offset":26392,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":27816,"parent_record_offset":27760,"ancestor_record_offset":26392,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":29964,"parent_record_offset":29940,"ancestor_record_offset":26392,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28068,"parent_record_offset":28032,"ancestor_record_offset":26392,"inlinee":13723,"source_line":237,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_adhoc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28960,"parent_record_offset":28924,"ancestor_record_offset":26392,"inlinee":13719,"source_line":114,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28032,"parent_record_offset":27988,"ancestor_record_offset":26392,"inlinee":13720,"source_line":88,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":27988,"parent_record_offset":27952,"ancestor_record_offset":26392,"inlinee":13725,"source_line":564,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytes","version":"1.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":26392,"inlinee":13369,"source_line":546,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":29600,"parent_record_offset":29572,"ancestor_record_offset":26392,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":30752,"parent_record_offset":30724,"ancestor_record_offset":26392,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":26392,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":30436,"parent_record_offset":30356,"ancestor_record_offset":26392,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":29624,"parent_record_offset":29600,"ancestor_record_offset":26392,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":26392,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":27476,"parent_record_offset":27440,"ancestor_record_offset":26392,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":32152,"parent_record_offset":32116,"ancestor_record_offset":26392,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":28248,"parent_record_offset":28220,"ancestor_record_offset":26392,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":30464,"parent_record_offset":30356,"ancestor_record_offset":26392,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":27344,"parent_record_offset":27304,"ancestor_record_offset":26392,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":30836,"parent_record_offset":30808,"ancestor_record_offset":26392,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":26392,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":26392,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27840,"parent_record_offset":27816,"ancestor_record_offset":26392,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":30000,"parent_record_offset":29964,"ancestor_record_offset":26392,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":26392,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":26392,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":26984,"parent_record_offset":26960,"ancestor_record_offset":26392,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":30024,"parent_record_offset":30000,"ancestor_record_offset":26392,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":30056,"parent_record_offset":29964,"ancestor_record_offset":26392,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":258,"record_offset":27440,"parent_record_offset":27408,"ancestor_record_offset":26392,"inlinee":13880,"source_line":43,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27304,"parent_record_offset":27272,"ancestor_record_offset":26392,"inlinee":13874,"source_line":949,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27732,"parent_record_offset":27304,"ancestor_record_offset":26392,"inlinee":13882,"source_line":49,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27760,"parent_record_offset":27732,"ancestor_record_offset":26392,"inlinee":13884,"source_line":987,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27788,"parent_record_offset":27760,"ancestor_record_offset":26392,"inlinee":13886,"source_line":1157,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27380,"parent_record_offset":27344,"ancestor_record_offset":26392,"inlinee":13876,"source_line":950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":258,"record_offset":27408,"parent_record_offset":27380,"ancestor_record_offset":26392,"inlinee":13878,"source_line":37,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":26392,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":26720,"parent_record_offset":26696,"ancestor_record_offset":26392,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":26696,"parent_record_offset":26672,"ancestor_record_offset":26392,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":26392,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::trim_cas
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3618
// 首条位置: {"module":258,"inlinee":13848}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":258,"record_offset":4832,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":4476,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":4504,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":11172,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":4940,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":4532,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":4392,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":4560,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":4968,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":13849,"source_line":220,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5760,"parent_record_offset":5704,"ancestor_record_offset":4260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":4260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5704,"parent_record_offset":5676,"ancestor_record_offset":4260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":11652,"parent_record_offset":11612,"ancestor_record_offset":4260,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":11612,"parent_record_offset":11584,"ancestor_record_offset":4260,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":4900,"parent_record_offset":4864,"ancestor_record_offset":4260,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":258,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":4260,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6176,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":13852,"source_line":1204,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6512,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4260,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":4260,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":4592,"parent_record_offset":4560,"ancestor_record_offset":4260,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":6540,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=104
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":5544,"parent_record_offset":5340,"ancestor_record_offset":4260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":11704,"parent_record_offset":11652,"ancestor_record_offset":4260,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":4788,"parent_record_offset":4560,"ancestor_record_offset":4260,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5100,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5136,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":4260,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":4260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":11736,"parent_record_offset":11652,"ancestor_record_offset":4260,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":11772,"parent_record_offset":11612,"ancestor_record_offset":4260,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$30::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":258,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":4260,"inlinee":4210,"source_line":92,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":6568,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":6428,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":4260,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4260,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":4260,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4260,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5164,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":13851,"source_line":1193,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":6868,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":13853,"source_line":2389,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5340,"parent_record_offset":4968,"ancestor_record_offset":4260,"inlinee":5858,"source_line":1887,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4260,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4260,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4260,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":4260,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":4260,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3619
// 首条位置: {"module":258,"inlinee":13849}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6176,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":13852,"source_line":1204,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6512,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":6540,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":6596,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5100,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5136,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":6568,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":6428,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5164,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":13851,"source_line":1193,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":6868,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":13853,"source_line":2389,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5340,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":5858,"source_line":1887,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":6288,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4968,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5760,"parent_record_offset":5704,"ancestor_record_offset":4968,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":4968,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5704,"parent_record_offset":5676,"ancestor_record_offset":4968,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4968,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":4968,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=75
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":5544,"parent_record_offset":5340,"ancestor_record_offset":4968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":4968,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":4968,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$30::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":258,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":4968,"inlinee":4210,"source_line":92,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":4968,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":4968,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":4968,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4968,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":4968,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":4968,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":5376,"parent_record_offset":5340,"ancestor_record_offset":4968,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":4968,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":4968,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":258,"record_offset":4052,"rva":4040784,"va":5372749904}
// 编译实例 RVAs: 0x3da850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$GT$$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4764799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e87fb44800","size":5,"soft_fail":false,"target_va":5377527680,"va":5372762876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4772535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8b7d24800","size":5,"soft_fail":false,"target_va":5377527504,"va":5372754964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$reqwest..into_url..IntoUrlSealed$GT$::into_url
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4922757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8851d4b00","size":5,"soft_fail":false,"target_va":5377679760,"va":5372756998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5090369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e841ac4d00","size":5,"soft_fail":false,"target_va":5377853984,"va":5372763610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..debt..list..LocalNode$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4146782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e85e463f00","size":5,"soft_fail":false,"target_va":5376902336,"va":5372755549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..strategy..hybrid..HybridStrategy$LT$Cfg$GT$$u20$as$u20$arc_swap..strategy..sealed..InnerStrategy$LT$T$GT$$GT$::load::_$u7b$$u7b$closure$u7d$$u7d$::h81ab2ec7c1d306dc (.llvm.501439338515271349)
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5010062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e88e724c00","size":5,"soft_fail":false,"target_va":5377765536,"va":5372755469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5061905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8113d4d00","size":5,"soft_fail":false,"target_va":5377821664,"va":5372759754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4657508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e864114700","size":5,"soft_fail":false,"target_va":5377408080,"va":5372750567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4713173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d5ea4700","size":5,"soft_fail":false,"target_va":5377465440,"va":5372752262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..time..SystemTime$u20$as$u20$core..cmp..PartialOrd$GT$::partial_cmp
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4755169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8e18e4800","size":5,"soft_fail":false,"target_va":5377510528,"va":5372755354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4690989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e82d944700","size":5,"soft_fail":false,"target_va":5377444128,"va":5372753134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4036447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8a168c2ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5372768698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3910671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8f153c4ff","size":5,"soft_fail":false,"target_va":5368859888,"va":5372770554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3123191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e80958d0ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5372755634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140107100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2982558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8627dd2ff","size":5,"soft_fail":false,"target_va":5369786624,"va":5372769177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010cbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2962219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d5ccd2ff","size":5,"soft_fail":false,"target_va":5369809888,"va":5372772102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa00
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1094481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8af4cefff","size":5,"soft_fail":false,"target_va":5371656704,"va":5372751180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402db2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1064151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e829c3efff","size":5,"soft_fail":false,"target_va":5371704048,"va":5372768194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-462318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e812f2f8ff","size":5,"soft_fail":false,"target_va":5372309344,"va":5372771657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-436345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e88757f9ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372750548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-447438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8322cf9ff","size":5,"soft_fail":false,"target_va":5372320512,"va":5372767945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e860f3ffff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372752923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=41
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e86efbffff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372750877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=10
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2420941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8cdf02400","size":5,"soft_fail":false,"target_va":5375171056,"va":5372750110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087ee90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4847259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e89bf64900","size":5,"soft_fail":false,"target_va":5377617552,"va":5372770288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f3a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4849778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e872004a00","size":5,"soft_fail":false,"target_va":5377618848,"va":5372769065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b3490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5071716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e864634d00","size":5,"soft_fail":false,"target_va":5377832080,"va":5372760359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b4da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5075728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e810734d00","size":5,"soft_fail":false,"target_va":5377838496,"va":5372762763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30833532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e87c7bd601","size":5,"soft_fail":false,"target_va":5403593344,"va":5372759807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30842882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e802a0d601","size":5,"soft_fail":false,"target_va":5403593360,"va":5372750473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t30213457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e85105cd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372764682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4775147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ebdc4800","size":5,"soft_fail":false,"target_va":5377529888,"va":5372754736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4762187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e84baa4800","size":5,"soft_fail":false,"target_va":5377527120,"va":5372764928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t4862127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8af304a00","size":5,"soft_fail":false,"target_va":5377617360,"va":5372755228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t30190728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e888accc01","size":5,"soft_fail":false,"target_va":5402950624,"va":5372759891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::haa3d630f77335115 (.llvm.923038179807553715)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30231650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8624ccd01","size":5,"soft_fail":false,"target_va":5402981936,"va":5372750281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t30232076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e80c4ecd01","size":5,"soft_fail":false,"target_va":5402982752,"va":5372750671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4605790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e85e474600","size":5,"soft_fail":false,"target_va":5377364304,"va":5372758509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t4613895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e807674600","size":5,"soft_fail":false,"target_va":5377364336,"va":5372750436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5012761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8197d4c00","size":5,"soft_fail":false,"target_va":5377767856,"va":5372755090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::Node::get
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4146305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e881443f00","size":5,"soft_fail":false,"target_va":5376901760,"va":5372755450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::THREAD_HEAD::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4146293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e875443f00","size":5,"soft_fail":false,"target_va":5376901696,"va":5372755398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5065429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d54a4d00","size":5,"soft_fail":false,"target_va":5377825072,"va":5372759638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] cache_control::CacheControl::from_value
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5007572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d4684c00","size":5,"soft_fail":false,"target_va":5377761408,"va":5372753831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5075223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e817714d00","size":5,"soft_fail":false,"target_va":5377837392,"va":5372762164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30214176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e82008cd01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372764699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30214677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8150acd01","size":5,"soft_fail":false,"target_va":5402979344,"va":5372764662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..sync..ArcInner$LT$core..result..Result$LT$std..path..PathBuf$C$theme_defs..EventDataError$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4847362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e802f74900","size":5,"soft_fail":false,"target_va":5377618752,"va":5372771385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$theme_fetcher..MaybePendingActiveEvent$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4864363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e86b394a00","size":5,"soft_fail":false,"target_va":5377632432,"va":5372768064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$anyhow..error..ErrorImpl$LT$reqwest..error..Error$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4860395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8eb294a00","size":5,"soft_fail":false,"target_va":5377629984,"va":5372769584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..async_impl..response..Response$GT$::h76f654618bf936e2 (.llvm.2483409784974327053)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4868780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ac4a4a00","size":5,"soft_fail":false,"target_va":5377628192,"va":5372759407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..blocking..response..Response$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4871330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8a2544a00","size":5,"soft_fail":false,"target_va":5377626320,"va":5372754985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..error..Error$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4855026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8f2144a00","size":5,"soft_fail":false,"target_va":5377622608,"va":5372767577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..ThemeFetcher$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4858554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ba224a00","size":5,"soft_fail":false,"target_va":5377623152,"va":5372764593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8debbffff","size":5,"soft_fail":false,"target_va":5372749776,"va":5372767213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4039303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8795dc2ff","size":5,"soft_fail":false,"target_va":5368732208,"va":5372771506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30215376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d00ccd01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372764747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap$LT$T$GT$::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5005854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e81e624c00","size":5,"soft_fail":false,"target_va":5377759632,"va":5372753773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5012521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8297c4c00","size":5,"soft_fail":false,"target_va":5377769584,"va":5372757058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::name::HdrName::from_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5000620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ac4d4c00","size":5,"soft_fail":false,"target_va":5377754048,"va":5372753423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::value::HeaderValue::to_str
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5001689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8d9514c00","size":5,"soft_fail":false,"target_va":5377755200,"va":5372753506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] httpdate::parse_http_date
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5003369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e869584c00","size":5,"soft_fail":false,"target_va":5377756928,"va":5372753554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t4603693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e82d3f4600","size":5,"soft_fail":false,"target_va":5377353776,"va":5372750078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::Client::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4885828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8448d4a00","size":5,"soft_fail":false,"target_va":5377638576,"va":5372752743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::header
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4886537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e809904a00","size":5,"soft_fail":false,"target_va":5377639376,"va":5372752834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4964344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8f8bf4b00","size":5,"soft_fail":false,"target_va":5377721920,"va":5372757571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::send
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4972805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e805e14b00","size":5,"soft_fail":false,"target_va":5377725680,"va":5372752870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::bytes
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5060263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8a7364d00","size":5,"soft_fail":false,"target_va":5377818512,"va":5372758244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::content_length
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5060230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e886364d00","size":5,"soft_fail":false,"target_va":5377818416,"va":5372758181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4645369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8f9e14600","size":5,"soft_fail":false,"target_va":5377395552,"va":5372750178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4657754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e85a124700","size":5,"soft_fail":false,"target_va":5377408720,"va":5372750961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4657941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e815134700","size":5,"soft_fail":false,"target_va":5377408592,"va":5372750646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4657391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ef104700","size":5,"soft_fail":false,"target_va":5377408224,"va":5372750828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4657253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e865104700","size":5,"soft_fail":false,"target_va":5377408048,"va":5372750790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4683388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e87c764700","size":5,"soft_fail":false,"target_va":5377439776,"va":5372756383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4679348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8b4664700","size":5,"soft_fail":false,"target_va":5377440784,"va":5372761431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4684698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e89a7b4700","size":5,"soft_fail":false,"target_va":5377441120,"va":5372756417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5075200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e800714d00","size":5,"soft_fail":false,"target_va":5377836912,"va":5372761707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4718091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e80bfe4700","size":5,"soft_fail":false,"target_va":5377468224,"va":5372750128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4718757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8a5004800","size":5,"soft_fail":false,"target_va":5377470368,"va":5372751606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4718831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8ef004800","size":5,"soft_fail":false,"target_va":5377469680,"va":5372750844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h09b851786680b0c0 (.llvm.10017346168270275662)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5013020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e81c7e4c00","size":5,"soft_fail":false,"target_va":5377768448,"va":5372755423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4640536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e818cf4600","size":5,"soft_fail":false,"target_va":5377393200,"va":5372752659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4690543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e86f924700","size":5,"soft_fail":false,"target_va":5377442944,"va":5372752396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::SystemTime::duration_since
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4693382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8869d4700","size":5,"soft_fail":false,"target_va":5377444656,"va":5372751269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::SystemTime::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4694520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8f8a14700","size":5,"soft_fail":false,"target_va":5377444608,"va":5372750083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] theme_fetcher::checked_zip::verify_folder_checksums
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4882873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8b9814a00","size":5,"soft_fail":false,"target_va":5377634224,"va":5372751346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::_$LT$impl$u20$zip..read..zip_archive..ZipArchive$LT$R$GT$$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5068728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372749904,"raw_hex":"e8b8574d00","size":5,"soft_fail":false,"target_va":5377828944,"va":5372760211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t*32(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372749904,"raw_hex":"ff5020","size":3,"soft_fail":false,"va":5372753272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":20124,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":33848,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":25692,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":21648,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":13864,"source_line":453,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":34080,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":13888,"source_line":461,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":35312,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":13632,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":13460,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":22800,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":37764,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":13488,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":13544,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":25720,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":24884,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":33332,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":23552,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4720,"source_line":1680,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":21704,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":258,"record_offset":21788,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":13516,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":13376,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":258,"record_offset":37400,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::download_blob
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":26392,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":13871,"source_line":158,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::fetch_events
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":13800,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":13854,"source_line":53,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::trim_cas
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":4260,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":13848,"source_line":198,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":4052,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":4052,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4052,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14276,"parent_record_offset":14240,"ancestor_record_offset":4052,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":14308,"parent_record_offset":14276,"ancestor_record_offset":4052,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":5760,"parent_record_offset":5704,"ancestor_record_offset":4052,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":14240,"parent_record_offset":14204,"ancestor_record_offset":4052,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$35::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":20232,"parent_record_offset":20124,"ancestor_record_offset":4052,"inlinee":13858,"source_line":1909,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$69::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":14204,"parent_record_offset":14168,"ancestor_record_offset":4052,"inlinee":13856,"source_line":2271,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":21332,"parent_record_offset":21304,"ancestor_record_offset":4052,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":15860,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":4052,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":4052,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":4052,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15324,"parent_record_offset":15296,"ancestor_record_offset":4052,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":15564,"parent_record_offset":15324,"ancestor_record_offset":4052,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":5704,"parent_record_offset":5676,"ancestor_record_offset":4052,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15256,"parent_record_offset":15220,"ancestor_record_offset":4052,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15220,"parent_record_offset":15184,"ancestor_record_offset":4052,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15148,"parent_record_offset":15112,"ancestor_record_offset":4052,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15184,"parent_record_offset":15148,"ancestor_record_offset":4052,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":15112,"parent_record_offset":15076,"ancestor_record_offset":4052,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":258,"record_offset":15076,"parent_record_offset":15040,"ancestor_record_offset":4052,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":14104,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":12066,"source_line":1776,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":20176,"parent_record_offset":20124,"ancestor_record_offset":4052,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":30356,"parent_record_offset":26392,"ancestor_record_offset":4052,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$37::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":15004,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":8005,"source_line":2639,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":15040,"parent_record_offset":15004,"ancestor_record_offset":4052,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":35436,"parent_record_offset":35404,"ancestor_record_offset":4052,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$82::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":30724,"parent_record_offset":26392,"ancestor_record_offset":4052,"inlinee":13891,"source_line":2325,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":4052,"inlinee":9129,"source_line":1510,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":4052,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":4052,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":11652,"parent_record_offset":11612,"ancestor_record_offset":4052,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":4052,"inlinee":9128,"source_line":1485,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":17884,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":27816,"parent_record_offset":27760,"ancestor_record_offset":4052,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":11612,"parent_record_offset":11584,"ancestor_record_offset":4052,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":4052,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":23740,"parent_record_offset":23708,"ancestor_record_offset":4052,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14564,"parent_record_offset":14468,"ancestor_record_offset":4052,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":17824,"parent_record_offset":17800,"ancestor_record_offset":4052,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":18944,"parent_record_offset":18920,"ancestor_record_offset":4052,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":258,"record_offset":35464,"parent_record_offset":35436,"ancestor_record_offset":4052,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28068,"parent_record_offset":28032,"ancestor_record_offset":4052,"inlinee":13723,"source_line":237,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_adhoc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28960,"parent_record_offset":28924,"ancestor_record_offset":4052,"inlinee":13719,"source_line":114,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":4900,"parent_record_offset":4864,"ancestor_record_offset":4052,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28032,"parent_record_offset":27988,"ancestor_record_offset":4052,"inlinee":13720,"source_line":88,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::msg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":28924,"parent_record_offset":26392,"ancestor_record_offset":4052,"inlinee":13722,"source_line":79,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":258,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":4052,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":258,"record_offset":4832,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":258,"record_offset":27988,"parent_record_offset":27952,"ancestor_record_offset":4052,"inlinee":13725,"source_line":564,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18144,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":13888,"source_line":461,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":4052,"inlinee":13889,"source_line":466,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":22412,"parent_record_offset":21936,"ancestor_record_offset":4052,"inlinee":9124,"source_line":65,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":4052,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::LocalNode::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":22928,"parent_record_offset":22892,"ancestor_record_offset":4052,"inlinee":10952,"source_line":223,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::THREAD_HEAD::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":258,"record_offset":22996,"parent_record_offset":22968,"ancestor_record_offset":4052,"inlinee":10944,"source_line":85,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":23144,"parent_record_offset":22968,"ancestor_record_offset":4052,"inlinee":10946,"source_line":226,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":23388,"parent_record_offset":23348,"ancestor_record_offset":4052,"inlinee":10947,"source_line":239,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":22328,"parent_record_offset":21936,"ancestor_record_offset":4052,"inlinee":9127,"source_line":96,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":18312,"parent_record_offset":18168,"ancestor_record_offset":4052,"inlinee":9140,"source_line":121,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":4052,"inlinee":9121,"source_line":104,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":21672,"parent_record_offset":21648,"ancestor_record_offset":4052,"inlinee":9133,"source_line":186,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":18256,"parent_record_offset":18168,"ancestor_record_offset":4052,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bytes","version":"1.6.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":4052,"inlinee":13369,"source_line":546,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":15388,"parent_record_offset":15352,"ancestor_record_offset":4052,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":21364,"parent_record_offset":21332,"ancestor_record_offset":4052,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":4052,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":258,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":4052,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":258,"record_offset":23228,"parent_record_offset":23144,"ancestor_record_offset":4052,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":258,"record_offset":24184,"parent_record_offset":23708,"ancestor_record_offset":4052,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":30752,"parent_record_offset":30724,"ancestor_record_offset":4052,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6176,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":13852,"source_line":1204,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":4476,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4052,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":4052,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":4052,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":4592,"parent_record_offset":4560,"ancestor_record_offset":4052,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":16112,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":4504,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":6540,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":20204,"parent_record_offset":20124,"ancestor_record_offset":4052,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":258,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":4052,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":258,"record_offset":36364,"parent_record_offset":36332,"ancestor_record_offset":4052,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":258,"record_offset":17848,"parent_record_offset":17824,"ancestor_record_offset":4052,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":17800,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":36332,"parent_record_offset":36012,"ancestor_record_offset":4052,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":18484,"parent_record_offset":18144,"ancestor_record_offset":4052,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":258,"record_offset":21976,"parent_record_offset":21936,"ancestor_record_offset":4052,"inlinee":9125,"source_line":140,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":4052,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":21388,"parent_record_offset":21364,"ancestor_record_offset":4052,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":19336,"parent_record_offset":19308,"ancestor_record_offset":4052,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":23072,"parent_record_offset":23048,"ancestor_record_offset":4052,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":23172,"parent_record_offset":23144,"ancestor_record_offset":4052,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":23196,"parent_record_offset":23172,"ancestor_record_offset":4052,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":4052,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":16592,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":22212,"parent_record_offset":21936,"ancestor_record_offset":4052,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":23440,"parent_record_offset":23388,"ancestor_record_offset":4052,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":258,"record_offset":36096,"parent_record_offset":36060,"ancestor_record_offset":4052,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18384,"parent_record_offset":18360,"ancestor_record_offset":4052,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":15604,"parent_record_offset":15256,"ancestor_record_offset":4052,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18432,"parent_record_offset":18408,"ancestor_record_offset":4052,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":4052,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=227
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":4940,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":258,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":4052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":11704,"parent_record_offset":11652,"ancestor_record_offset":4052,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":21844,"parent_record_offset":21788,"ancestor_record_offset":4052,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":22264,"parent_record_offset":22236,"ancestor_record_offset":4052,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":14384,"parent_record_offset":14356,"ancestor_record_offset":4052,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":22292,"parent_record_offset":22236,"ancestor_record_offset":4052,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":20260,"parent_record_offset":20124,"ancestor_record_offset":4052,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":22496,"parent_record_offset":22412,"ancestor_record_offset":4052,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":4788,"parent_record_offset":4560,"ancestor_record_offset":4052,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":17512,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":4052,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":23348,"parent_record_offset":22928,"ancestor_record_offset":4052,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5100,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":5136,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$32::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":35352,"parent_record_offset":35312,"ancestor_record_offset":4052,"inlinee":5218,"source_line":498,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":30836,"parent_record_offset":30808,"ancestor_record_offset":4052,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":4052,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":17716,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":4052,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14620,"parent_record_offset":14592,"ancestor_record_offset":4052,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":258,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":4052,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":4052,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18200,"parent_record_offset":18168,"ancestor_record_offset":4052,"inlinee":12880,"source_line":1493,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":22440,"parent_record_offset":22412,"ancestor_record_offset":4052,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27840,"parent_record_offset":27816,"ancestor_record_offset":4052,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":4052,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":4052,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":4052,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":22464,"parent_record_offset":22440,"ancestor_record_offset":4052,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":4052,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":11736,"parent_record_offset":11652,"ancestor_record_offset":4052,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":4052,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":11772,"parent_record_offset":11612,"ancestor_record_offset":4052,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$30::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":258,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":4052,"inlinee":4210,"source_line":92,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14440,"parent_record_offset":13800,"ancestor_record_offset":4052,"inlinee":13860,"source_line":676,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14468,"parent_record_offset":14440,"ancestor_record_offset":4052,"inlinee":13861,"source_line":683,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14508,"parent_record_offset":14468,"ancestor_record_offset":4052,"inlinee":13863,"source_line":3422,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":258,"record_offset":4532,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":258,"record_offset":4392,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":258,"record_offset":27440,"parent_record_offset":27408,"ancestor_record_offset":4052,"inlinee":13880,"source_line":43,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27272,"parent_record_offset":26392,"ancestor_record_offset":4052,"inlinee":13873,"source_line":892,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27304,"parent_record_offset":27272,"ancestor_record_offset":4052,"inlinee":13874,"source_line":949,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27732,"parent_record_offset":27304,"ancestor_record_offset":4052,"inlinee":13882,"source_line":49,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27760,"parent_record_offset":27732,"ancestor_record_offset":4052,"inlinee":13884,"source_line":987,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27788,"parent_record_offset":27760,"ancestor_record_offset":4052,"inlinee":13886,"source_line":1157,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27380,"parent_record_offset":27344,"ancestor_record_offset":4052,"inlinee":13876,"source_line":950,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":258,"record_offset":27408,"parent_record_offset":27380,"ancestor_record_offset":4052,"inlinee":13878,"source_line":37,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":23652,"parent_record_offset":23624,"ancestor_record_offset":4052,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":4052,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4052,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":4052,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":4052,"inlinee":8480,"source_line":1597,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5164,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":13851,"source_line":1193,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":4560,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":8478,"source_line":2465,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":6868,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":13853,"source_line":2389,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":258,"record_offset":5340,"parent_record_offset":4968,"ancestor_record_offset":4052,"inlinee":5858,"source_line":1887,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":26764,"parent_record_offset":26392,"ancestor_record_offset":4052,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4052,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36060,"parent_record_offset":36012,"ancestor_record_offset":4052,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36012,"parent_record_offset":35724,"ancestor_record_offset":4052,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":23624,"parent_record_offset":23552,"ancestor_record_offset":4052,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4052,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":35724,"parent_record_offset":35572,"ancestor_record_offset":4052,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4052,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":4052,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36188,"parent_record_offset":36060,"ancestor_record_offset":4052,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":258,"record_offset":36220,"parent_record_offset":36012,"ancestor_record_offset":4052,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":258,"record_offset":23680,"parent_record_offset":23652,"ancestor_record_offset":4052,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":4052,"inlinee":8482,"source_line":1026,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":258,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":4052,"inlinee":8483,"source_line":1032,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":258,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":4052,"inlinee":4208,"source_line":15,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$2::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":258,"record_offset":35636,"parent_record_offset":35612,"ancestor_record_offset":4052,"inlinee":13901,"source_line":111,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":258,"record_offset":23020,"parent_record_offset":22996,"ancestor_record_offset":4052,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":258,"record_offset":23048,"parent_record_offset":23020,"ancestor_record_offset":4052,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":258,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":4052,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":258,"record_offset":22968,"parent_record_offset":22928,"ancestor_record_offset":4052,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":258,"record_offset":13656,"parent_record_offset":13632,"ancestor_record_offset":4052,"inlinee":4206,"source_line":153,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$29::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":258,"record_offset":35612,"parent_record_offset":35572,"ancestor_record_offset":4052,"inlinee":13899,"source_line":246,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":258,"record_offset":22768,"parent_record_offset":22744,"ancestor_record_offset":4052,"inlinee":13866,"source_line":246,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":23592,"parent_record_offset":23552,"ancestor_record_offset":4052,"inlinee":13868,"source_line":3,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":35404,"parent_record_offset":35352,"ancestor_record_offset":4052,"inlinee":13893,"source_line":3,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":36652,"parent_record_offset":36596,"ancestor_record_offset":4052,"inlinee":13895,"source_line":10,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":35572,"parent_record_offset":35352,"ancestor_record_offset":4052,"inlinee":13897,"source_line":10,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":4968,"parent_record_offset":4260,"ancestor_record_offset":4052,"inlinee":13849,"source_line":220,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":33380,"parent_record_offset":33332,"ancestor_record_offset":4052,"inlinee":13887,"source_line":286,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":4052,"inlinee":13870,"source_line":297,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x363F
// 首条位置: {"module":258,"inlinee":13887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":33416,"parent_record_offset":33380,"ancestor_record_offset":33380,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":33784,"parent_record_offset":33380,"ancestor_record_offset":33380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":33728,"parent_record_offset":33700,"ancestor_record_offset":33380,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":33700,"parent_record_offset":33636,"ancestor_record_offset":33380,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":33668,"parent_record_offset":33636,"ancestor_record_offset":33380,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":33636,"parent_record_offset":33608,"ancestor_record_offset":33380,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":33468,"parent_record_offset":33416,"ancestor_record_offset":33380,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":33496,"parent_record_offset":33416,"ancestor_record_offset":33380,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":33560,"parent_record_offset":33416,"ancestor_record_offset":33380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":258,"record_offset":33524,"parent_record_offset":33416,"ancestor_record_offset":33380,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x362E
// 首条位置: {"module":258,"inlinee":13870}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::MaybePendingActiveEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::ArcSwapAny", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<theme_fetcher::MaybePendingActiveEvent,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "theme_fetcher::impl$0::fetch_events::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::ThemeFetcher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::ThemeFetcher>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::ThemeFetcher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "theme_fetcher::ThemeFetcher::new<std::path::PathBuf>", "constructor": "theme_fetcher::ThemeFetcher::new", "arguments": ["std::path::PathBuf"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::download_blob
// 信息 ID: a7c0cb725ea2ceee0a559bc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=26392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::fetch_events
// 信息 ID: faae9e882b7a0740b73bad66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=13800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 信息 ID: 26178abd7acf2e9eec4ab18e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] theme_fetcher::ThemeFetcher::new -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x2f5491 -> 0x39efe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::run
// 信息 ID: 2afa18d9480792cebc0722d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > @ 0x1ac51 -> 0x3da850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::trim_cas
// 信息 ID: e9578d70e860d479ebe0b96b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=4260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 信息 ID: 42cd92d2143110bdf0320c01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=4968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::fetch_events::closure$0
// 信息 ID: c91eadcf7e15b68c4a5ecdcb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::fetch_events", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<theme_fetcher::MaybePendingActiveEvent,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$19::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; module=1838, inline_record=1140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$0
// 信息 ID: 6306f3976c4a4d5c947f1683
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=33380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$1
// 信息 ID: f627dce02f3165c4835419d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=24912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
