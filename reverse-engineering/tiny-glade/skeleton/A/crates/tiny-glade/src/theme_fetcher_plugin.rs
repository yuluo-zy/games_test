// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/theme_fetcher_plugin.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e0800e47bfb560f3c3b5878036612ec14603403ce9ff3c405819651f37f121e4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: theme_fetcher_plugin.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 191
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":191,"record_offset":7684,"rva":3101456,"va":5371810576}
// 编译实例 RVAs: 0x2f5310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403647a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t454132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8f4ed0600","size":5,"soft_fail":false,"target_va":5372266400,"va":5371812263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t456950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8f6f80600","size":5,"soft_fail":false,"target_va":5372269200,"va":5371812245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036a2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t478007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8374b0700","size":5,"soft_fail":false,"target_va":5372289744,"va":5371811732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t501888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e880a80700","size":5,"soft_fail":false,"target_va":5372314208,"va":5371812315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140373c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t517086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8dee30700","size":5,"soft_fail":false,"target_va":5372329024,"va":5371811933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t566594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e842a50800","size":5,"soft_fail":false,"target_va":5372378176,"va":5371811577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14039efe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t695114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e84a9b0a00","size":5,"soft_fail":false,"target_va":5372506080,"va":5371810961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b6680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t790438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8a60f0c00","size":5,"soft_fail":false,"target_va":5372601984,"va":5371811541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t938908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e89c530e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371810783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t937875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8934f0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371811832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140697cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3810858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e82a263a00","size":5,"soft_fail":false,"target_va":5375622320,"va":5371811457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31782654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8fef6e401","size":5,"soft_fail":false,"target_va":5403593360,"va":5371810701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31166532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e84490db01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371811607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5553170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e812bc5400","size":5,"soft_fail":false,"target_va":5377364336,"va":5371811161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-421196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8b492f9ff","size":5,"soft_fail":false,"target_va":5371390288,"va":5371811479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31168440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8b897db01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371811683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5542647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8f7925400","size":5,"soft_fail":false,"target_va":5377353776,"va":5371811124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4343268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8e4454200","size":5,"soft_fail":false,"target_va":5376154144,"va":5371810871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5628852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8b4e35500","size":5,"soft_fail":false,"target_va":5377439776,"va":5371810919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5581664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e8602b5500","size":5,"soft_fail":false,"target_va":5377393008,"va":5371811339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1134713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371810576,"raw_hex":"e887afeeff","size":5,"soft_fail":false,"target_va":5370676688,"va":5371811396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":191,"record_offset":9132,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":191,"record_offset":8296,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":191,"record_offset":8436,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":8324,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":8380,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":9088,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":191,"record_offset":8352,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":191,"record_offset":8212,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":8184,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":191,"record_offset":7856,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":7943,"source_line":525,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":191,"record_offset":8408,"parent_record_offset":7684,"ancestor_record_offset":7684,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7684,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7684,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":7684,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":8016,"parent_record_offset":7980,"ancestor_record_offset":7684,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":8048,"parent_record_offset":8016,"ancestor_record_offset":7684,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":7684,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":191,"record_offset":7980,"parent_record_offset":7948,"ancestor_record_offset":7684,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":8812,"parent_record_offset":8776,"ancestor_record_offset":7684,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":8776,"parent_record_offset":8740,"ancestor_record_offset":7684,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":8964,"parent_record_offset":8776,"ancestor_record_offset":7684,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":7684,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":8700,"parent_record_offset":8664,"ancestor_record_offset":7684,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":8664,"parent_record_offset":8628,"ancestor_record_offset":7684,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":8592,"parent_record_offset":8544,"ancestor_record_offset":7684,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":8628,"parent_record_offset":8592,"ancestor_record_offset":7684,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":8544,"parent_record_offset":8508,"ancestor_record_offset":7684,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":191,"record_offset":8508,"parent_record_offset":8472,"ancestor_record_offset":7684,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":8472,"parent_record_offset":8436,"ancestor_record_offset":7684,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":8740,"parent_record_offset":8700,"ancestor_record_offset":7684,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":191,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":7684,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":191,"record_offset":9000,"parent_record_offset":8700,"ancestor_record_offset":7684,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":7684,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":8124,"parent_record_offset":8096,"ancestor_record_offset":7684,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":7684,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7684,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":191,"record_offset":7888,"parent_record_offset":7856,"ancestor_record_offset":7684,"inlinee":7894,"source_line":51,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":191,"record_offset":7948,"parent_record_offset":7888,"ancestor_record_offset":7684,"inlinee":7898,"source_line":21,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::list::Channel::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\list.rs
//     evidence={"module":191,"record_offset":7920,"parent_record_offset":7888,"ancestor_record_offset":7684,"inlinee":7896,"source_line":160,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x158B
// 首条位置: {"module":14,"inlinee":5515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":191,"record_offset":9476,"rva":3103232,"va":5371812352}
// 编译实例 RVAs: 0x2f5a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140107100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2029209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e86709e1ff","size":5,"soft_fail":false,"target_va":5369786624,"va":5371815828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140268b10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-578621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8c32bf7ff","size":5,"soft_fail":false,"target_va":5371235088,"va":5371813704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140293820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-402589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e863dbf9ff","size":5,"soft_fail":false,"target_va":5371410464,"va":5371813048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140294f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-397963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e875edf9ff","size":5,"soft_fail":false,"target_va":5371416400,"va":5371814358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t500477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8fda20700","size":5,"soft_fail":false,"target_va":5372314208,"va":5371813726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t506115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e803b90700","size":5,"soft_fail":false,"target_va":5372320512,"va":5371814392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=2
//     target_aliases=37 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t575191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8d7c60800","size":5,"soft_fail":false,"target_va":5372390432,"va":5371815236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403843a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t583750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e846e80800","size":5,"soft_fail":false,"target_va":5372396448,"va":5371812693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=19
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t937024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8404c0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371812683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=7
//     target_aliases=28 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3358282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e84a3e3300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371812769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31131373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8ed06db01","size":5,"soft_fail":false,"target_va":5402945184,"va":5371813806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h1bde5606b9697a83 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31132925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8fd0cdb01","size":5,"soft_fail":false,"target_va":5402946352,"va":5371813422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h22a1244e8f4d2ce7 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31133575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8870fdb01","size":5,"soft_fail":false,"target_va":5402946768,"va":5371813188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8068e61a17c505da (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31135577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e85917db01","size":5,"soft_fail":false,"target_va":5402948592,"va":5371813010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5550226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e892b05400","size":5,"soft_fail":false,"target_va":5377364304,"va":5371814073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5550206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e87eb05400","size":5,"soft_fail":false,"target_va":5377364336,"va":5371814125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31165577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8898cdb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371814546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2442361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e879442500","size":5,"soft_fail":false,"target_va":5374255728,"va":5371813362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31155546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e85a65db01","size":5,"soft_fail":false,"target_va":5402968992,"va":5371813441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t31158264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8f86fdb01","size":5,"soft_fail":false,"target_va":5402970800,"va":5371812531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5582637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e82d2f5500","size":5,"soft_fail":false,"target_va":5377395552,"va":5371812910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5592237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8ad545500","size":5,"soft_fail":false,"target_va":5377405072,"va":5371812830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t5626968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e858dc5500","size":5,"soft_fail":false,"target_va":5377439776,"va":5371812803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5627433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e829de5500","size":5,"soft_fail":false,"target_va":5377441024,"va":5371813586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2029412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e89c08e1ff","size":5,"soft_fail":false,"target_va":5369783056,"va":5371812463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t180170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e8cabf0200","size":5,"soft_fail":false,"target_va":5371992624,"va":5371812449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t759710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371812352,"raw_hex":"e89e970b00","size":5,"soft_fail":false,"target_va":5372572192,"va":5371812477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":191,"record_offset":10224,"parent_record_offset":9476,"ancestor_record_offset":9476,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":9872,"parent_record_offset":9476,"ancestor_record_offset":9476,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":10544,"parent_record_offset":9476,"ancestor_record_offset":9476,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":191,"record_offset":9628,"parent_record_offset":9476,"ancestor_record_offset":9476,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":191,"record_offset":9808,"parent_record_offset":9476,"ancestor_record_offset":9476,"inlinee":7942,"source_line":789,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":9476,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":10464,"parent_record_offset":10408,"ancestor_record_offset":9476,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":10436,"parent_record_offset":10408,"ancestor_record_offset":9476,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":10408,"parent_record_offset":10380,"ancestor_record_offset":9476,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":17008,"parent_record_offset":16956,"ancestor_record_offset":9476,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":9476,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":9476,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":9476,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":11820,"parent_record_offset":11784,"ancestor_record_offset":9476,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11784,"parent_record_offset":11744,"ancestor_record_offset":9476,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":12660,"parent_record_offset":12628,"ancestor_record_offset":9476,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11744,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":191,"record_offset":10252,"parent_record_offset":10224,"ancestor_record_offset":9476,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":13576,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":17036,"parent_record_offset":16956,"ancestor_record_offset":9476,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":191,"record_offset":14288,"parent_record_offset":14260,"ancestor_record_offset":9476,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":191,"record_offset":14692,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":9476,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":17064,"parent_record_offset":16956,"ancestor_record_offset":9476,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":9744,"parent_record_offset":9656,"ancestor_record_offset":9476,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":9476,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":16880,"parent_record_offset":10588,"ancestor_record_offset":9476,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":13508,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":9952,"parent_record_offset":9920,"ancestor_record_offset":9476,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":9688,"parent_record_offset":9656,"ancestor_record_offset":9476,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":9476,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":191,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":9476,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":191,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":9476,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":191,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":9476,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":191,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":9476,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":9476,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":9476,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":9476,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":191,"record_offset":10940,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10908,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":9476,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10768,"parent_record_offset":10628,"ancestor_record_offset":9476,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10972,"parent_record_offset":10940,"ancestor_record_offset":9476,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":191,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9476,"inlinee":7892,"source_line":287,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":10628,"parent_record_offset":10588,"ancestor_record_offset":9476,"inlinee":12506,"source_line":61,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":9476,"inlinee":12510,"source_line":85,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":10588,"parent_record_offset":10544,"ancestor_record_offset":9476,"inlinee":12504,"source_line":56,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":16924,"parent_record_offset":16880,"ancestor_record_offset":9476,"inlinee":12508,"source_line":57,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30D8
// 首条位置: {"module":191,"inlinee":12504}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":16880,"parent_record_offset":10588,"ancestor_record_offset":10588,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":10628,"parent_record_offset":10588,"ancestor_record_offset":10588,"inlinee":12506,"source_line":61,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":10588,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":11356,"parent_record_offset":11300,"ancestor_record_offset":10588,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":10588,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":11300,"parent_record_offset":11272,"ancestor_record_offset":10588,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":17008,"parent_record_offset":16956,"ancestor_record_offset":10588,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":10588,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":10588,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":10588,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":11820,"parent_record_offset":11784,"ancestor_record_offset":10588,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11784,"parent_record_offset":11744,"ancestor_record_offset":10588,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":12660,"parent_record_offset":12628,"ancestor_record_offset":10588,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11744,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":13576,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":17036,"parent_record_offset":16956,"ancestor_record_offset":10588,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":191,"record_offset":14288,"parent_record_offset":14260,"ancestor_record_offset":10588,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":191,"record_offset":14692,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=85
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":11140,"parent_record_offset":10940,"ancestor_record_offset":10588,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":17064,"parent_record_offset":16956,"ancestor_record_offset":10588,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":13508,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10588,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10588,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":10588,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":191,"record_offset":10940,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10908,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10588,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10768,"parent_record_offset":10628,"ancestor_record_offset":10588,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10972,"parent_record_offset":10940,"ancestor_record_offset":10588,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":10588,"inlinee":12510,"source_line":85,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":16924,"parent_record_offset":16880,"ancestor_record_offset":10588,"inlinee":12508,"source_line":57,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30DC
// 首条位置: {"module":191,"inlinee":12508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":16924,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":17308,"parent_record_offset":16924,"ancestor_record_offset":16924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":17256,"parent_record_offset":17232,"ancestor_record_offset":16924,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":17232,"parent_record_offset":17176,"ancestor_record_offset":16924,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":17204,"parent_record_offset":17176,"ancestor_record_offset":16924,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":17176,"parent_record_offset":17148,"ancestor_record_offset":16924,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":17008,"parent_record_offset":16956,"ancestor_record_offset":16924,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":17036,"parent_record_offset":16956,"ancestor_record_offset":16924,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":17100,"parent_record_offset":16956,"ancestor_record_offset":16924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":17064,"parent_record_offset":16956,"ancestor_record_offset":16924,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30DA
// 首条位置: {"module":191,"inlinee":12506}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11744,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":13576,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":191,"record_offset":14692,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":11976,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":13508,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":191,"record_offset":10940,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10908,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10768,"parent_record_offset":10628,"ancestor_record_offset":10628,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":10628,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":11356,"parent_record_offset":11300,"ancestor_record_offset":10628,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":10628,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":11300,"parent_record_offset":11272,"ancestor_record_offset":10628,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":10628,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":10628,"inlinee":10981,"source_line":316,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":11820,"parent_record_offset":11784,"ancestor_record_offset":10628,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":11784,"parent_record_offset":11744,"ancestor_record_offset":10628,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":12660,"parent_record_offset":12628,"ancestor_record_offset":10628,"inlinee":10980,"source_line":33,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":191,"record_offset":14288,"parent_record_offset":14260,"ancestor_record_offset":10628,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=68
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":11140,"parent_record_offset":10940,"ancestor_record_offset":10628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":10628,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10628,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10628,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":10628,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10628,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10628,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":191,"record_offset":10972,"parent_record_offset":10940,"ancestor_record_offset":10628,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":191,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":10628,"inlinee":12510,"source_line":85,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30DE
// 首条位置: {"module":191,"inlinee":12510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":14812,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":14964,"parent_record_offset":14940,"ancestor_record_offset":14720,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":14940,"parent_record_offset":14884,"ancestor_record_offset":14720,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":14912,"parent_record_offset":14884,"ancestor_record_offset":14720,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14720,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":14780,"parent_record_offset":14748,"ancestor_record_offset":14720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
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
// 完整结构及原始类型记录: theme_fetcher_plugin.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ActiveEventReceiver
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>,bevy_app::main_schedule::Update,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "bevy_app::main_schedule::Update", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$4::init_state<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::system::system_param::impl$4::init_state", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>,bevy_app::main_schedule::Update,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "bevy_app::main_schedule::Update", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$7::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::sealed::impl$7::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: theme_fetcher_plugin.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: theme_fetcher_plugin.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build
// 信息 ID: 63fbec34cfa38312207180eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 信息 ID: c00ec7e82ad161bce4679158
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; module=14, inline_record=2428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 信息 ID: a8e7cc2ad317376916b721a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=191, inline_record=10628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 信息 ID: 6be65f79bd2daf2432308729
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::try_load_theme_info", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=191, inline_record=14720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 信息 ID: bac8549cb61bf26cfa0cfa7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c4ff6cd0298cc47ef1ee6925: void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > @ 0xc8be2 -> 0x2f5a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> @ 0x10aa30 -> 0x2f5a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 信息 ID: 9442e52abeb8e6a3cf0ad463
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=191, inline_record=10588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 信息 ID: fa6b5567ba2db46cf417ddba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=191, inline_record=16924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
