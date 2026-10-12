// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e6c0e7ece82750d821303fdaf46fcab79c032a89fef25473b5ed2c5912a04714
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 308, 1067, 2588, 2589, 2599, 2605, 2606, 2607, 2608, 2611, 2613, 3799, 3801, 3804, 3805, 3806, 3810, 3811, 4480, 4706, 4803
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: alloc::collections::btree::node::Handle::insert_recursing<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2589,"record_offset":12292,"rva":30083488,"va":5398792608}
// 编译实例 RVAs: 0x1cb09a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..btree..mem..replace..PanicGuard$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15957612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e894810cff","size":5,"soft_fail":false,"target_va":5382838208,"va":5398795815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23713416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8782996fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5398794899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23714251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8352696fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5398795750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141caf720
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e858e1ffff","size":5,"soft_fail":false,"target_va":5398787872,"va":5398795715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13054295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e85731c700","size":5,"soft_fail":false,"target_va":5411847184,"va":5398792884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12745317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8657ac200","size":5,"soft_fail":false,"target_va":5411540992,"va":5398795670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle::split<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-3854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8f2f0ffff","size":5,"soft_fail":false,"target_va":5398788976,"va":5398792825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12746118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8867dc200","size":5,"soft_fail":false,"target_va":5411541776,"va":5398795653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12746453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398792608,"raw_hex":"e8d57ec200","size":5,"soft_fail":false,"target_va":5411542064,"va":5398795606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::map::entry::impl$8::insert_entry::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map\entry.rs
//     evidence={"module":2589,"record_offset":17224,"parent_record_offset":12292,"ancestor_record_offset":12292,"inlinee":13239,"source_line":412,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::Handle::awaken
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":13812,"parent_record_offset":12292,"ancestor_record_offset":12292,"inlinee":13237,"source_line":858,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::Handle::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":12676,"parent_record_offset":12292,"ancestor_record_offset":12292,"inlinee":13231,"source_line":1011,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":13840,"parent_record_offset":12292,"ancestor_record_offset":12292,"inlinee":10179,"source_line":318,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2589,"record_offset":17652,"parent_record_offset":17628,"ancestor_record_offset":12292,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2589,"record_offset":17676,"parent_record_offset":17652,"ancestor_record_offset":12292,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2589,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":12292,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2589,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":12292,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2589,"record_offset":13732,"parent_record_offset":13676,"ancestor_record_offset":12292,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2589,"record_offset":17572,"parent_record_offset":17536,"ancestor_record_offset":12292,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2589,"record_offset":17604,"parent_record_offset":17572,"ancestor_record_offset":12292,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2589,"record_offset":17372,"parent_record_offset":17340,"ancestor_record_offset":12292,"inlinee":10172,"source_line":18,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::take_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2589,"record_offset":17340,"parent_record_offset":17308,"ancestor_record_offset":12292,"inlinee":13241,"source_line":9,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::take_mut::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\mem.rs
//     evidence={"module":2589,"record_offset":17444,"parent_record_offset":17372,"ancestor_record_offset":12292,"inlinee":13243,"source_line":10,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::correct_parent_link
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":14812,"parent_record_offset":14600,"ancestor_record_offset":12292,"inlinee":13219,"source_line":980,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":14864,"parent_record_offset":14812,"ancestor_record_offset":12292,"inlinee":5891,"source_line":1093,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::insert_fit
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":12824,"parent_record_offset":12676,"ancestor_record_offset":12292,"inlinee":13232,"source_line":993,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::new_kv
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":13128,"parent_record_offset":12676,"ancestor_record_offset":12292,"inlinee":13214,"source_line":792,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::InternalNode::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17536,"parent_record_offset":17500,"ancestor_record_offset":12292,"inlinee":13222,"source_line":113,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::LeafNode::init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17752,"parent_record_offset":17536,"ancestor_record_offset":12292,"inlinee":13218,"source_line":70,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::as_leaf_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":12292,"inlinee":13212,"source_line":300,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::correct_all_childrens_parent_links
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":12292,"inlinee":13224,"source_line":554,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::correct_childrens_parent_links
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":14600,"parent_record_offset":14148,"ancestor_record_offset":12292,"inlinee":13225,"source_line":547,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::from_new_internal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17928,"parent_record_offset":17500,"ancestor_record_offset":12292,"inlinee":13223,"source_line":234,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":12768,"parent_record_offset":12676,"ancestor_record_offset":12292,"inlinee":10177,"source_line":277,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::new_internal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":12292,"inlinee":13246,"source_line":226,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":18180,"parent_record_offset":17224,"ancestor_record_offset":12292,"inlinee":13247,"source_line":684,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::push_internal_level
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17308,"parent_record_offset":17224,"ancestor_record_offset":12292,"inlinee":13240,"source_line":588,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::reborrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":14120,"parent_record_offset":13900,"ancestor_record_offset":12292,"inlinee":13235,"source_line":426,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::set_parent_link
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":15040,"parent_record_offset":14812,"ancestor_record_offset":12292,"inlinee":13220,"source_line":563,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$30::push_internal_level::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":17472,"parent_record_offset":17444,"ancestor_record_offset":12292,"inlinee":13245,"source_line":592,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::slice_insert
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":12872,"parent_record_offset":12824,"ancestor_record_offset":12292,"inlinee":13233,"source_line":1812,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::splitpoint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":2589,"record_offset":13172,"parent_record_offset":12676,"ancestor_record_offset":12292,"inlinee":13234,"source_line":905,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2589,"record_offset":13704,"parent_record_offset":13676,"ancestor_record_offset":12292,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2589,"record_offset":13676,"parent_record_offset":13652,"ancestor_record_offset":12292,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2589,"record_offset":13652,"parent_record_offset":13628,"ancestor_record_offset":12292,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2589,"record_offset":14720,"parent_record_offset":14680,"ancestor_record_offset":12292,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$96::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":2589,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":12292,"inlinee":10844,"source_line":77,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2589,"record_offset":14748,"parent_record_offset":14680,"ancestor_record_offset":12292,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2589,"record_offset":14680,"parent_record_offset":14644,"ancestor_record_offset":12292,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2589,"record_offset":14644,"parent_record_offset":14600,"ancestor_record_offset":12292,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2589,"record_offset":14904,"parent_record_offset":14864,"ancestor_record_offset":12292,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2589,"record_offset":12944,"parent_record_offset":12872,"ancestor_record_offset":12292,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2589,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":12292,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2589,"record_offset":17272,"parent_record_offset":17224,"ancestor_record_offset":12292,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2589,"record_offset":13868,"parent_record_offset":13840,"ancestor_record_offset":12292,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2589,"record_offset":14944,"parent_record_offset":14904,"ancestor_record_offset":12292,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2589,"record_offset":13580,"parent_record_offset":12676,"ancestor_record_offset":12292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2589,"record_offset":12916,"parent_record_offset":12872,"ancestor_record_offset":12292,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2589,"record_offset":17780,"parent_record_offset":17752,"ancestor_record_offset":12292,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2589,"record_offset":14984,"parent_record_offset":14944,"ancestor_record_offset":12292,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2589,"record_offset":17704,"parent_record_offset":17676,"ancestor_record_offset":12292,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2589,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":12292,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=12
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2599,"record_offset":1452,"rva":30216960,"va":5398926080}
// 编译实例 RVAs: 0x1cd1300, 0x1ce3830, 0x1ce56c0, 0x1ce6370, 0x22c9a30, 0x22cc150, 0x22ce020, 0x22ceda0, 0x22d02a0, 0x22d4630, 0x22d66b0, 0x2609230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=48
//     evidence={"assembly":"\tcallq\t12615712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398926080,"raw_hex":"e82080c000","size":5,"soft_fail":false,"target_va":5411542296,"va":5398926579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2599,"record_offset":1728,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2599,"record_offset":1856,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2599,"record_offset":1636,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2599,"record_offset":2008,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2599,"record_offset":1812,"parent_record_offset":1784,"ancestor_record_offset":1452,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2599,"record_offset":1944,"parent_record_offset":1884,"ancestor_record_offset":1452,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2599,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1452,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2599,"record_offset":1784,"parent_record_offset":1756,"ancestor_record_offset":1452,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2599,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":1452,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2599,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1452,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2599,"record_offset":1688,"parent_record_offset":1636,"ancestor_record_offset":1452,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4706,"record_offset":1964,"rva":360400,"va":5369069520}
// 编译实例 RVAs: 0x57fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42241493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369069520,"raw_hex":"e8d58d8402","size":5,"soft_fail":false,"target_va":5411311520,"va":5369070022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer$LT$R$GT$::parse_exponent_overflow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42407592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369069520,"raw_hex":"e8a8168702","size":5,"soft_fail":false,"target_va":5411477328,"va":5369069731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4706,"record_offset":2716,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":6492,"source_line":1668,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4706,"record_offset":2744,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":6493,"source_line":1714,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":2256,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":2772,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":6482,"source_line":562,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":2412,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":2140,"parent_record_offset":1964,"ancestor_record_offset":1964,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":4706,"record_offset":2964,"parent_record_offset":2772,"ancestor_record_offset":1964,"inlinee":6486,"source_line":546,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4706,"record_offset":2840,"parent_record_offset":2772,"ancestor_record_offset":1964,"inlinee":6484,"source_line":2192,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4706,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":1964,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4706,"record_offset":2896,"parent_record_offset":2840,"ancestor_record_offset":1964,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":1964,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2276,"parent_record_offset":2256,"ancestor_record_offset":1964,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1964,"inlinee":6479,"source_line":632,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1964,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2296,"parent_record_offset":2276,"ancestor_record_offset":1964,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1964,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1964,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_number<serde_json::read::StrRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4706,"record_offset":404,"rva":358192,"va":5369067312}
// 编译实例 RVAs: 0x57730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42241815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369067312,"raw_hex":"e8178f8402","size":5,"soft_fail":false,"target_va":5411309856,"va":5369068036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42243396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369067312,"raw_hex":"e844958402","size":5,"soft_fail":false,"target_va":5411311520,"va":5369068119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer$LT$R$GT$::parse_decimal_overflow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42409287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369067312,"raw_hex":"e8471d8702","size":5,"soft_fail":false,"target_va":5411476960,"va":5369067668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369067312,"raw_hex":"e84f080000","size":5,"soft_fail":false,"target_va":5369069520,"va":5369067388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_decimal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":680,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":6481,"source_line":456,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":564,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":4706,"record_offset":1264,"parent_record_offset":1072,"ancestor_record_offset":404,"inlinee":6486,"source_line":546,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4706,"record_offset":1140,"parent_record_offset":1072,"ancestor_record_offset":404,"inlinee":6484,"source_line":2192,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4706,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":404,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4706,"record_offset":1196,"parent_record_offset":1140,"ancestor_record_offset":404,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":772,"parent_record_offset":680,"ancestor_record_offset":404,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::f64_from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":1072,"parent_record_offset":680,"ancestor_record_offset":404,"inlinee":6482,"source_line":562,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":404,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4706,"record_offset":856,"parent_record_offset":680,"ancestor_record_offset":404,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":404,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":404,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":404,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4706,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":404,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/04706.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4803,"record_offset":260,"rva":42535536,"va":5411244656}
// 编译实例 RVAs: 0x2890a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36163510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e84a30d8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5411245009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed4560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26987781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8fb3264fe","size":5,"soft_fail":false,"target_va":5384258912,"va":5411246688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed47d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26986051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8bd3964fe","size":5,"soft_fail":false,"target_va":5384259536,"va":5411245582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed5750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26982449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8cf4764fe","size":5,"soft_fail":false,"target_va":5384263504,"va":5411245948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed7380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26975250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8ee6364fe","size":5,"soft_fail":false,"target_va":5384270720,"va":5411245965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ed74f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26974682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8266664fe","size":5,"soft_fail":false,"target_va":5384271088,"va":5411245765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14285e720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-205959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e879dbfcff","size":5,"soft_fail":false,"target_va":5411039008,"va":5411244962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14285f780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-201518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8d2ecfcff","size":5,"soft_fail":false,"target_va":5411043200,"va":5411244713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14285f900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-201601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e87fecfcff","size":5,"soft_fail":false,"target_va":5411043584,"va":5411245180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t64920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e898fd0000","size":5,"soft_fail":false,"target_va":5411309856,"va":5411244931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12464950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e8cacc41ff","size":5,"soft_fail":false,"target_va":5398780176,"va":5411245121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-12465019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411244656,"raw_hex":"e885cc41ff","size":5,"soft_fail":false,"target_va":5398780720,"va":5411245734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4803,"record_offset":1036,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4803,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4803,"record_offset":512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4803,"record_offset":1868,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6627,"source_line":1876,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4803,"record_offset":568,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":4803,"record_offset":1548,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13849,"source_line":125,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":4803,"record_offset":540,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13848,"source_line":38,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4803,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4803,"record_offset":1188,"parent_record_offset":1132,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4803,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4803,"record_offset":1496,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":4803,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":260,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4803,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4803,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4803,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":260,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4803,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4803,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4803,"record_offset":1364,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_entry_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4803,"record_offset":1912,"parent_record_offset":1868,"ancestor_record_offset":260,"inlinee":6628,"source_line":1822,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4803,"record_offset":1000,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4803,"record_offset":824,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4803,"record_offset":636,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4803,"record_offset":676,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4803,"record_offset":720,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":4803,"record_offset":600,"parent_record_offset":568,"ancestor_record_offset":260,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4803,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":260,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4803,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4803,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":260,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":4803,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":260,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/04803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3811,"record_offset":7600,"rva":36531920,"va":5405241040}
// 编译实例 RVAs: 0x22d6ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22598326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e84a2da7fe","size":5,"soft_fail":false,"target_va":5382644528,"va":5405242849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36298286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d221d6fd","size":5,"soft_fail":false,"target_va":5368983552,"va":5405281833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140542890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31017770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d6b426fe","size":5,"soft_fail":false,"target_va":5374224528,"va":5405242293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30464983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e829242ffe","size":5,"soft_fail":false,"target_va":5374814320,"va":5405279298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30160872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e818c833fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405242355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30171307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8559f33fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405252806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29726652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e844683afe","size":5,"soft_fail":false,"target_va":5375516032,"va":5405242679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140899a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27555799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e829885bfe","size":5,"soft_fail":false,"target_va":5377727088,"va":5405282882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140960ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26699630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8929868fe","size":5,"soft_fail":false,"target_va":5378542272,"va":5405241897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ea800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26145176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8680e71fe","size":5,"soft_fail":false,"target_va":5379106816,"va":5405251987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25965641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8b7cb73fe","size":5,"soft_fail":false,"target_va":5379280368,"va":5405246004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aac630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25341245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8c3527dfe","size":5,"soft_fail":false,"target_va":5379900976,"va":5405242216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24618977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e81f5888fe","size":5,"soft_fail":false,"target_va":5380623152,"va":5405242124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24627594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8763688fe","size":5,"soft_fail":false,"target_va":5380624688,"va":5405252277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23311136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8e04c9cfe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405245659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23310976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8804d9cfe","size":5,"soft_fail":false,"target_va":5381934560,"va":5405245531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eecf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20882202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8e65cc1fe","size":5,"soft_fail":false,"target_va":5384359680,"va":5405241877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fabc80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20103501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8b33ecdfe","size":5,"soft_fail":false,"target_va":5385141376,"va":5405244872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17371650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8feedf6fe","size":5,"soft_fail":false,"target_va":5387874224,"va":5405245869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141247c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17364456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8180af7fe","size":5,"soft_fail":false,"target_va":5387877520,"va":5405241971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14145c1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15185620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e82c4918ff","size":5,"soft_fail":false,"target_va":5390057888,"va":5405243503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141762b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12026573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8337d48ff","size":5,"soft_fail":false,"target_va":5393230736,"va":5405257304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cd64a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6305909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e88bc79fff","size":5,"soft_fail":false,"target_va":5398946976,"va":5405252880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf3900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6185528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8c89da1ff","size":5,"soft_fail":false,"target_va":5399066880,"va":5405252403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf6a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6170194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8aed9a1ff","size":5,"soft_fail":false,"target_va":5399079520,"va":5405249709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142612d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3373624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8387a3300","size":5,"soft_fail":false,"target_va":5408632080,"va":5405258451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6597369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8f9aa6400","size":5,"soft_fail":false,"target_va":5411847152,"va":5405249778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6588249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e859876400","size":5,"soft_fail":false,"target_va":5411847184,"va":5405258930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6601169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d1b96400","size":5,"soft_fail":false,"target_va":5411847808,"va":5405246634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6602447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8cfbe6400","size":5,"soft_fail":false,"target_va":5411847904,"va":5405245452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6601493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e815bb6400","size":5,"soft_fail":false,"target_va":5411848144,"va":5405246646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6267811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8a3a35f00","size":5,"soft_fail":false,"target_va":5411540992,"va":5405273176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6462853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87b629dff","size":5,"soft_fail":false,"target_va":5398779792,"va":5405242640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22412397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e89303aafe","size":5,"soft_fail":false,"target_va":5382840208,"va":5405252600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30901418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8567b28fe","size":5,"soft_fail":false,"target_va":5374357488,"va":5405258901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t6526097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e891946300","size":5,"soft_fail":false,"target_va":5411768560,"va":5405242458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6268382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8dea55f00","size":5,"soft_fail":false,"target_va":5411540963,"va":5405272576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t6270285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e84dad5f00","size":5,"soft_fail":false,"target_va":5411541584,"va":5405271294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t6269749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e835ab5f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405272022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6211024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d0c55e00","size":5,"soft_fail":false,"target_va":5411482433,"va":5405271404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6270198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8f6ac5f00","size":5,"soft_fail":false,"target_va":5411542064,"va":5405271861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6270180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8e4ac5f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5405272111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6270326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e876ad5f00","size":5,"soft_fail":false,"target_va":5411542880,"va":5405272549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e848f5ffff","size":5,"soft_fail":false,"target_va":5405238464,"va":5405241203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t6302075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87b296000","size":5,"soft_fail":false,"target_va":5411543360,"va":5405241280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::is_halloween2024
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24711242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8b6ef86fe","size":5,"soft_fail":false,"target_va":5380532320,"va":5405243557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24833667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87d1185fe","size":5,"soft_fail":false,"target_va":5380417152,"va":5405250814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25460229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8fb817bfe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405252848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get_unchecked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25457881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8278b7bfe","size":5,"soft_fail":false,"target_va":5379792912,"va":5405250788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27310357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8eb465ffe","size":5,"soft_fail":false,"target_va":5377935472,"va":5405245824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25111177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e877d580fe","size":5,"soft_fail":false,"target_va":5380131872,"va":5405243044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25383844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e85cac7cfe","size":5,"soft_fail":false,"target_va":5379873296,"va":5405257135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_rectangle2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25374663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e839d07cfe","size":5,"soft_fail":false,"target_va":5379873696,"va":5405248354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24994133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8ab9e82fe","size":5,"soft_fail":false,"target_va":5380263600,"va":5405257728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24994161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e88f9e82fe","size":5,"soft_fail":false,"target_va":5380263536,"va":5405257692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24993960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8589f82fe","size":5,"soft_fail":false,"target_va":5380263792,"va":5405257747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::color
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24996926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8c29382fe","size":5,"soft_fail":false,"target_va":5380255296,"va":5405252217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::line_width
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24999630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8328982fe","size":5,"soft_fail":false,"target_va":5380255344,"va":5405254969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24996931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8bd9382fe","size":5,"soft_fail":false,"target_va":5380255248,"va":5405252174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::show
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24996808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8389482fe","size":5,"soft_fail":false,"target_va":5380255440,"va":5405252243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26699395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87d9968fe","size":5,"soft_fail":false,"target_va":5378542640,"va":5405242030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23235401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8b7749dfe","size":5,"soft_fail":false,"target_va":5382009728,"va":5405245124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6249522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8325c5f00","size":5,"soft_fail":false,"target_va":5411491664,"va":5405242137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23418034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e84eab9afe","size":5,"soft_fail":false,"target_va":5381824016,"va":5405242045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23856171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d5fb93fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405241702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23850982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e81a1094fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405241617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23856503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e889fa93fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405242978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23856485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e89bfa93fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405241184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6266920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e828a05f00","size":5,"soft_fail":false,"target_va":5411538608,"va":5405271683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6267347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d3a15f00","size":5,"soft_fail":false,"target_va":5411538960,"va":5405271608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::futex::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6267421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e81da25f00","size":5,"soft_fail":false,"target_va":5411538976,"va":5405271550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t6561148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87c1d6400","size":5,"soft_fail":false,"target_va":5411809152,"va":5405247999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t-20880491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e89563c1fe","size":5,"soft_fail":false,"target_va":5384376048,"va":5405256534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t6265817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d99b5f00","size":5,"soft_fail":false,"target_va":5411537104,"va":5405271282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::AutoClutterState::flush_candidates_pending_removal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20915762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8ced9c0fe","size":5,"soft_fail":false,"target_va":5384326128,"va":5405241885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::AutoClutterState::mark_dirty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20916919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e849d5c0fe","size":5,"soft_fail":false,"target_va":5384325312,"va":5405242226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_radius
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17406642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e84e65f6fe","size":5,"soft_fail":false,"target_va":5387845744,"va":5405252381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23339180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e854df9bfe","size":5,"soft_fail":false,"target_va":5381914864,"va":5405254039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23328542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8e2089cfe","size":5,"soft_fail":false,"target_va":5381928576,"va":5405257113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23329464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e848059cfe","size":5,"soft_fail":false,"target_va":5381924512,"va":5405253971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23331284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e82cfe9bfe","size":5,"soft_fail":false,"target_va":5381917056,"va":5405248335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23329529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e807059cfe","size":5,"soft_fail":false,"target_va":5381927968,"va":5405257492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23339310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8d2de9bfe","size":5,"soft_fail":false,"target_va":5381915520,"va":5405254825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23338994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e80ee09bfe","size":5,"soft_fail":false,"target_va":5381916032,"va":5405255021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::set_orientation_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23331184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e890fe9bfe","size":5,"soft_fail":false,"target_va":5381915936,"va":5405247115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::translate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23331203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e87dfe9bfe","size":5,"soft_fail":false,"target_va":5381915840,"va":5405247038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng_u64
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23338054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405241040,"raw_hex":"e8bae39bfe","size":5,"soft_fail":false,"target_va":5381907056,"va":5405245105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5405241040,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5405249809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":3811,"record_offset":14240,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":6029,"source_line":905,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$26::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3811,"record_offset":12756,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":19790,"source_line":2642,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3811,"record_offset":12208,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":95520,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":17176,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":15896,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":74768,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":10620,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":18644,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3811,"record_offset":10440,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":24556,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":14352,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3811,"record_offset":74632,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":24416,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":24388,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3811,"record_offset":19368,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":21504,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3811,"record_offset":10968,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3811,"record_offset":12164,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3811,"record_offset":74696,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":10880,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Handle::is_valid
//     owner=private_source_ownership_candidate; count=172
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":3811,"record_offset":17204,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":121709,"source_line":106,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::impl$92::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3811,"record_offset":24616,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":121665,"source_line":396,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":3811,"record_offset":21572,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":24398,"source_line":108,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":3811,"record_offset":11668,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":32545,"source_line":160,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":22868,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":21648,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":23332,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":23508,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":23772,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::as_ivec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":17116,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":28189,"source_line":1017,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":17024,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3811,"record_offset":21676,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3811,"record_offset":14384,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3811,"record_offset":14268,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":9424,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":7992,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":8448,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$45::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3811,"record_offset":15952,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":19936,"source_line":2352,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3811,"record_offset":16312,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3811,"record_offset":21816,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3811,"record_offset":21844,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":3811,"record_offset":14184,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":3811,"record_offset":12040,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterRegion::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":21216,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58549,"source_line":516,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterRegion::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":21476,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58575,"source_line":460,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterRegion::rectangle2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":25832,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58548,"source_line":521,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterRegion::sample_with_margin
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":19948,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58576,"source_line":477,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterState::remove_stale_regions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":10500,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58567,"source_line":233,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterCandidateIndex::is_none
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":16996,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58572,"source_line":355,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterCandidateIndex::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":74732,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":58571,"source_line":341,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::align_to_nearby_wall
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":3811,"record_offset":27452,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":121713,"source_line":944,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":3811,"record_offset":44104,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":121714,"source_line":688,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::pick_clutter_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":3811,"record_offset":19424,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":121711,"source_line":918,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] tabloid::impl$3::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":3811,"record_offset":10940,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":24756,"source_line":164,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3811,"record_offset":24500,"parent_record_offset":7600,"ancestor_record_offset":7600,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":7600,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":7600,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":7600,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":66796,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":12572,"parent_record_offset":12544,"ancestor_record_offset":7600,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3811,"record_offset":13320,"parent_record_offset":13256,"ancestor_record_offset":7600,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3811,"record_offset":73684,"parent_record_offset":73660,"ancestor_record_offset":7600,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3811,"record_offset":37140,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3811,"record_offset":37188,"parent_record_offset":37164,"ancestor_record_offset":7600,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":7600,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":7600,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":12480,"parent_record_offset":12448,"ancestor_record_offset":7600,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":7600,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":13288,"parent_record_offset":13256,"ancestor_record_offset":7600,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":7600,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":7600,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":7600,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":7600,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":7600,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":12544,"parent_record_offset":12512,"ancestor_record_offset":7600,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":12512,"parent_record_offset":12480,"ancestor_record_offset":7600,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3811,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":7600,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":12412,"parent_record_offset":12380,"ancestor_record_offset":7600,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":66988,"parent_record_offset":66960,"ancestor_record_offset":7600,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":7600,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":66960,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":12316,"parent_record_offset":12272,"ancestor_record_offset":7600,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":12348,"parent_record_offset":12316,"ancestor_record_offset":7600,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3811,"record_offset":12272,"parent_record_offset":12240,"ancestor_record_offset":7600,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3811,"record_offset":12240,"parent_record_offset":12208,"ancestor_record_offset":7600,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3811,"record_offset":12796,"parent_record_offset":12756,"ancestor_record_offset":7600,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3811,"record_offset":11396,"parent_record_offset":10992,"ancestor_record_offset":7600,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3811,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":7600,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3811,"record_offset":74496,"parent_record_offset":74460,"ancestor_record_offset":7600,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3811,"record_offset":24668,"parent_record_offset":24616,"ancestor_record_offset":7600,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3811,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":7600,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":7600,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":7600,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":10676,"parent_record_offset":10644,"ancestor_record_offset":7600,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":52908,"parent_record_offset":52884,"ancestor_record_offset":7600,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":52884,"parent_record_offset":52852,"ancestor_record_offset":7600,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":65976,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":7600,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":7600,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":67016,"parent_record_offset":66988,"ancestor_record_offset":7600,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":53464,"parent_record_offset":53440,"ancestor_record_offset":7600,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":13112,"parent_record_offset":12868,"ancestor_record_offset":7600,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":12904,"parent_record_offset":12868,"ancestor_record_offset":7600,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":12448,"parent_record_offset":12412,"ancestor_record_offset":7600,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":19080,"parent_record_offset":19052,"ancestor_record_offset":7600,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":52852,"parent_record_offset":52824,"ancestor_record_offset":7600,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3811,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":7600,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3811,"record_offset":19152,"parent_record_offset":19080,"ancestor_record_offset":7600,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3811,"record_offset":19324,"parent_record_offset":18708,"ancestor_record_offset":7600,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3811,"record_offset":76848,"parent_record_offset":76824,"ancestor_record_offset":7600,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3811,"record_offset":18676,"parent_record_offset":18644,"ancestor_record_offset":7600,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3811,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":7600,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3811,"record_offset":55004,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3811,"record_offset":54224,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3811,"record_offset":54308,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3811,"record_offset":55200,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3811,"record_offset":54140,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3811,"record_offset":10468,"parent_record_offset":10440,"ancestor_record_offset":7600,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3811,"record_offset":54916,"parent_record_offset":54892,"ancestor_record_offset":7600,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3811,"record_offset":54400,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3811,"record_offset":53524,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3811,"record_offset":54892,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3811,"record_offset":54940,"parent_record_offset":54916,"ancestor_record_offset":7600,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3811,"record_offset":52800,"parent_record_offset":52716,"ancestor_record_offset":7600,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3811,"record_offset":52824,"parent_record_offset":52800,"ancestor_record_offset":7600,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3811,"record_offset":53124,"parent_record_offset":53100,"ancestor_record_offset":7600,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3811,"record_offset":53100,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3811,"record_offset":53940,"parent_record_offset":53916,"ancestor_record_offset":7600,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3811,"record_offset":53624,"parent_record_offset":53600,"ancestor_record_offset":7600,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3811,"record_offset":53600,"parent_record_offset":53576,"ancestor_record_offset":7600,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3811,"record_offset":53800,"parent_record_offset":53776,"ancestor_record_offset":7600,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":53576,"parent_record_offset":53548,"ancestor_record_offset":7600,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":53548,"parent_record_offset":53524,"ancestor_record_offset":7600,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":53440,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":54964,"parent_record_offset":54940,"ancestor_record_offset":7600,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":53356,"parent_record_offset":53124,"ancestor_record_offset":7600,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3811,"record_offset":53916,"parent_record_offset":53892,"ancestor_record_offset":7600,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3811,"record_offset":53892,"parent_record_offset":53868,"ancestor_record_offset":7600,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3811,"record_offset":52716,"parent_record_offset":52692,"ancestor_record_offset":7600,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3811,"record_offset":52768,"parent_record_offset":52744,"ancestor_record_offset":7600,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3811,"record_offset":52692,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3811,"record_offset":52744,"parent_record_offset":52716,"ancestor_record_offset":7600,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3811,"record_offset":55328,"parent_record_offset":55304,"ancestor_record_offset":7600,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3811,"record_offset":18912,"parent_record_offset":18884,"ancestor_record_offset":7600,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3811,"record_offset":19024,"parent_record_offset":18996,"ancestor_record_offset":7600,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9768,"parent_record_offset":9740,"ancestor_record_offset":7600,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=78
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":7600,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=78
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9796,"parent_record_offset":9768,"ancestor_record_offset":7600,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9716,"parent_record_offset":9684,"ancestor_record_offset":7600,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":7600,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":9912,"parent_record_offset":9740,"ancestor_record_offset":7600,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3811,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":7600,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=172
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":7600,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":7600,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":7600,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3811,"record_offset":45008,"parent_record_offset":44968,"ancestor_record_offset":7600,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3811,"record_offset":11980,"parent_record_offset":11668,"ancestor_record_offset":7600,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":3811,"record_offset":74660,"parent_record_offset":74632,"ancestor_record_offset":7600,"inlinee":38980,"source_line":255,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3811,"record_offset":16164,"parent_record_offset":16136,"ancestor_record_offset":7600,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3811,"record_offset":16224,"parent_record_offset":16192,"ancestor_record_offset":7600,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3811,"record_offset":20716,"parent_record_offset":20628,"ancestor_record_offset":7600,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3811,"record_offset":20656,"parent_record_offset":20628,"ancestor_record_offset":7600,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3811,"record_offset":20344,"parent_record_offset":20320,"ancestor_record_offset":7600,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":21612,"parent_record_offset":21572,"ancestor_record_offset":7600,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3811,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":7600,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3811,"record_offset":37224,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3811,"record_offset":12696,"parent_record_offset":12672,"ancestor_record_offset":7600,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":3811,"record_offset":18940,"parent_record_offset":18912,"ancestor_record_offset":7600,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3811,"record_offset":11736,"parent_record_offset":11708,"ancestor_record_offset":7600,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3811,"record_offset":11708,"parent_record_offset":11668,"ancestor_record_offset":7600,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3811,"record_offset":19524,"parent_record_offset":19500,"ancestor_record_offset":7600,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3811,"record_offset":19596,"parent_record_offset":19548,"ancestor_record_offset":7600,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3811,"record_offset":19392,"parent_record_offset":19368,"ancestor_record_offset":7600,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":3811,"record_offset":19500,"parent_record_offset":19476,"ancestor_record_offset":7600,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":3811,"record_offset":19624,"parent_record_offset":19596,"ancestor_record_offset":7600,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3811,"record_offset":18884,"parent_record_offset":18708,"ancestor_record_offset":7600,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3811,"record_offset":19052,"parent_record_offset":19024,"ancestor_record_offset":7600,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3811,"record_offset":19476,"parent_record_offset":19424,"ancestor_record_offset":7600,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3811,"record_offset":18968,"parent_record_offset":18940,"ancestor_record_offset":7600,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3811,"record_offset":11028,"parent_record_offset":10992,"ancestor_record_offset":7600,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3811,"record_offset":80996,"parent_record_offset":80964,"ancestor_record_offset":7600,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=79
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3811,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":7600,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3811,"record_offset":11952,"parent_record_offset":11668,"ancestor_record_offset":7600,"inlinee":10102,"source_line":161,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3811,"record_offset":19660,"parent_record_offset":19548,"ancestor_record_offset":7600,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3811,"record_offset":95692,"parent_record_offset":95660,"ancestor_record_offset":7600,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3811,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":7600,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3811,"record_offset":16604,"parent_record_offset":16572,"ancestor_record_offset":7600,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3811,"record_offset":16572,"parent_record_offset":16540,"ancestor_record_offset":7600,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":20044,"parent_record_offset":20016,"ancestor_record_offset":7600,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":20228,"parent_record_offset":20200,"ancestor_record_offset":7600,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":17056,"parent_record_offset":17024,"ancestor_record_offset":7600,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$247::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":3811,"record_offset":19888,"parent_record_offset":19424,"ancestor_record_offset":7600,"inlinee":28528,"source_line":35,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":33792,"parent_record_offset":33764,"ancestor_record_offset":7600,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":7600,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3811,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":7600,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3811,"record_offset":9576,"parent_record_offset":9532,"ancestor_record_offset":7600,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3811,"record_offset":80896,"parent_record_offset":80856,"ancestor_record_offset":7600,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3811,"record_offset":33020,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3811,"record_offset":33044,"parent_record_offset":33020,"ancestor_record_offset":7600,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":3811,"record_offset":18996,"parent_record_offset":18968,"ancestor_record_offset":7600,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":53736,"parent_record_offset":53712,"ancestor_record_offset":7600,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":11916,"parent_record_offset":11736,"ancestor_record_offset":7600,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":51728,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":37164,"parent_record_offset":37140,"ancestor_record_offset":7600,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3811,"record_offset":32984,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3811,"record_offset":14440,"parent_record_offset":14408,"ancestor_record_offset":7600,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3811,"record_offset":14468,"parent_record_offset":14408,"ancestor_record_offset":7600,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":7600,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":10392,"parent_record_offset":9532,"ancestor_record_offset":7600,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":7600,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":12672,"parent_record_offset":12412,"ancestor_record_offset":7600,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":81028,"parent_record_offset":80996,"ancestor_record_offset":7600,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3811,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":7600,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=192
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3811,"record_offset":9976,"parent_record_offset":9684,"ancestor_record_offset":7600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3811,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":7600,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3811,"record_offset":16708,"parent_record_offset":16684,"ancestor_record_offset":7600,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3811,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":7600,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3811,"record_offset":74528,"parent_record_offset":74496,"ancestor_record_offset":7600,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3811,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":7600,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3811,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":7600,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3811,"record_offset":9044,"parent_record_offset":9016,"ancestor_record_offset":7600,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3811,"record_offset":11336,"parent_record_offset":11308,"ancestor_record_offset":7600,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3811,"record_offset":66912,"parent_record_offset":66884,"ancestor_record_offset":7600,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=82
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3811,"record_offset":9868,"parent_record_offset":9844,"ancestor_record_offset":7600,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3811,"record_offset":45092,"parent_record_offset":45064,"ancestor_record_offset":7600,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3811,"record_offset":45120,"parent_record_offset":45092,"ancestor_record_offset":7600,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3811,"record_offset":61684,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3811,"record_offset":53380,"parent_record_offset":53356,"ancestor_record_offset":7600,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3811,"record_offset":28828,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3811,"record_offset":31956,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3811,"record_offset":61748,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3811,"record_offset":38892,"parent_record_offset":38816,"ancestor_record_offset":7600,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3811,"record_offset":53404,"parent_record_offset":53380,"ancestor_record_offset":7600,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3811,"record_offset":38924,"parent_record_offset":38892,"ancestor_record_offset":7600,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3811,"record_offset":31980,"parent_record_offset":31956,"ancestor_record_offset":7600,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":7600,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":7600,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":8812,"parent_record_offset":8788,"ancestor_record_offset":7600,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3811,"record_offset":10816,"parent_record_offset":10644,"ancestor_record_offset":7600,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":9116,"parent_record_offset":9088,"ancestor_record_offset":7600,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":19548,"parent_record_offset":19524,"ancestor_record_offset":7600,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":11764,"parent_record_offset":11736,"ancestor_record_offset":7600,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":8764,"parent_record_offset":8648,"ancestor_record_offset":7600,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3811,"record_offset":11800,"parent_record_offset":11764,"ancestor_record_offset":7600,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3811,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7600,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3811,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":7600,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3811,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":7600,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3811,"record_offset":8584,"parent_record_offset":8548,"ancestor_record_offset":7600,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3811,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":7600,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3811,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":7600,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3811,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":7600,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3811,"record_offset":9088,"parent_record_offset":8548,"ancestor_record_offset":7600,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":7600,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7600,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8248,"parent_record_offset":8216,"ancestor_record_offset":7600,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8344,"parent_record_offset":8248,"ancestor_record_offset":7600,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7600,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":45036,"parent_record_offset":45008,"ancestor_record_offset":7600,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8028,"parent_record_offset":7992,"ancestor_record_offset":7600,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3811,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7600,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3811,"record_offset":8648,"parent_record_offset":8616,"ancestor_record_offset":7600,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3811,"record_offset":8932,"parent_record_offset":8648,"ancestor_record_offset":7600,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3811,"record_offset":8904,"parent_record_offset":8648,"ancestor_record_offset":7600,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3811,"record_offset":8708,"parent_record_offset":8648,"ancestor_record_offset":7600,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":14828,"parent_record_offset":14632,"ancestor_record_offset":7600,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":7600,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":14928,"parent_record_offset":14896,"ancestor_record_offset":7600,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":24696,"parent_record_offset":24668,"ancestor_record_offset":7600,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":11236,"parent_record_offset":11112,"ancestor_record_offset":7600,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":7600,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":7600,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":7600,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":7600,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":11260,"parent_record_offset":11236,"ancestor_record_offset":7600,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":7600,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3811,"record_offset":11624,"parent_record_offset":11532,"ancestor_record_offset":7600,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=172
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":3811,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":7600,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3811,"record_offset":44936,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3811,"record_offset":44968,"parent_record_offset":44936,"ancestor_record_offset":7600,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3811,"record_offset":44880,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3811,"record_offset":20860,"parent_record_offset":20832,"ancestor_record_offset":7600,"inlinee":32522,"source_line":207,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3811,"record_offset":20164,"parent_record_offset":20080,"ancestor_record_offset":7600,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3811,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":7600,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::transpose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3811,"record_offset":20320,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":30692,"source_line":274,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":3811,"record_offset":20604,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":66360,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":22952,"parent_record_offset":22920,"ancestor_record_offset":7600,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20832,"parent_record_offset":20808,"ancestor_record_offset":7600,"inlinee":32521,"source_line":607,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::fract_gl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20808,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":30965,"source_line":656,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":22920,"parent_record_offset":22892,"ancestor_record_offset":7600,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":7600,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":7600,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32512,"parent_record_offset":32488,"ancestor_record_offset":7600,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20436,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":33492,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":7600,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20492,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20200,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20752,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20016,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":32076,"parent_record_offset":32040,"ancestor_record_offset":7600,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20548,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":20380,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3811,"record_offset":66272,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":62108,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":33312,"parent_record_offset":33288,"ancestor_record_offset":7600,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":33400,"parent_record_offset":33376,"ancestor_record_offset":7600,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":62488,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":24580,"parent_record_offset":24556,"ancestor_record_offset":7600,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":21360,"parent_record_offset":21216,"ancestor_record_offset":7600,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":21416,"parent_record_offset":21216,"ancestor_record_offset":7600,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":33764,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3811,"record_offset":33196,"parent_record_offset":33164,"ancestor_record_offset":7600,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":3811,"record_offset":17144,"parent_record_offset":17116,"ancestor_record_offset":7600,"inlinee":28191,"source_line":69,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3811,"record_offset":16500,"parent_record_offset":16460,"ancestor_record_offset":7600,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3811,"record_offset":16540,"parent_record_offset":16500,"ancestor_record_offset":7600,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3811,"record_offset":16648,"parent_record_offset":16460,"ancestor_record_offset":7600,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3811,"record_offset":16460,"parent_record_offset":16404,"ancestor_record_offset":7600,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3811,"record_offset":16136,"parent_record_offset":16108,"ancestor_record_offset":7600,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3811,"record_offset":16192,"parent_record_offset":16136,"ancestor_record_offset":7600,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3811,"record_offset":16796,"parent_record_offset":16404,"ancestor_record_offset":7600,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3811,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":7600,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3811,"record_offset":16340,"parent_record_offset":16312,"ancestor_record_offset":7600,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16684,"parent_record_offset":16404,"ancestor_record_offset":7600,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16108,"parent_record_offset":16048,"ancestor_record_offset":7600,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16404,"parent_record_offset":16368,"ancestor_record_offset":7600,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":7600,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16080,"parent_record_offset":16048,"ancestor_record_offset":7600,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":7600,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3811,"record_offset":16368,"parent_record_offset":16340,"ancestor_record_offset":7600,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":3811,"record_offset":11084,"parent_record_offset":11056,"ancestor_record_offset":7600,"inlinee":5808,"source_line":2431,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3811,"record_offset":14496,"parent_record_offset":14408,"ancestor_record_offset":7600,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3811,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":7600,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3811,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":7600,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3811,"record_offset":53824,"parent_record_offset":53800,"ancestor_record_offset":7600,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":3811,"record_offset":11148,"parent_record_offset":11112,"ancestor_record_offset":7600,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":3811,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":7600,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3811,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":7600,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":28860,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=41
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":9460,"parent_record_offset":9424,"ancestor_record_offset":7600,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":15068,"parent_record_offset":15032,"ancestor_record_offset":7600,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":15592,"parent_record_offset":15328,"ancestor_record_offset":7600,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3811,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":7600,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":9684,"parent_record_offset":9532,"ancestor_record_offset":7600,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":9948,"parent_record_offset":9684,"ancestor_record_offset":7600,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3811,"record_offset":27836,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3811,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":7600,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3811,"record_offset":10528,"parent_record_offset":10500,"ancestor_record_offset":7600,"inlinee":17336,"source_line":705,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3811,"record_offset":20884,"parent_record_offset":20860,"ancestor_record_offset":7600,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3811,"record_offset":22984,"parent_record_offset":22952,"ancestor_record_offset":7600,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":3811,"record_offset":14728,"parent_record_offset":14696,"ancestor_record_offset":7600,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":3811,"record_offset":14696,"parent_record_offset":14664,"ancestor_record_offset":7600,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":3811,"record_offset":14632,"parent_record_offset":14592,"ancestor_record_offset":7600,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":3811,"record_offset":14592,"parent_record_offset":14560,"ancestor_record_offset":7600,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":3811,"record_offset":14896,"parent_record_offset":14592,"ancestor_record_offset":7600,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":3811,"record_offset":12076,"parent_record_offset":12040,"ancestor_record_offset":7600,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3811,"record_offset":9636,"parent_record_offset":9604,"ancestor_record_offset":7600,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3811,"record_offset":9532,"parent_record_offset":9496,"ancestor_record_offset":7600,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3811,"record_offset":9496,"parent_record_offset":9460,"ancestor_record_offset":7600,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":3811,"record_offset":14664,"parent_record_offset":14632,"ancestor_record_offset":7600,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::AutoClutterRegion::basis_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":20080,"parent_record_offset":19948,"ancestor_record_offset":7600,"inlinee":58555,"source_line":468,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::AutoClutterRegion::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":25868,"parent_record_offset":25832,"ancestor_record_offset":7600,"inlinee":58549,"source_line":516,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::AutoClutterRegion::sec_xz_norm
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3811,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":7600,"inlinee":58556,"source_line":464,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":3811,"record_offset":12008,"parent_record_offset":11668,"ancestor_record_offset":7600,"inlinee":97990,"source_line":152,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":3811,"record_offset":21540,"parent_record_offset":21504,"ancestor_record_offset":7600,"inlinee":121712,"source_line":488,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid::bushes_near_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":3811,"record_offset":49676,"parent_record_offset":44104,"ancestor_record_offset":7600,"inlinee":58531,"source_line":42,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":3811,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":7600,"inlinee":24758,"source_line":147,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3811,"record_offset":38864,"parent_record_offset":38816,"ancestor_record_offset":7600,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3811,"record_offset":38964,"parent_record_offset":38816,"ancestor_record_offset":7600,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3811,"record_offset":38816,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_squared_segment_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32040,"parent_record_offset":27452,"ancestor_record_offset":7600,"inlinee":98021,"source_line":174,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32312,"parent_record_offset":32168,"ancestor_record_offset":7600,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length_squared
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32688,"parent_record_offset":32040,"ancestor_record_offset":7600,"inlinee":98020,"source_line":63,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32488,"parent_record_offset":32140,"ancestor_record_offset":7600,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":33288,"parent_record_offset":33164,"ancestor_record_offset":7600,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":33376,"parent_record_offset":33132,"ancestor_record_offset":7600,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32140,"parent_record_offset":32040,"ancestor_record_offset":7600,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3811,"record_offset":32168,"parent_record_offset":32140,"ancestor_record_offset":7600,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/03811.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3810,"record_offset":2040,"rva":36522096,"va":5405231216}
// 编译实例 RVAs: 0x22d4870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35169780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e80c5ae7fd","size":5,"soft_fail":false,"target_va":5370067488,"va":5405237263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30373761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e87f8830fe","size":5,"soft_fail":false,"target_va":5374864432,"va":5405238188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30150822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e85aef33fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405232305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30151260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8a4ed33fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405232759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27775085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8932f58fe","size":5,"soft_fail":false,"target_va":5377460112,"va":5405235192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27493013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e86b7d5cfe","size":5,"soft_fail":false,"target_va":5377742176,"va":5405235184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27326895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e851065ffe","size":5,"soft_fail":false,"target_va":5377906288,"va":5405233178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23300338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e80e779cfe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405234861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23300440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8a8769cfe","size":5,"soft_fail":false,"target_va":5381934560,"va":5405234995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cd5a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6288311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8490ca0ff","size":5,"soft_fail":false,"target_va":5398944400,"va":5405232706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cd5f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6288555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8550ba0ff","size":5,"soft_fail":false,"target_va":5398945664,"va":5405234214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf6a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6152624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8501ea2ff","size":5,"soft_fail":false,"target_va":5399079520,"va":5405232139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422e21e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e818c90000","size":5,"soft_fail":false,"target_va":5405286880,"va":5405235395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6111055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e84f3f5d00","size":5,"soft_fail":false,"target_va":5411346944,"va":5405235884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6614945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8a1ef6400","size":5,"soft_fail":false,"target_va":5411847152,"va":5405232202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6614124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e86cec6400","size":5,"soft_fail":false,"target_va":5411847168,"va":5405233039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6614703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8afee6400","size":5,"soft_fail":false,"target_va":5411847776,"va":5405233068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6304552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e828336000","size":5,"soft_fail":false,"target_va":5411540992,"va":5405236435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6304500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8f4326000","size":5,"soft_fail":false,"target_va":5411540963,"va":5405236458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6305283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e803366000","size":5,"soft_fail":false,"target_va":5411541584,"va":5405236296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6305463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8b7366000","size":5,"soft_fail":false,"target_va":5411541776,"va":5405236308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6246433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e821505f00","size":5,"soft_fail":false,"target_va":5411482433,"va":5405235995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6305922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e882386000","size":5,"soft_fail":false,"target_va":5411542064,"va":5405236137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6305939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e893386000","size":5,"soft_fail":false,"target_va":5411542296,"va":5405236352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6305791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8ff376000","size":5,"soft_fail":false,"target_va":5411541904,"va":5405236108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e83afbffff","size":5,"soft_fail":false,"target_va":5405230144,"va":5405231361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6311917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8ed4f6000","size":5,"soft_fail":false,"target_va":5411543360,"va":5405231438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27204965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e89be260fe","size":5,"soft_fail":false,"target_va":5378028432,"va":5405233392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25440540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8e4ce7bfe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405233159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25439629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e873d27bfe","size":5,"soft_fail":false,"target_va":5379793584,"va":5405233208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23224827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8059e9dfe","size":5,"soft_fail":false,"target_va":5382009728,"va":5405234550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6256689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e831785f00","size":5,"soft_fail":false,"target_va":5411492368,"va":5405235674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6261962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8ca8c5f00","size":5,"soft_fail":false,"target_va":5411494800,"va":5405232833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23408858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e826cf9afe","size":5,"soft_fail":false,"target_va":5381823600,"va":5405232453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23408798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e862cf9afe","size":5,"soft_fail":false,"target_va":5381824016,"va":5405232809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23846333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8432294fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405231864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23841144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8883694fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405231779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23849282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8be1694fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405235757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23846642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e80e2194fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405231341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6573246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8be4c6400","size":5,"soft_fail":false,"target_va":5411809152,"va":5405235901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20860801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e87fb0c1fe","size":5,"soft_fail":false,"target_va":5384376048,"va":5405236844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6301148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8dc256000","size":5,"soft_fail":false,"target_va":5411537104,"va":5405235951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23318556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e8e42f9cfe","size":5,"soft_fail":false,"target_va":5381914128,"va":5405232679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23313407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e801449cfe","size":5,"soft_fail":false,"target_va":5381919216,"va":5405232618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23351650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405231216,"raw_hex":"e89eae9bfe","size":5,"soft_fail":false,"target_va":5381881936,"va":5405233581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5405231216,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5405232222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":9332,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":7016,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3810,"record_offset":4748,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3810,"record_offset":12740,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3810,"record_offset":4892,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":7744,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":5312,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":9276,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9187,"source_line":672,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":7684,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":5220,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3810,"record_offset":6340,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3810,"record_offset":4808,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3810,"record_offset":4980,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3810,"record_offset":5900,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3810,"record_offset":12952,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3810,"record_offset":12584,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3810,"record_offset":12832,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3810,"record_offset":12416,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3810,"record_offset":12528,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3810,"record_offset":12472,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3810,"record_offset":12136,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":3764,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":2332,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":2788,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3810,"record_offset":7656,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs
//     evidence={"module":3810,"record_offset":13408,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":121703,"source_line":76,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3810,"record_offset":10876,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3810,"record_offset":7628,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3810,"record_offset":10664,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":2040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":2040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":5452,"parent_record_offset":5424,"ancestor_record_offset":2040,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":5484,"parent_record_offset":5452,"ancestor_record_offset":2040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3810,"record_offset":6528,"parent_record_offset":6436,"ancestor_record_offset":2040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3810,"record_offset":5424,"parent_record_offset":5384,"ancestor_record_offset":2040,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":2040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":2040,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":2040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":2040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":2040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":2040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":7280,"parent_record_offset":7248,"ancestor_record_offset":2040,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":7248,"parent_record_offset":7216,"ancestor_record_offset":2040,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":2040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3810,"record_offset":7316,"parent_record_offset":7280,"ancestor_record_offset":2040,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3810,"record_offset":7148,"parent_record_offset":7116,"ancestor_record_offset":2040,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3810,"record_offset":7116,"parent_record_offset":7048,"ancestor_record_offset":2040,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3810,"record_offset":5608,"parent_record_offset":5384,"ancestor_record_offset":2040,"inlinee":12282,"source_line":1709,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3810,"record_offset":5660,"parent_record_offset":5608,"ancestor_record_offset":2040,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":2040,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":7088,"parent_record_offset":7048,"ancestor_record_offset":2040,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":7184,"parent_record_offset":7148,"ancestor_record_offset":2040,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":10968,"parent_record_offset":10908,"ancestor_record_offset":2040,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3810,"record_offset":7048,"parent_record_offset":7016,"ancestor_record_offset":2040,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3810,"record_offset":8196,"parent_record_offset":7864,"ancestor_record_offset":2040,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3810,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":2040,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3810,"record_offset":7864,"parent_record_offset":7820,"ancestor_record_offset":2040,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3810,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":2040,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3810,"record_offset":8668,"parent_record_offset":7820,"ancestor_record_offset":2040,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3810,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":2040,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3810,"record_offset":9120,"parent_record_offset":7820,"ancestor_record_offset":2040,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3810,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":2040,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3810,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":2040,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3810,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":2040,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3810,"record_offset":8768,"parent_record_offset":8744,"ancestor_record_offset":2040,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3810,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":2040,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3810,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":2040,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3810,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":2040,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3810,"record_offset":8408,"parent_record_offset":7820,"ancestor_record_offset":2040,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3810,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":2040,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3810,"record_offset":9020,"parent_record_offset":8996,"ancestor_record_offset":2040,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3810,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":2040,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3810,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":2040,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3810,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":2040,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3810,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":2040,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3810,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":2040,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3810,"record_offset":8256,"parent_record_offset":8228,"ancestor_record_offset":2040,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":2040,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":2040,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":2040,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":2040,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":2040,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4244,"parent_record_offset":4072,"ancestor_record_offset":2040,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3810,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":2040,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3810,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":2040,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3810,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":2040,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3810,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":2040,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3810,"record_offset":5048,"parent_record_offset":5008,"ancestor_record_offset":2040,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3810,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":2040,"inlinee":11098,"source_line":22,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3810,"record_offset":13008,"parent_record_offset":12952,"ancestor_record_offset":2040,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3810,"record_offset":12980,"parent_record_offset":12952,"ancestor_record_offset":2040,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3810,"record_offset":10816,"parent_record_offset":10664,"ancestor_record_offset":2040,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3810,"record_offset":10844,"parent_record_offset":10664,"ancestor_record_offset":2040,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3810,"record_offset":5716,"parent_record_offset":5608,"ancestor_record_offset":2040,"inlinee":11097,"source_line":268,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3810,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":2040,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3810,"record_offset":9508,"parent_record_offset":9480,"ancestor_record_offset":2040,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3810,"record_offset":9480,"parent_record_offset":9444,"ancestor_record_offset":2040,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3810,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":2040,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3810,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":2040,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3810,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":2040,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3810,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":2040,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3810,"record_offset":6264,"parent_record_offset":6236,"ancestor_record_offset":2040,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3810,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":2040,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3810,"record_offset":6496,"parent_record_offset":6464,"ancestor_record_offset":2040,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3810,"record_offset":2488,"parent_record_offset":2456,"ancestor_record_offset":2040,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3810,"record_offset":8140,"parent_record_offset":7864,"ancestor_record_offset":2040,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3810,"record_offset":12244,"parent_record_offset":12136,"ancestor_record_offset":2040,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3810,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":2040,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3810,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":2040,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3810,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":2040,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3810,"record_offset":3916,"parent_record_offset":3872,"ancestor_record_offset":2040,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3810,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":2040,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3810,"record_offset":10704,"parent_record_offset":10664,"ancestor_record_offset":2040,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":2040,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":9536,"parent_record_offset":9508,"ancestor_record_offset":2040,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":2040,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":6172,"parent_record_offset":5900,"ancestor_record_offset":2040,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":8904,"parent_record_offset":8768,"ancestor_record_offset":2040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":2040,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":5980,"parent_record_offset":5900,"ancestor_record_offset":2040,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":6208,"parent_record_offset":5900,"ancestor_record_offset":2040,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3810,"record_offset":10028,"parent_record_offset":9588,"ancestor_record_offset":2040,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3810,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2040,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3810,"record_offset":4704,"parent_record_offset":3872,"ancestor_record_offset":2040,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3810,"record_offset":7556,"parent_record_offset":7148,"ancestor_record_offset":2040,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3810,"record_offset":4308,"parent_record_offset":4020,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3810,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":2040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3810,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":2040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3810,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":2040,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3810,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2040,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3810,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2040,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3810,"record_offset":3384,"parent_record_offset":3356,"ancestor_record_offset":2040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3810,"record_offset":5560,"parent_record_offset":5532,"ancestor_record_offset":2040,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3810,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":2040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3810,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":2040,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3810,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":2040,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3810,"record_offset":7896,"parent_record_offset":7864,"ancestor_record_offset":2040,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3810,"record_offset":6144,"parent_record_offset":5900,"ancestor_record_offset":2040,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3810,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":2040,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3810,"record_offset":11000,"parent_record_offset":10968,"ancestor_record_offset":2040,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3810,"record_offset":7936,"parent_record_offset":7896,"ancestor_record_offset":2040,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3810,"record_offset":11032,"parent_record_offset":11000,"ancestor_record_offset":2040,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3810,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":2040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2040,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":2040,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":2040,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":2040,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":2040,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":3104,"parent_record_offset":2988,"ancestor_record_offset":2040,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3810,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":2040,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3810,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":2040,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3810,"record_offset":2816,"parent_record_offset":2788,"ancestor_record_offset":2040,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3810,"record_offset":2956,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3810,"record_offset":2924,"parent_record_offset":2888,"ancestor_record_offset":2040,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3810,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":2040,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3810,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2040,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3810,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":2040,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3810,"record_offset":3428,"parent_record_offset":2888,"ancestor_record_offset":2040,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":2040,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":2040,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2588,"parent_record_offset":2556,"ancestor_record_offset":2040,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2684,"parent_record_offset":2588,"ancestor_record_offset":2040,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2040,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":2040,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":2040,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3810,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":2040,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3810,"record_offset":2988,"parent_record_offset":2956,"ancestor_record_offset":2040,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3810,"record_offset":3272,"parent_record_offset":2988,"ancestor_record_offset":2040,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3810,"record_offset":3244,"parent_record_offset":2988,"ancestor_record_offset":2040,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3810,"record_offset":3048,"parent_record_offset":2988,"ancestor_record_offset":2040,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":6948,"parent_record_offset":6888,"ancestor_record_offset":2040,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":6056,"parent_record_offset":6020,"ancestor_record_offset":2040,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":5828,"parent_record_offset":5608,"ancestor_record_offset":2040,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":5772,"parent_record_offset":5608,"ancestor_record_offset":2040,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":2040,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":2040,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3810,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":2040,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3810,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":2040,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3810,"record_offset":5384,"parent_record_offset":5352,"ancestor_record_offset":2040,"inlinee":24520,"source_line":117,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3810,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":2040,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3810,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":2040,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3810,"record_offset":8228,"parent_record_offset":7820,"ancestor_record_offset":2040,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3810,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":2040,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3810,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":2040,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3810,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":2040,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3810,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":2040,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3810,"record_offset":12768,"parent_record_offset":12740,"ancestor_record_offset":2040,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3810,"record_offset":12664,"parent_record_offset":12632,"ancestor_record_offset":2040,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3810,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":2040,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3810,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":2040,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3810,"record_offset":11240,"parent_record_offset":11212,"ancestor_record_offset":2040,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3810,"record_offset":11268,"parent_record_offset":11240,"ancestor_record_offset":2040,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3810,"record_offset":9736,"parent_record_offset":9588,"ancestor_record_offset":2040,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3810,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":2040,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3810,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":2040,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3810,"record_offset":6020,"parent_record_offset":5900,"ancestor_record_offset":2040,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3810,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":2040,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3810,"record_offset":6112,"parent_record_offset":6020,"ancestor_record_offset":2040,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3810,"record_offset":6920,"parent_record_offset":6888,"ancestor_record_offset":2040,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":3800,"parent_record_offset":3764,"ancestor_record_offset":2040,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":15140,"parent_record_offset":15108,"ancestor_record_offset":2040,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":15656,"parent_record_offset":15396,"ancestor_record_offset":2040,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3810,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":2040,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":4020,"parent_record_offset":3872,"ancestor_record_offset":2040,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3810,"record_offset":4280,"parent_record_offset":4020,"ancestor_record_offset":2040,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3810,"record_offset":12696,"parent_record_offset":12664,"ancestor_record_offset":2040,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3810,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":2040,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3810,"record_offset":3872,"parent_record_offset":3836,"ancestor_record_offset":2040,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3810,"record_offset":3836,"parent_record_offset":3800,"ancestor_record_offset":2040,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs
//     evidence={"module":3810,"record_offset":5352,"parent_record_offset":5312,"ancestor_record_offset":2040,"inlinee":121699,"source_line":72,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs
//     evidence={"module":3810,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":2040,"inlinee":121700,"source_line":124,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs
//     evidence={"module":3810,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":2040,"inlinee":121701,"source_line":128,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs
//     evidence={"module":3810,"record_offset":9300,"parent_record_offset":9276,"ancestor_record_offset":2040,"inlinee":121702,"source_line":129,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3810,"record_offset":10908,"parent_record_offset":10876,"ancestor_record_offset":2040,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3810,"record_offset":10940,"parent_record_offset":10908,"ancestor_record_offset":2040,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3810,"record_offset":11076,"parent_record_offset":10908,"ancestor_record_offset":2040,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3810,"record_offset":11212,"parent_record_offset":10908,"ancestor_record_offset":2040,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/03810.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":308,"record_offset":1472,"rva":6627408,"va":5375336528}
// 编译实例 RVAs: 0x652050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$T$u20$as$u20$alloc..vec..spec_from_elem..SpecFromElem$GT$::from_elem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8977834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8aafd8800","size":5,"soft_fail":false,"target_va":5384315840,"va":5375338001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6620102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e83afc9aff","size":5,"soft_fail":false,"target_va":5368718192,"va":5375338289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6460489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8b76b9dff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375339716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5736567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e88977a8ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5375337362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=12
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5720497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e84fb6a8ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375336604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140edb900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8951786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8ea978800","size":5,"soft_fail":false,"target_va":5384288512,"va":5375336721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140edec90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8964958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e85ecb8800","size":5,"soft_fail":false,"target_va":5384301712,"va":5375336749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ee0e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8973566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8feec8800","size":5,"soft_fail":false,"target_va":5384310352,"va":5375336781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ee5f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8993609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8493b8900","size":5,"soft_fail":false,"target_va":5384331088,"va":5375337474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9003531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e80b628900","size":5,"soft_fail":false,"target_va":5384341168,"va":5375337632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8935903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8df598800","size":5,"soft_fail":false,"target_va":5384272640,"va":5375336732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9024803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e823b58900","size":5,"soft_fail":false,"target_va":5384363344,"va":5375338536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8984033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8e1158900","size":5,"soft_fail":false,"target_va":5384321136,"va":5375337098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8976572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8bcf88800","size":5,"soft_fail":false,"target_va":5384313472,"va":5375336895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8979183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8ef028900","size":5,"soft_fail":false,"target_va":5384316256,"va":5375337068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8968106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8aad78800","size":5,"soft_fail":false,"target_va":5384304880,"va":5375336769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8977068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8acfa8800","size":5,"soft_fail":false,"target_va":5384314192,"va":5375337119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8942466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e882738800","size":5,"soft_fail":false,"target_va":5384279168,"va":5375336697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36202356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e874672802","size":5,"soft_fail":false,"target_va":5411541904,"va":5375339543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8932465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8714c8800","size":5,"soft_fail":false,"target_va":5384272352,"va":5375339882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7320115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e833b26f00","size":5,"soft_fail":false,"target_va":5382657856,"va":5375337736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7416021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e8d5287100","size":5,"soft_fail":false,"target_va":5382753792,"va":5375337766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36197431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336528,"raw_hex":"e837542802","size":5,"soft_fail":false,"target_va":5411537104,"va":5375339668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t9025504","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375336528,"raw_hex":"e9e0b78900","size":5,"soft_fail":false,"target_va":5384364928,"va":5375339419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":308,"record_offset":6908,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":308,"record_offset":1644,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":308,"record_offset":2844,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_meshes::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":308,"record_offset":6176,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18383,"source_line":28,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::shape_intersections::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shape_intersections.rs
//     evidence={"module":308,"record_offset":4724,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18363,"source_line":8,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":308,"record_offset":5604,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18373,"source_line":66,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":308,"record_offset":5632,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18375,"source_line":79,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_clutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_clutter/mod.rs
//     evidence={"module":308,"record_offset":5688,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18379,"source_line":20,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_clutter::ui_focus_decoclutter::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_clutter/ui_focus_decoclutter.rs
//     evidence={"module":308,"record_offset":6144,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18381,"source_line":53,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":308,"record_offset":4088,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18355,"source_line":139,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":308,"record_offset":5660,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18377,"source_line":75,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":308,"record_offset":5484,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18367,"source_line":114,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/detect_inter_roof_merlons.rs
//     evidence={"module":308,"record_offset":5456,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18365,"source_line":42,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/detect_intra_shape_corners.rs
//     evidence={"module":308,"record_offset":4696,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18361,"source_line":32,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::stitch_bricks::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/stitch_bricks.rs
//     evidence={"module":308,"record_offset":4668,"parent_record_offset":1472,"ancestor_record_offset":1472,"inlinee":18359,"source_line":141,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":308,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1472,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":308,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1472,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":308,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1472,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":308,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1472,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":308,"record_offset":2148,"parent_record_offset":2108,"ancestor_record_offset":1472,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":308,"record_offset":4524,"parent_record_offset":4088,"ancestor_record_offset":1472,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":308,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1472,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":308,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":1472,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":308,"record_offset":1672,"parent_record_offset":1644,"ancestor_record_offset":1472,"inlinee":9172,"source_line":211,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":308,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1472,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":308,"record_offset":2724,"parent_record_offset":1672,"ancestor_record_offset":1472,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":308,"record_offset":2884,"parent_record_offset":2844,"ancestor_record_offset":1472,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":308,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":1472,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":308,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":1472,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":308,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":1472,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":308,"record_offset":2364,"parent_record_offset":1832,"ancestor_record_offset":1472,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":308,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":1472,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":308,"record_offset":1864,"parent_record_offset":1832,"ancestor_record_offset":1472,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":308,"record_offset":2448,"parent_record_offset":2388,"ancestor_record_offset":1472,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":308,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1472,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":308,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1472,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":308,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1472,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":308,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":1472,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":308,"record_offset":1744,"parent_record_offset":1708,"ancestor_record_offset":1472,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":308,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1472,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":308,"record_offset":1708,"parent_record_offset":1672,"ancestor_record_offset":1472,"inlinee":9173,"source_line":2000,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":308,"record_offset":7020,"parent_record_offset":6992,"ancestor_record_offset":1472,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":308,"record_offset":2540,"parent_record_offset":1832,"ancestor_record_offset":1472,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":308,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":1472,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":308,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1472,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":308,"record_offset":1832,"parent_record_offset":1768,"ancestor_record_offset":1472,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":308,"record_offset":2664,"parent_record_offset":1708,"ancestor_record_offset":1472,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":308,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":1472,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":308,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":1472,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":308,"record_offset":4332,"parent_record_offset":4276,"ancestor_record_offset":1472,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":308,"record_offset":2692,"parent_record_offset":1708,"ancestor_record_offset":1472,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":308,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":1472,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":308,"record_offset":5308,"parent_record_offset":5248,"ancestor_record_offset":1472,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":308,"record_offset":2000,"parent_record_offset":1940,"ancestor_record_offset":1472,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":6992,"parent_record_offset":6960,"ancestor_record_offset":1472,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":2020,"parent_record_offset":2000,"ancestor_record_offset":1472,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":7496,"parent_record_offset":6960,"ancestor_record_offset":1472,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":2060,"parent_record_offset":1888,"ancestor_record_offset":1472,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":1472,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":308,"record_offset":3096,"parent_record_offset":3068,"ancestor_record_offset":1472,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":308,"record_offset":4248,"parent_record_offset":4220,"ancestor_record_offset":1472,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":308,"record_offset":4552,"parent_record_offset":4088,"ancestor_record_offset":1472,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":308,"record_offset":5088,"parent_record_offset":4972,"ancestor_record_offset":1472,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":308,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":1472,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":308,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":1472,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":308,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":1472,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":308,"record_offset":1940,"parent_record_offset":1912,"ancestor_record_offset":1472,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":308,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1472,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":308,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":1472,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":308,"record_offset":2288,"parent_record_offset":2108,"ancestor_record_offset":1472,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":308,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1472,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":308,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":1472,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":308,"record_offset":6100,"parent_record_offset":5760,"ancestor_record_offset":1472,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":308,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":1472,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":308,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":1472,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":308,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":1472,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":308,"record_offset":4464,"parent_record_offset":4124,"ancestor_record_offset":1472,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":308,"record_offset":4124,"parent_record_offset":4088,"ancestor_record_offset":1472,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":308,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":1472,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":308,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":1472,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":308,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":1472,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":308,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":1472,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":308,"record_offset":4276,"parent_record_offset":4220,"ancestor_record_offset":1472,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":308,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":1472,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":308,"record_offset":4940,"parent_record_offset":4912,"ancestor_record_offset":1472,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":308,"record_offset":4972,"parent_record_offset":4940,"ancestor_record_offset":1472,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":308,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":1472,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":308,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":1472,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":308,"record_offset":4496,"parent_record_offset":4088,"ancestor_record_offset":1472,"inlinee":18357,"source_line":133,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":308,"record_offset":5516,"parent_record_offset":5484,"ancestor_record_offset":1472,"inlinee":18369,"source_line":18,"archive":"_pdb/B/modules/00308.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":308,"record_offset":5572,"parent_record_offset":5484,"ancestor_record_offset":1472,"inlinee":18371,"source_line":50,"archive":"_pdb/B/modules/00308.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] system_clutter::ClutterPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_clutter::ClutterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,system_clutter::ClutterPlugin>", "constructor": "bevy_app::app::App::add_plugins", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 24; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>", "bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>", "bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::ui_clutter::decoclutter_show_pins::DecoClutterLock>,ref_mut$<country_core::components::Ui2dWsExtent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::ui_clutter::decoclutter_show_pins::DecoClutterLock>,ref_mut$<country_core::components::Ui2dWsExtent> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::ui_clutter::decoclutter_show_pins::DecoClutterLock>,ref_mut$<country_core::components::Ui2dWsExtent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>,bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >", "bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>", "bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowAutoClutterCandidates>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowAutoClutterCandidates>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowAutoClutterCandidates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::AutoClutter>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: alloc::collections::btree::node::Handle::insert_recursing<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global> >
// 信息 ID: 2305e42fc33e49bd077b6bae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0 -> ["alloc::string::String", "enum2$<serde_json::value::Value>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] alloc::collections::btree::node::Handle::insert_recursing -> ["alloc::string::String", "enum2$<serde_json::value::Value>", "alloc::alloc::Global", "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::collections::btree::map::entry::VacantEntry::insert_entry<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global> @ 0x22bfd22 -> 0x1cb09a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1cd12b7 -> 0x1cd1300 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1ce37e7 -> 0x1ce3830 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1ce5677 -> 0x1ce56c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1ce6327 -> 0x1ce6370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22c99e7 -> 0x22c9a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22cc107 -> 0x22cc150 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22cdfd7 -> 0x22ce020 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22ced57 -> 0x22ceda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22d0257 -> 0x22d02a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22d45e7 -> 0x22d4630 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x22d6667 -> 0x22d66b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x26091e7 -> 0x2609230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::StrRead>
// 信息 ID: 7ec086b32ebb5cddd25622bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x5777c -> 0x57fd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_number<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x578c9 -> 0x57fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_number<serde_json::read::StrRead>
// 信息 ID: cd880b0c8b4c9e620d0c8ed4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_number -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::StrRead> | 其余 1 个同地址别名见共享索引 @ 0x57b77 -> 0x57730 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::value::de::impl$0::deserialize::impl$0::visit_map<serde_json::de::MapAccess<serde_json::read::StrRead> >
// 信息 ID: 33db0ff46eb1f439ce3b9d07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::MapAccess -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// [类型关系] serde_json::value::de::impl$0::deserialize::impl$0::visit_map -> ["serde_json::de::MapAccess<serde_json::read::StrRead>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions
// 信息 ID: 730a00b2f1911067e61ea70e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b056645eec6ecdc7eeaef773: void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > > @ 0x1cddb1e -> 0x22d6ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x1cecf77 -> 0x22d6ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::impl$0::build
// 信息 ID: c15a9b35a2b21eb82da4fcbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons
// 信息 ID: a4e5d507615ea90d4287c117
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c6e3c7f43d691b7bc52dd669: void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > > @ 0x1cdc850 -> 0x22d4870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0x1ce838b -> 0x22d4870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
