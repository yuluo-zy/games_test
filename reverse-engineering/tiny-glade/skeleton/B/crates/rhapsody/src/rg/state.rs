// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/state.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a9ddf193770090d29d2fd37f7b320e2c9d52b66368b9120b50f4fb0d5aabe4e0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 66, 692
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::state::impl$0::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":692,"record_offset":256,"rva":12474320,"va":5381183440}
// 编译实例 RVAs: 0xbe57d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381183440,"raw_hex":"e8e799ffff","size":5,"soft_fail":false,"target_va":5381157904,"va":5381184036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e76a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12590273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381183440,"raw_hex":"e8c11cc000","size":5,"soft_fail":false,"target_va":5393774240,"va":5381183962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30663375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381183440,"raw_hex":"e8cfe2d301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381183788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1474293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381183440,"raw_hex":"e8f57e1600","size":5,"soft_fail":false,"target_va":5382657856,"va":5381183558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1570199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381183440,"raw_hex":"e897f51700","size":5,"soft_fail":false,"target_va":5382753792,"va":5381183588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":692,"record_offset":1976,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::impl$12::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":692,"record_offset":384,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":39380,"source_line":49,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":692,"record_offset":408,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":14540,"source_line":79,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":692,"record_offset":960,"parent_record_offset":928,"ancestor_record_offset":256,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":692,"record_offset":1020,"parent_record_offset":988,"ancestor_record_offset":256,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":692,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":256,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":692,"record_offset":988,"parent_record_offset":928,"ancestor_record_offset":256,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":692,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":256,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":692,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":256,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":692,"record_offset":1860,"parent_record_offset":408,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":692,"record_offset":768,"parent_record_offset":652,"ancestor_record_offset":256,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":692,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":256,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":692,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":692,"record_offset":872,"parent_record_offset":816,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":692,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":256,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":692,"record_offset":1128,"parent_record_offset":444,"ancestor_record_offset":256,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":692,"record_offset":444,"parent_record_offset":408,"ancestor_record_offset":256,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":692,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":256,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":692,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":256,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":692,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":256,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":692,"record_offset":928,"parent_record_offset":540,"ancestor_record_offset":256,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":692,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":256,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":692,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":692,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":692,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":692,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":256,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00692.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":66,"record_offset":27368,"rva":1460144,"va":5370169264}
// 编译实例 RVAs: 0x1647b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1217854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8c26aedff","size":5,"soft_fail":false,"target_va":5368953728,"va":5370171577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1153936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e87064eeff","size":5,"soft_fail":false,"target_va":5369021440,"va":5370175371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ceb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1148810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e87678eeff","size":5,"soft_fail":false,"target_va":5369024176,"va":5370172981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c6970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-648144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8301cf6ff","size":5,"soft_fail":false,"target_va":5369522544,"va":5370170683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-572335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e85144f7ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5370173130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=9
//     target_aliases=19 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-553563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8a58df7ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5370169670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161b50
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e857b8ffff","size":5,"soft_fail":false,"target_va":5370157904,"va":5370176244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e863b8ffff","size":5,"soft_fail":false,"target_va":5370158128,"va":5370176456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c4280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1435938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e822e91500","size":5,"soft_fail":false,"target_va":5371609728,"va":5370173785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f7ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3745669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e885273900","size":5,"soft_fail":false,"target_va":5373918368,"va":5370172694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140530430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3976243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e833ac3c00","size":5,"soft_fail":false,"target_va":5374149680,"va":5370173432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4541612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8ac4c4500","size":5,"soft_fail":false,"target_va":5374712576,"va":5370170959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405da820
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4673346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8424f4700","size":5,"soft_fail":false,"target_va":5374847008,"va":5370173657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405daf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4672826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e83a4d4700","size":5,"soft_fail":false,"target_va":5374848768,"va":5370175937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dcdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4680595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8936b4700","size":5,"soft_fail":false,"target_va":5374856640,"va":5370176040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4726664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8881f4800","size":5,"soft_fail":false,"target_va":5374900736,"va":5370174067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ea740
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4738394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e85a4d4800","size":5,"soft_fail":false,"target_va":5374912320,"va":5370173921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eb290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4741168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e830584800","size":5,"soft_fail":false,"target_va":5374915216,"va":5370174043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4743896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8d8624800","size":5,"soft_fail":false,"target_va":5374917872,"va":5370173971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4749456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e890784800","size":5,"soft_fail":false,"target_va":5374923456,"va":5370173995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4756936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8c8954800","size":5,"soft_fail":false,"target_va":5374928352,"va":5370171411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef210
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4757875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e873994800","size":5,"soft_fail":false,"target_va":5374931472,"va":5370173592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fcbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4811098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e85a694900","size":5,"soft_fail":false,"target_va":5374987232,"va":5370176129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fd1d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4815109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e805794900","size":5,"soft_fail":false,"target_va":5374988752,"va":5370173638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060b5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4871621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8c5554a00","size":5,"soft_fail":false,"target_va":5375047088,"va":5370175462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4910347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e80bed4a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5370171152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5345945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e899925100","size":5,"soft_fail":false,"target_va":5375516032,"va":5370170082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10948545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8c10fa700","size":5,"soft_fail":false,"target_va":5381120896,"va":5370172346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10952987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e81b21a700","size":5,"soft_fail":false,"target_va":5381122816,"va":5370169824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10951531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e86b1ba700","size":5,"soft_fail":false,"target_va":5381123296,"va":5370171760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10998758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8e6d3a700","size":5,"soft_fail":false,"target_va":5381170352,"va":5370171589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41116282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e87a627302","size":5,"soft_fail":false,"target_va":5411285568,"va":5370169281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41677776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8d0f37b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370169387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t41367470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8ae377702","size":5,"soft_fail":false,"target_va":5411541776,"va":5370174301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41367572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e814387702","size":5,"soft_fail":false,"target_va":5411542064,"va":5370174487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e897ceffff","size":5,"soft_fail":false,"target_va":5370158176,"va":5370170820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41369004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8ac3d7702","size":5,"soft_fail":false,"target_va":5411543264,"va":5370174255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8099517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8bd967b00","size":5,"soft_fail":false,"target_va":5378272176,"va":5370172654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8226417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e871867d00","size":5,"soft_fail":false,"target_va":5378396224,"va":5370169802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8223166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8be797d00","size":5,"soft_fail":false,"target_va":5378395696,"va":5370172525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8225654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e876837d00","size":5,"soft_fail":false,"target_va":5378395440,"va":5370169781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SavedSession::loading_background_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10362154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e82a1d9e00","size":5,"soft_fail":false,"target_va":5380532816,"va":5370170657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::RenderContext::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8881759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e85f868700","size":5,"soft_fail":false,"target_va":5379054160,"va":5370172396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8786865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8b1138600","size":5,"soft_fail":false,"target_va":5378959104,"va":5370172234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7154813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e87d2c6d00","size":5,"soft_fail":false,"target_va":5377327088,"va":5370172270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::vermeer_render_from_target
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7617089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8413a7400","size":5,"soft_fail":false,"target_va":5377789296,"va":5370172202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10783507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8138ba400","size":5,"soft_fail":false,"target_va":5380954320,"va":5370170808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41318130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8f2767602","size":5,"soft_fail":false,"target_va":5411492368,"va":5370174233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41320518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e846807602","size":5,"soft_fail":false,"target_va":5411494800,"va":5370174277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11653481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e869d1b100","size":5,"soft_fail":false,"target_va":5381823600,"va":5370170114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11653694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e83ed2b100","size":5,"soft_fail":false,"target_va":5381824016,"va":5370170317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11131917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e80ddca900","size":5,"soft_fail":false,"target_va":5381302080,"va":5370170158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11130804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8b4d7a900","size":5,"soft_fail":false,"target_va":5381301104,"va":5370170295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10945719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8b704a700","size":5,"soft_fail":false,"target_va":5381116576,"va":5370170852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10945898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e86a05a700","size":5,"soft_fail":false,"target_va":5381116784,"va":5370170881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11027240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e82843a800","size":5,"soft_fail":false,"target_va":5381199056,"va":5370171811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::on_retire
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11025362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8d23ba800","size":5,"soft_fail":false,"target_va":5381198880,"va":5370173513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11019124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e87423a800","size":5,"soft_fail":false,"target_va":5381192608,"va":5370173479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41362598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370169264,"raw_hex":"e8a6247702","size":5,"soft_fail":false,"target_va":5411537104,"va":5370174501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":40088,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":27740,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":27864,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":40204,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":66,"record_offset":40892,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":10020,"source_line":48,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":28068,"parent_record_offset":27368,"ancestor_record_offset":27368,"inlinee":10026,"source_line":496,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":27368,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33168,"parent_record_offset":33112,"ancestor_record_offset":27368,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":27368,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33140,"parent_record_offset":33112,"ancestor_record_offset":27368,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":27368,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":27368,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":27368,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33088,"parent_record_offset":33064,"ancestor_record_offset":27368,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":27368,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28696,"parent_record_offset":28656,"ancestor_record_offset":27368,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":40964,"parent_record_offset":40940,"ancestor_record_offset":27368,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":40940,"parent_record_offset":40892,"ancestor_record_offset":27368,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28656,"parent_record_offset":28632,"ancestor_record_offset":27368,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":29236,"parent_record_offset":29212,"ancestor_record_offset":27368,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":29212,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":29044,"parent_record_offset":29020,"ancestor_record_offset":27368,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":29020,"parent_record_offset":28996,"ancestor_record_offset":27368,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":28968,"parent_record_offset":28936,"ancestor_record_offset":27368,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":29996,"parent_record_offset":29460,"ancestor_record_offset":27368,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28912,"parent_record_offset":28372,"ancestor_record_offset":27368,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28936,"parent_record_offset":28912,"ancestor_record_offset":27368,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28404,"parent_record_offset":28372,"ancestor_record_offset":27368,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":30080,"parent_record_offset":30020,"ancestor_record_offset":27368,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28996,"parent_record_offset":28936,"ancestor_record_offset":27368,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":27368,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":27368,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28632,"parent_record_offset":28608,"ancestor_record_offset":27368,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":29380,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":28280,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":27368,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29088,"parent_record_offset":28372,"ancestor_record_offset":27368,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28332,"parent_record_offset":28304,"ancestor_record_offset":27368,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29404,"parent_record_offset":29380,"ancestor_record_offset":27368,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29460,"parent_record_offset":29404,"ancestor_record_offset":27368,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":27368,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28372,"parent_record_offset":28304,"ancestor_record_offset":27368,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":41288,"parent_record_offset":41260,"ancestor_record_offset":27368,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":27368,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":41316,"parent_record_offset":41260,"ancestor_record_offset":27368,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":27368,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":36032,"parent_record_offset":35964,"ancestor_record_offset":27368,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":31316,"parent_record_offset":31280,"ancestor_record_offset":27368,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":31428,"parent_record_offset":31400,"ancestor_record_offset":27368,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":66,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":27368,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":66,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":27368,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":28544,"parent_record_offset":28480,"ancestor_record_offset":27368,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":27368,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":31772,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":28608,"parent_record_offset":28428,"ancestor_record_offset":27368,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":35292,"parent_record_offset":31812,"ancestor_record_offset":27368,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":27368,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":41232,"parent_record_offset":41204,"ancestor_record_offset":27368,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":31352,"parent_record_offset":31200,"ancestor_record_offset":27368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":66,"record_offset":40648,"parent_record_offset":40560,"ancestor_record_offset":27368,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":27796,"parent_record_offset":27768,"ancestor_record_offset":27368,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":27828,"parent_record_offset":27768,"ancestor_record_offset":27368,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":66,"record_offset":32728,"parent_record_offset":31812,"ancestor_record_offset":27368,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":31280,"parent_record_offset":31248,"ancestor_record_offset":27368,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":32700,"parent_record_offset":31812,"ancestor_record_offset":27368,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":28480,"parent_record_offset":28452,"ancestor_record_offset":27368,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":27368,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28516,"parent_record_offset":28480,"ancestor_record_offset":27368,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28836,"parent_record_offset":28656,"ancestor_record_offset":27368,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":27368,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31704,"parent_record_offset":31640,"ancestor_record_offset":27368,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31052,"parent_record_offset":31016,"ancestor_record_offset":27368,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31728,"parent_record_offset":31704,"ancestor_record_offset":27368,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31076,"parent_record_offset":31052,"ancestor_record_offset":27368,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":27368,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":66,"record_offset":36092,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":66,"record_offset":41088,"parent_record_offset":41060,"ancestor_record_offset":27368,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40560,"parent_record_offset":40528,"ancestor_record_offset":27368,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40472,"parent_record_offset":40448,"ancestor_record_offset":27368,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40528,"parent_record_offset":40472,"ancestor_record_offset":27368,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40500,"parent_record_offset":40472,"ancestor_record_offset":27368,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40592,"parent_record_offset":40560,"ancestor_record_offset":27368,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":40332,"parent_record_offset":40308,"ancestor_record_offset":27368,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":66,"record_offset":41060,"parent_record_offset":41032,"ancestor_record_offset":27368,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":66,"record_offset":32868,"parent_record_offset":31812,"ancestor_record_offset":27368,"inlinee":5993,"source_line":688,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":30988,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":31144,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":31612,"parent_record_offset":31584,"ancestor_record_offset":27368,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":31016,"parent_record_offset":30988,"ancestor_record_offset":27368,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":31640,"parent_record_offset":31612,"ancestor_record_offset":27368,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":31108,"parent_record_offset":31016,"ancestor_record_offset":27368,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":31676,"parent_record_offset":31640,"ancestor_record_offset":27368,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":29324,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":31200,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":31248,"parent_record_offset":31200,"ancestor_record_offset":27368,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":29348,"parent_record_offset":29324,"ancestor_record_offset":27368,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":27368,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":34976,"parent_record_offset":31812,"ancestor_record_offset":27368,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":35032,"parent_record_offset":34976,"ancestor_record_offset":27368,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":35964,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":37904,"parent_record_offset":28068,"ancestor_record_offset":27368,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":66,"record_offset":40996,"parent_record_offset":40892,"ancestor_record_offset":27368,"inlinee":10024,"source_line":12,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":36060,"parent_record_offset":35964,"ancestor_record_offset":27368,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":66,"record_offset":41032,"parent_record_offset":40996,"ancestor_record_offset":27368,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":41144,"parent_record_offset":41116,"ancestor_record_offset":27368,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":41260,"parent_record_offset":41204,"ancestor_record_offset":27368,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":41116,"parent_record_offset":41088,"ancestor_record_offset":27368,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":41204,"parent_record_offset":41172,"ancestor_record_offset":27368,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":41172,"parent_record_offset":41144,"ancestor_record_offset":27368,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":31812,"parent_record_offset":31772,"ancestor_record_offset":27368,"inlinee":10028,"source_line":526,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":66,"record_offset":4616,"rva":1449168,"va":5370158288}
// 编译实例 RVAs: 0x161cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12684950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8968ec100","size":5,"soft_fail":false,"target_va":5382847888,"va":5370162933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t12483457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8817bbe00","size":5,"soft_fail":false,"target_va":5382644528,"va":5370161066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1146944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8c07feeff","size":5,"soft_fail":false,"target_va":5369021440,"va":5370168379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ceb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1141339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a595eeff","size":5,"soft_fail":false,"target_va":5369024176,"va":5370165510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-564864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e88061f7ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5370165659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=17
//     target_aliases=19 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-542627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85db8f7ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5370158734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161b50
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e867d8ffff","size":5,"soft_fail":false,"target_va":5370157904,"va":5370168036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e803d5ffff","size":5,"soft_fail":false,"target_va":5370158128,"va":5370169128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c4280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1443420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85c061600","size":5,"soft_fail":false,"target_va":5371609728,"va":5370166303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f7ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3753140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8b4443900","size":5,"soft_fail":false,"target_va":5373918368,"va":5370165223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140530430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3983714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e862c93c00","size":5,"soft_fail":false,"target_va":5374149680,"va":5370165961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405da820
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4680828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e87c6c4700","size":5,"soft_fail":false,"target_va":5374847008,"va":5370166175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405daf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4679818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e88a684700","size":5,"soft_fail":false,"target_va":5374848768,"va":5370168945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dcdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4688346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8da894700","size":5,"soft_fail":false,"target_va":5374856640,"va":5370168289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef210
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4765350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a6b64800","size":5,"soft_fail":false,"target_va":5374931472,"va":5370166117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fcbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4819306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e86a894900","size":5,"soft_fail":false,"target_va":5374987232,"va":5370167921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fd1d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4822591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e83f964900","size":5,"soft_fail":false,"target_va":5374988752,"va":5370166156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060b5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4878613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e815714a00","size":5,"soft_fail":false,"target_va":5375047088,"va":5370168470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4916316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85c044b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5370165183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=7
//     target_aliases=69 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5356733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8bdbc5100","size":5,"soft_fail":false,"target_va":5375516032,"va":5370159294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408aea50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7650228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8b4bb7400","size":5,"soft_fail":false,"target_va":5377813072,"va":5370162839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fc60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8241139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8f3bf7d00","size":5,"soft_fail":false,"target_va":5378407520,"va":5370166376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10956221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8bd2da700","size":5,"soft_fail":false,"target_va":5381120896,"va":5370164670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10963921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8d14ba700","size":5,"soft_fail":false,"target_va":5381122816,"va":5370158890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10963308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e86c49a700","size":5,"soft_fail":false,"target_va":5381123296,"va":5370159983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11002775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e897e3a700","size":5,"soft_fail":false,"target_va":5381170352,"va":5370167572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41127258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85a8d7302","size":5,"soft_fail":false,"target_va":5411285568,"va":5370158305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41688745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a91e7c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370158418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41374555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85b537702","size":5,"soft_fail":false,"target_va":5411541808,"va":5370167248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t41374703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8ef537702","size":5,"soft_fail":false,"target_va":5411541776,"va":5370167068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41376293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8255a7702","size":5,"soft_fail":false,"target_va":5411543264,"va":5370166966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8107172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a4b47b00","size":5,"soft_fail":false,"target_va":5378272176,"va":5370164999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8237351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e827b17d00","size":5,"soft_fail":false,"target_va":5378396224,"va":5370158868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8230821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a5977d00","size":5,"soft_fail":false,"target_va":5378395696,"va":5370164870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8236589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e82dae7d00","size":5,"soft_fail":false,"target_va":5378395440,"va":5370158846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::is_profiling
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8112308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8b4c87b00","size":5,"soft_fail":false,"target_va":5378273520,"va":5370161207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::frame_constants::FrameConstants::should_reset_temporal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8471022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8ee418100","size":5,"soft_fail":false,"target_va":5378631808,"va":5370160781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::export_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8579731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e893ea8200","size":5,"soft_fail":false,"target_va":5378744864,"va":5370165128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::import_to_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8585727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8ff018300","size":5,"soft_fail":false,"target_va":5378744768,"va":5370159036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gpu_lut::GpuLutTracker::render_luts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8044436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e894bf7a00","size":5,"soft_fail":false,"target_va":5378204288,"va":5370159847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::grading_lut::ColorGradingState::flattened
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10083438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e86edc9900","size":5,"soft_fail":false,"target_va":5380246304,"va":5370162861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::cast_ui_shadows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8243161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8d9c77d00","size":5,"soft_fail":false,"target_va":5378407424,"va":5370164258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8246912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e880d67d00","size":5,"soft_fail":false,"target_va":5378407392,"va":5370160475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8246901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e875d67d00","size":5,"soft_fail":false,"target_va":5378407360,"va":5370160454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::should_show_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8247131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e85bd77d00","size":5,"soft_fail":false,"target_va":5378407456,"va":5370160320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::RenderContext::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8889428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e854a48700","size":5,"soft_fail":false,"target_va":5379054160,"va":5370164727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8956173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e80da98800","size":5,"soft_fail":false,"target_va":5379119200,"va":5370163022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8766510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e82ec48500","size":5,"soft_fail":false,"target_va":5378927776,"va":5370161261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8029550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e86e857a00","size":5,"soft_fail":false,"target_va":5378189424,"va":5370159869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8839838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e89ee28600","size":5,"soft_fail":false,"target_va":5379004176,"va":5370164333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7612134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8e6267400","size":5,"soft_fail":false,"target_va":5377776240,"va":5370164101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::final_post::final_post
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8036304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8d09f7a00","size":5,"soft_fail":false,"target_va":5378199552,"va":5370163243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7613560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8782c7400","size":5,"soft_fail":false,"target_va":5377777488,"va":5370163923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8794802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8b2328600","size":5,"soft_fail":false,"target_va":5378959104,"va":5370164297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8786019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e863108600","size":5,"soft_fail":false,"target_va":5378950256,"va":5370164232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7162646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8164b6d00","size":5,"soft_fail":false,"target_va":5377327088,"va":5370164437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7161943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e857486d00","size":5,"soft_fail":false,"target_va":5377326144,"va":5370164196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8820578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e862978600","size":5,"soft_fail":false,"target_va":5378984736,"va":5370164153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7655930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8fad17400","size":5,"soft_fail":false,"target_va":5377820000,"va":5370164065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::menu_background::render_menu_bg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7760158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e81e697600","size":5,"soft_fail":false,"target_va":5377923632,"va":5370163469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::painterly::PainterlyState::has_target
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7618013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8dd3d7400","size":5,"soft_fail":false,"target_va":5377778640,"va":5370160622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::painterly::painterly_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7615165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8bd327400","size":5,"soft_fail":false,"target_va":5377778704,"va":5370163534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::session_selection::render_session_selection_previews
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7944382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8be387900","size":5,"soft_fail":false,"target_va":5378108416,"va":5370164029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::vermeer_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7622772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e874507400","size":5,"soft_fail":false,"target_va":5377786608,"va":5370163831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41325419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e86b937602","size":5,"soft_fail":false,"target_va":5411492368,"va":5370166944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41327807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8bf9c7602","size":5,"soft_fail":false,"target_va":5411494800,"va":5370166988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11664269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e88dfbb100","size":5,"soft_fail":false,"target_va":5381823600,"va":5370159326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11664482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e862fcb100","size":5,"soft_fail":false,"target_va":5381824016,"va":5370159529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11142705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e83106aa00","size":5,"soft_fail":false,"target_va":5381302080,"va":5370159370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11141592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8d801aa00","size":5,"soft_fail":false,"target_va":5381301104,"va":5370159507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10957090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e82231a700","size":5,"soft_fail":false,"target_va":5381117024,"va":5370159929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10956674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8822fa700","size":5,"soft_fail":false,"target_va":5381116576,"va":5370159897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11039017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e82971a800","size":5,"soft_fail":false,"target_va":5381199056,"va":5370160034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::on_retire
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11032837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e80559a800","size":5,"soft_fail":false,"target_va":5381198880,"va":5370166038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11026599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8a740a800","size":5,"soft_fail":false,"target_va":5381192608,"va":5370166004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41369656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e838407702","size":5,"soft_fail":false,"target_va":5411537104,"va":5370167443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::egui_backend::EguiBackend::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-546959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e871a7f7ff","size":5,"soft_fail":false,"target_va":5369617536,"va":5370164490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::capture_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4351776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e820674200","size":5,"soft_fail":false,"target_va":5374516368,"va":5370164587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::handle_screenshot_error
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3706628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8048f3800","size":5,"soft_fail":false,"target_va":5373870000,"va":5370163367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::render_world
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3708434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e812963800","size":5,"soft_fail":false,"target_va":5373870352,"va":5370161913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::handle_screenshot_opportunity
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1810602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370158288,"raw_hex":"e8aaa01b00","size":5,"soft_fail":false,"target_va":5371973952,"va":5370163345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":25840,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":4980,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":5108,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":25956,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":66,"record_offset":26644,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":10020,"source_line":48,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_main_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":5312,"parent_record_offset":4616,"ancestor_record_offset":4616,"inlinee":9998,"source_line":81,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":4616,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":24532,"parent_record_offset":24476,"ancestor_record_offset":4616,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":4616,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24504,"parent_record_offset":24476,"ancestor_record_offset":4616,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":4616,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":4616,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":4616,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24452,"parent_record_offset":24428,"ancestor_record_offset":4616,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":4616,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":4616,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":4616,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6288,"parent_record_offset":6248,"ancestor_record_offset":4616,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":4616,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17644,"parent_record_offset":17432,"ancestor_record_offset":4616,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":26692,"parent_record_offset":26644,"ancestor_record_offset":4616,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":4616,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":4616,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":4616,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":6804,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":4616,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":4616,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6560,"parent_record_offset":6528,"ancestor_record_offset":4616,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":8564,"parent_record_offset":8028,"ancestor_record_offset":4616,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6504,"parent_record_offset":5964,"ancestor_record_offset":4616,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":4616,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":5996,"parent_record_offset":5964,"ancestor_record_offset":4616,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":8648,"parent_record_offset":8588,"ancestor_record_offset":4616,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6588,"parent_record_offset":6528,"ancestor_record_offset":4616,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4616,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":4616,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4616,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":7948,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":5872,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":4616,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":6680,"parent_record_offset":5964,"ancestor_record_offset":4616,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":4616,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":4616,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":8028,"parent_record_offset":7972,"ancestor_record_offset":4616,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":4616,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5964,"parent_record_offset":5896,"ancestor_record_offset":4616,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":66,"record_offset":19552,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":66,"record_offset":19668,"parent_record_offset":19576,"ancestor_record_offset":4616,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27040,"parent_record_offset":27012,"ancestor_record_offset":4616,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":4616,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27068,"parent_record_offset":27012,"ancestor_record_offset":4616,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":4616,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":66,"record_offset":14756,"parent_record_offset":14640,"ancestor_record_offset":4616,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":11356,"parent_record_offset":11288,"ancestor_record_offset":4616,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":9884,"parent_record_offset":9848,"ancestor_record_offset":4616,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":9996,"parent_record_offset":9968,"ancestor_record_offset":4616,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":66,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":4616,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":66,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":4616,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":66,"record_offset":26368,"parent_record_offset":26344,"ancestor_record_offset":4616,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":66,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":4616,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":6136,"parent_record_offset":6072,"ancestor_record_offset":4616,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":66,"record_offset":19612,"parent_record_offset":19576,"ancestor_record_offset":4616,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":66,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":4616,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":4616,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":25132,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":17992,"parent_record_offset":17432,"ancestor_record_offset":4616,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":6200,"parent_record_offset":6020,"ancestor_record_offset":4616,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":19524,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":13772,"parent_record_offset":12968,"ancestor_record_offset":4616,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":16904,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":4616,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14844,"parent_record_offset":14812,"ancestor_record_offset":4616,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14872,"parent_record_offset":14812,"ancestor_record_offset":4616,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":4616,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":4616,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":9920,"parent_record_offset":9768,"ancestor_record_offset":4616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":66,"record_offset":26400,"parent_record_offset":26312,"ancestor_record_offset":4616,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":4616,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17876,"parent_record_offset":17840,"ancestor_record_offset":4616,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":4616,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":5072,"parent_record_offset":5008,"ancestor_record_offset":4616,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":9848,"parent_record_offset":9816,"ancestor_record_offset":4616,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":4616,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":4616,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6108,"parent_record_offset":6072,"ancestor_record_offset":4616,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6428,"parent_record_offset":6248,"ancestor_record_offset":4616,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":4616,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17840,"parent_record_offset":17432,"ancestor_record_offset":4616,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17904,"parent_record_offset":17840,"ancestor_record_offset":4616,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":4616,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":10272,"parent_record_offset":10208,"ancestor_record_offset":4616,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":9620,"parent_record_offset":9584,"ancestor_record_offset":4616,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":14696,"parent_record_offset":14672,"ancestor_record_offset":4616,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":4616,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":4616,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":4616,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":66,"record_offset":12968,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":66,"record_offset":26840,"parent_record_offset":26812,"ancestor_record_offset":4616,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26312,"parent_record_offset":26280,"ancestor_record_offset":4616,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":4616,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26280,"parent_record_offset":26224,"ancestor_record_offset":4616,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26252,"parent_record_offset":26224,"ancestor_record_offset":4616,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26344,"parent_record_offset":26312,"ancestor_record_offset":4616,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":26084,"parent_record_offset":26060,"ancestor_record_offset":4616,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":66,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":4616,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":9556,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":9712,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":4616,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14900,"parent_record_offset":14812,"ancestor_record_offset":4616,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14788,"parent_record_offset":14640,"ancestor_record_offset":4616,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":4616,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14924,"parent_record_offset":14900,"ancestor_record_offset":4616,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14672,"parent_record_offset":14640,"ancestor_record_offset":4616,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":4616,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":4616,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":9676,"parent_record_offset":9584,"ancestor_record_offset":4616,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":10244,"parent_record_offset":10208,"ancestor_record_offset":4616,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":6916,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":9768,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":9816,"parent_record_offset":9768,"ancestor_record_offset":4616,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":4616,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":4616,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":11328,"parent_record_offset":11288,"ancestor_record_offset":4616,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":11288,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":21480,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":66,"record_offset":26748,"parent_record_offset":26644,"ancestor_record_offset":4616,"inlinee":10024,"source_line":12,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":11384,"parent_record_offset":11288,"ancestor_record_offset":4616,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":66,"record_offset":26784,"parent_record_offset":26748,"ancestor_record_offset":4616,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26896,"parent_record_offset":26868,"ancestor_record_offset":4616,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":27012,"parent_record_offset":26956,"ancestor_record_offset":4616,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26868,"parent_record_offset":26840,"ancestor_record_offset":4616,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26956,"parent_record_offset":26924,"ancestor_record_offset":4616,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26924,"parent_record_offset":26896,"ancestor_record_offset":4616,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":16940,"parent_record_offset":16904,"ancestor_record_offset":4616,"inlinee":10015,"source_line":199,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":17140,"parent_record_offset":17104,"ancestor_record_offset":4616,"inlinee":10016,"source_line":207,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":14640,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10014,"source_line":139,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::next_requested_screenshot_stage
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":66,"record_offset":17432,"parent_record_offset":5312,"ancestor_record_offset":4616,"inlinee":10018,"source_line":511,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":66,"record_offset":41644,"rva":1467664,"va":5370176784}
// 编译实例 RVAs: 0x166510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1160187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8054ceeff","size":5,"soft_fail":false,"target_va":5369021440,"va":5370181622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ceb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1154865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8cf60eeff","size":5,"soft_fail":false,"target_va":5369024176,"va":5370179036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-578390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8aa2cf7ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5370179185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=5
//     target_aliases=19 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-561104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e83070f7ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5370177211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161b50
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e825a8ffff","size":5,"soft_fail":false,"target_va":5370157904,"va":5370180390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8d8a4ffff","size":5,"soft_fail":false,"target_va":5370158128,"va":5370181459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c4280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1429898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e88ad11500","size":5,"soft_fail":false,"target_va":5371609728,"va":5370179825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f7ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3739626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8ea0f3900","size":5,"soft_fail":false,"target_va":5373918368,"va":5370178737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140530430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3970188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e88c943c00","size":5,"soft_fail":false,"target_va":5374149680,"va":5370179487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405da820
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4667306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8aa374700","size":5,"soft_fail":false,"target_va":5374847008,"va":5370179697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405daf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4667583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8bf384700","size":5,"soft_fail":false,"target_va":5374848768,"va":5370181180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dcdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4675992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e898594700","size":5,"soft_fail":false,"target_va":5374856640,"va":5370180643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4747687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8a7714800","size":5,"soft_fail":false,"target_va":5374928352,"va":5370180660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef210
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4751828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8d4814800","size":5,"soft_fail":false,"target_va":5374931472,"va":5370179639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fcbe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4806959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e82f594900","size":5,"soft_fail":false,"target_va":5374987232,"va":5370180268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fd1d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4809069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e86d614900","size":5,"soft_fail":false,"target_va":5374988752,"va":5370179678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060b5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4866346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e82a414a00","size":5,"soft_fail":false,"target_va":5375047088,"va":5370180737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4902407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e807ce4a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5370179092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5338334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8de745100","size":5,"soft_fail":false,"target_va":5375516032,"va":5370177693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd60a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10941905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8d1f5a600","size":5,"soft_fail":false,"target_va":5381120160,"va":5370178250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10942325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e875f7a600","size":5,"soft_fail":false,"target_va":5381120544,"va":5370178214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10942511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e82ff8a600","size":5,"soft_fail":false,"target_va":5381120896,"va":5370178380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10945415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e88703a700","size":5,"soft_fail":false,"target_va":5381122816,"va":5370177396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10990156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e84cb2a700","size":5,"soft_fail":false,"target_va":5381170352,"va":5370180191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41108762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e81a457302","size":5,"soft_fail":false,"target_va":5411285568,"va":5370176801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41670264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e878d67b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5370176899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t41361792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e880217702","size":5,"soft_fail":false,"target_va":5411541776,"va":5370179979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41363340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e88c277702","size":5,"soft_fail":false,"target_va":5411543264,"va":5370179919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8093474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8227f7b00","size":5,"soft_fail":false,"target_va":5378272176,"va":5370178697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8218852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8e4687d00","size":5,"soft_fail":false,"target_va":5378396224,"va":5370177367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8217131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e82b627d00","size":5,"soft_fail":false,"target_va":5378395696,"va":5370178560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8218101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8f5657d00","size":5,"soft_fail":false,"target_va":5378395440,"va":5370177334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::RenderContext::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8875725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8cd6e8700","size":5,"soft_fail":false,"target_va":5379054160,"va":5370178430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8023364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8446d7a00","size":5,"soft_fail":false,"target_va":5378201680,"va":5370178311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41312466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8d2607602","size":5,"soft_fail":false,"target_va":5411492368,"va":5370179897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t41314847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e81f6a7602","size":5,"soft_fail":false,"target_va":5411494800,"va":5370179948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11645862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8a6b3b100","size":5,"soft_fail":false,"target_va":5381823600,"va":5370177733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11646061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e86db4b100","size":5,"soft_fail":false,"target_va":5381824016,"va":5370177950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11124291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e843bea900","size":5,"soft_fail":false,"target_va":5381302080,"va":5370177784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11123178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8eab9a900","size":5,"soft_fail":false,"target_va":5381301104,"va":5370177921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::on_retire
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11019315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e83324a800","size":5,"soft_fail":false,"target_va":5381198880,"va":5370179560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11013077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8d50ba800","size":5,"soft_fail":false,"target_va":5381192608,"va":5370179526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41356984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370176784,"raw_hex":"e8b80e7702","size":5,"soft_fail":false,"target_va":5411537104,"va":5370180115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":48088,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":41944,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":42068,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":48204,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":66,"record_offset":48892,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":10020,"source_line":48,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::execute_presentation_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":42268,"parent_record_offset":41644,"ancestor_record_offset":41644,"inlinee":10032,"source_line":427,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":48712,"parent_record_offset":48680,"ancestor_record_offset":41644,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":48680,"parent_record_offset":48528,"ancestor_record_offset":41644,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42876,"parent_record_offset":42852,"ancestor_record_offset":41644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42924,"parent_record_offset":42900,"ancestor_record_offset":41644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":41644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42852,"parent_record_offset":42828,"ancestor_record_offset":41644,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42828,"parent_record_offset":42788,"ancestor_record_offset":41644,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":48964,"parent_record_offset":48940,"ancestor_record_offset":41644,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":48940,"parent_record_offset":48892,"ancestor_record_offset":41644,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42788,"parent_record_offset":42764,"ancestor_record_offset":41644,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":43364,"parent_record_offset":43340,"ancestor_record_offset":41644,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":43340,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43176,"parent_record_offset":43152,"ancestor_record_offset":41644,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":41644,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43100,"parent_record_offset":43068,"ancestor_record_offset":41644,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":44112,"parent_record_offset":43576,"ancestor_record_offset":41644,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43044,"parent_record_offset":42504,"ancestor_record_offset":41644,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43068,"parent_record_offset":43044,"ancestor_record_offset":41644,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":42536,"parent_record_offset":42504,"ancestor_record_offset":41644,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":44196,"parent_record_offset":44136,"ancestor_record_offset":41644,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43128,"parent_record_offset":43068,"ancestor_record_offset":41644,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42584,"parent_record_offset":42560,"ancestor_record_offset":41644,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42560,"parent_record_offset":42536,"ancestor_record_offset":41644,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42764,"parent_record_offset":42740,"ancestor_record_offset":41644,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":43244,"parent_record_offset":43220,"ancestor_record_offset":41644,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43220,"parent_record_offset":42504,"ancestor_record_offset":41644,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42464,"parent_record_offset":42436,"ancestor_record_offset":41644,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43508,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43576,"parent_record_offset":43508,"ancestor_record_offset":41644,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42436,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42504,"parent_record_offset":42436,"ancestor_record_offset":41644,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":49288,"parent_record_offset":49260,"ancestor_record_offset":41644,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":49340,"parent_record_offset":49316,"ancestor_record_offset":41644,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":49316,"parent_record_offset":49260,"ancestor_record_offset":41644,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":41644,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":45400,"parent_record_offset":45364,"ancestor_record_offset":41644,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":45512,"parent_record_offset":45484,"ancestor_record_offset":41644,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":66,"record_offset":41972,"parent_record_offset":41944,"ancestor_record_offset":41644,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":66,"record_offset":48616,"parent_record_offset":48592,"ancestor_record_offset":41644,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":42676,"parent_record_offset":42612,"ancestor_record_offset":41644,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":42700,"parent_record_offset":42676,"ancestor_record_offset":41644,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":42740,"parent_record_offset":42560,"ancestor_record_offset":41644,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":47860,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":44324,"parent_record_offset":44296,"ancestor_record_offset":41644,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":49232,"parent_record_offset":49204,"ancestor_record_offset":41644,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":45436,"parent_record_offset":45284,"ancestor_record_offset":41644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":66,"record_offset":48648,"parent_record_offset":48560,"ancestor_record_offset":41644,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":42000,"parent_record_offset":41972,"ancestor_record_offset":41644,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":42032,"parent_record_offset":41972,"ancestor_record_offset":41644,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":45364,"parent_record_offset":45332,"ancestor_record_offset":41644,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":42612,"parent_record_offset":42584,"ancestor_record_offset":41644,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42992,"parent_record_offset":42968,"ancestor_record_offset":41644,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42648,"parent_record_offset":42612,"ancestor_record_offset":41644,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42968,"parent_record_offset":42788,"ancestor_record_offset":41644,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":43268,"parent_record_offset":43244,"ancestor_record_offset":41644,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45788,"parent_record_offset":45724,"ancestor_record_offset":41644,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45136,"parent_record_offset":45100,"ancestor_record_offset":41644,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45812,"parent_record_offset":45788,"ancestor_record_offset":41644,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45160,"parent_record_offset":45136,"ancestor_record_offset":41644,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":43292,"parent_record_offset":43268,"ancestor_record_offset":41644,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":66,"record_offset":49088,"parent_record_offset":49060,"ancestor_record_offset":41644,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48560,"parent_record_offset":48528,"ancestor_record_offset":41644,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48472,"parent_record_offset":48448,"ancestor_record_offset":41644,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48528,"parent_record_offset":48472,"ancestor_record_offset":41644,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48500,"parent_record_offset":48472,"ancestor_record_offset":41644,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48592,"parent_record_offset":48560,"ancestor_record_offset":41644,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":66,"record_offset":48332,"parent_record_offset":48308,"ancestor_record_offset":41644,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":66,"record_offset":49060,"parent_record_offset":49032,"ancestor_record_offset":41644,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45072,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45228,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45696,"parent_record_offset":45668,"ancestor_record_offset":41644,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":45100,"parent_record_offset":45072,"ancestor_record_offset":41644,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":45724,"parent_record_offset":45696,"ancestor_record_offset":41644,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":45192,"parent_record_offset":45100,"ancestor_record_offset":41644,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":45760,"parent_record_offset":45724,"ancestor_record_offset":41644,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":43452,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":45284,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":45332,"parent_record_offset":45284,"ancestor_record_offset":41644,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":43476,"parent_record_offset":43452,"ancestor_record_offset":41644,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":41644,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45912,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::import_owned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45856,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":9691,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45884,"parent_record_offset":42268,"ancestor_record_offset":41644,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":66,"record_offset":48996,"parent_record_offset":48892,"ancestor_record_offset":41644,"inlinee":10024,"source_line":12,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":66,"record_offset":49032,"parent_record_offset":48996,"ancestor_record_offset":41644,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":49144,"parent_record_offset":49116,"ancestor_record_offset":41644,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":49260,"parent_record_offset":49204,"ancestor_record_offset":41644,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":49116,"parent_record_offset":49088,"ancestor_record_offset":41644,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":49204,"parent_record_offset":49172,"ancestor_record_offset":41644,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":49172,"parent_record_offset":49144,"ancestor_record_offset":41644,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
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
// 完整结构及原始类型记录: state.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::state::RenderGraphState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>", "constructor": "rhapsody::rg::state::RenderGraphState::compile", "arguments": ["rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::compile_main_render_graph::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>", "constructor": "rhapsody::rg::state::RenderGraphState::compile", "arguments": ["rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>", "constructor": "rhapsody::rg::state::RenderGraphState::compile", "arguments": ["tuple$<>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::state::RenderGraphState>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::state::RenderGraphState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "rhapsody::rg::state::RenderGraphState", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>
// 信息 ID: ffe0a76440f13df42021539d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::resource::ResourceExportHandle -> ["rhapsody::raw::texture::Texture"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::state::RenderGraphState::compile -> ["rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e89ea -> 0x1647b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>
// 信息 ID: 3ddaa49064f2eee98747816f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::resource::ResourceExportHandle -> ["rhapsody::raw::texture::Texture"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::state::RenderGraphState::compile -> ["rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::compile_main_render_graph::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e890f -> 0x161cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e940a -> 0x161cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>
// 信息 ID: 2d1e228aca3fbed6113e461b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] rhapsody::rg::state::RenderGraphState::compile -> ["tuple$<>", "country_core::render::pass_builder::pass_resources::PassResources", "tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0xfb609 -> 0x166510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::state::impl$0::default
// 信息 ID: a815860fa8412ec027d5186f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd74b4 -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd74c0 -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a6058 -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a6064 -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a5e92 -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a5e9e -> 0xbe57d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
