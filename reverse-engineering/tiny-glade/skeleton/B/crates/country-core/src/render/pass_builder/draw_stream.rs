// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/pass_builder/draw_stream.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e151f1daa278bec757c171dcd47381ecaa87c2397c8b892e4d30829ca8e4d282
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_stream.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 33, 107, 149, 205, 475, 482, 485, 487, 489, 495, 497, 500, 501, 503, 518, 521, 529, 530, 531, 532, 544, 549, 589, 595, 599, 1585, 1610, 1613, 1623, 1624, 1655, 1657
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=59
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6581
// 首条位置: {"module":475,"inlinee":25985}
// 编译实例 RVAs: 0x867850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3665635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377521744,"raw_hex":"e8e3ee3700","size":5,"soft_fail":false,"target_va":5381187408,"va":5377521768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5136076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377521744,"raw_hex":"e8cc5e4e00","size":5,"soft_fail":false,"target_va":5382657856,"va":5377521775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5231982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377521744,"raw_hex":"e86ed54f00","size":5,"soft_fail":false,"target_va":5382753792,"va":5377521805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=48
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":21212,"parent_record_offset":21164,"ancestor_record_offset":21164,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":21164,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21516,"parent_record_offset":21492,"ancestor_record_offset":21164,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21492,"parent_record_offset":21436,"ancestor_record_offset":21164,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":21164,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":10880,"parent_record_offset":10824,"ancestor_record_offset":10180,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":10480,"parent_record_offset":10448,"ancestor_record_offset":10180,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":10180,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":21408,"parent_record_offset":21376,"ancestor_record_offset":21164,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":10668,"parent_record_offset":10560,"ancestor_record_offset":10180,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":10180,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":10180,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":21564,"parent_record_offset":21540,"ancestor_record_offset":21164,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":21244,"parent_record_offset":21212,"ancestor_record_offset":21164,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21308,"parent_record_offset":21276,"ancestor_record_offset":21164,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":10628,"parent_record_offset":10608,"ancestor_record_offset":10180,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":10504,"parent_record_offset":10480,"ancestor_record_offset":10180,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21436,"parent_record_offset":21376,"ancestor_record_offset":21164,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21276,"parent_record_offset":21244,"ancestor_record_offset":21164,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":10180,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":10180,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":21376,"parent_record_offset":21340,"ancestor_record_offset":21164,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":21340,"parent_record_offset":21308,"ancestor_record_offset":21164,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::variant
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68B4
// 首条位置: {"module":503,"inlinee":26804}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::stencil_ref
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68B5
// 首条位置: {"module":531,"inlinee":26805}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$14::default
// 记录形式: inline_instance, inlinee_declaration; instances=60
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6583
// 首条位置: {"module":475,"inlinee":25987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":21244,"parent_record_offset":21212,"ancestor_record_offset":21212,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":21212,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21516,"parent_record_offset":21492,"ancestor_record_offset":21212,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21492,"parent_record_offset":21436,"ancestor_record_offset":21212,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":21212,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":10880,"parent_record_offset":10824,"ancestor_record_offset":10328,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":10480,"parent_record_offset":10448,"ancestor_record_offset":10328,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":10328,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":21408,"parent_record_offset":21376,"ancestor_record_offset":21212,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":10668,"parent_record_offset":10560,"ancestor_record_offset":10328,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":10328,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":10328,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":21564,"parent_record_offset":21540,"ancestor_record_offset":21212,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21308,"parent_record_offset":21276,"ancestor_record_offset":21212,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":10628,"parent_record_offset":10608,"ancestor_record_offset":10328,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":10504,"parent_record_offset":10480,"ancestor_record_offset":10328,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21436,"parent_record_offset":21376,"ancestor_record_offset":21212,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21276,"parent_record_offset":21244,"ancestor_record_offset":21212,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":10328,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":10328,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":21376,"parent_record_offset":21340,"ancestor_record_offset":21212,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":21340,"parent_record_offset":21308,"ancestor_record_offset":21212,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ECC
// 首条位置: {"module":107,"inlinee":11980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68B8
// 首条位置: {"module":482,"record_offset":11032,"rva":8812864,"va":5377521984}
// 编译实例 RVAs: 0x867940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14145baf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12534171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377521984,"raw_hex":"e89b41bf00","size":5,"soft_fail":false,"target_va":5390056176,"va":5377522000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$2::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EF8
// 首条位置: {"module":503,"inlinee":28408}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$3::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E05
// 首条位置: {"module":500,"inlinee":28165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":500,"record_offset":20684,"parent_record_offset":20656,"ancestor_record_offset":20656,"inlinee":28166,"source_line":320,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$4::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73DB
// 首条位置: {"module":518,"inlinee":29659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":518,"record_offset":34180,"parent_record_offset":34152,"ancestor_record_offset":34152,"inlinee":28166,"source_line":320,"archive":"_pdb/B/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":24988,"rva":9367504,"va":5378076624}
// 编译实例 RVAs: 0x8eefd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33465220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378076624,"raw_hex":"e884a3fe01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378076679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":500,"record_offset":25152,"parent_record_offset":24988,"ancestor_record_offset":24988,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$22::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x685A
// 首条位置: {"module":482,"inlinee":26714}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11904,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":11960,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":11860,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":12488,"parent_record_offset":12456,"ancestor_record_offset":11860,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":12548,"parent_record_offset":12516,"ancestor_record_offset":11860,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":482,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":11860,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":12516,"parent_record_offset":12456,"ancestor_record_offset":11860,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":12092,"parent_record_offset":12056,"ancestor_record_offset":11860,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":12208,"parent_record_offset":12180,"ancestor_record_offset":11860,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":12296,"parent_record_offset":12180,"ancestor_record_offset":11860,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12372,"parent_record_offset":12344,"ancestor_record_offset":11860,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":11860,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12400,"parent_record_offset":12344,"ancestor_record_offset":11860,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":12656,"parent_record_offset":11960,"ancestor_record_offset":11860,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":11860,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":12256,"parent_record_offset":12232,"ancestor_record_offset":11860,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":12120,"parent_record_offset":12092,"ancestor_record_offset":11860,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":12456,"parent_record_offset":12056,"ancestor_record_offset":11860,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":11860,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":11860,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":12180,"parent_record_offset":12148,"ancestor_record_offset":11860,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":11860,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11860,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>
// 记录形式: compiled_procedure; instances=1
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":1038696,"rva":9006464,"va":5377715584}
// 编译实例 RVAs: 0x896d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33826233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377715584,"raw_hex":"e8b9250402","size":5,"soft_fail":false,"target_va":5411541904,"va":5377715666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33822932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377715584,"raw_hex":"e8d4180402","size":5,"soft_fail":false,"target_va":5411538608,"va":5377715671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$9::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":485,"record_offset":1038876,"parent_record_offset":1038696,"ancestor_record_offset":1038696,"inlinee":26742,"source_line":260,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":485,"record_offset":1039048,"parent_record_offset":1038876,"ancestor_record_offset":1038696,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1038984,"parent_record_offset":1038956,"ancestor_record_offset":1038696,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1039004,"parent_record_offset":1038984,"ancestor_record_offset":1038696,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":485,"record_offset":1038956,"parent_record_offset":1038932,"ancestor_record_offset":1038696,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":485,"record_offset":1038932,"parent_record_offset":1038908,"ancestor_record_offset":1038696,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":485,"record_offset":1038908,"parent_record_offset":1038876,"ancestor_record_offset":1038696,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$9::drop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6876
// 首条位置: {"module":482,"inlinee":26742}
// 编译实例 RVAs: 0x896d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33826233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377715584,"raw_hex":"e8b9250402","size":5,"soft_fail":false,"target_va":5411541904,"va":5377715666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33822932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377715584,"raw_hex":"e8d4180402","size":5,"soft_fail":false,"target_va":5411538608,"va":5377715671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":482,"record_offset":17820,"parent_record_offset":17624,"ancestor_record_offset":17624,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":482,"record_offset":17664,"parent_record_offset":17624,"ancestor_record_offset":17624,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":17752,"parent_record_offset":17720,"ancestor_record_offset":17624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":17624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":482,"record_offset":17720,"parent_record_offset":17692,"ancestor_record_offset":17624,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":482,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":17624,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x685B
// 首条位置: {"module":482,"inlinee":26715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13716,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":482,"record_offset":13848,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14284,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14312,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":16488,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":482,"record_offset":14548,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":482,"record_offset":14576,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":26732,"source_line":19,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14416,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14492,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":8268,"source_line":3023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":15280,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13636,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":13636,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":13636,"inlinee":26725,"source_line":1068,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13808,"parent_record_offset":13788,"ancestor_record_offset":13636,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":13636,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":13636,"inlinee":26738,"source_line":1031,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13900,"parent_record_offset":13872,"ancestor_record_offset":13636,"inlinee":26724,"source_line":1046,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":13636,"inlinee":26740,"source_line":1082,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13636,"inlinee":26722,"source_line":1131,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":13636,"inlinee":26734,"source_line":1141,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13636,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16684,"parent_record_offset":16656,"ancestor_record_offset":13636,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14632,"parent_record_offset":14608,"ancestor_record_offset":13636,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14212,"parent_record_offset":13924,"ancestor_record_offset":13636,"inlinee":26727,"source_line":1115,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13636,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":13636,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":13636,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":13636,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":13636,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15712,"parent_record_offset":15656,"ancestor_record_offset":13636,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":13636,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":15748,"parent_record_offset":15616,"ancestor_record_offset":13636,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":482,"record_offset":14872,"parent_record_offset":14828,"ancestor_record_offset":13636,"inlinee":5077,"source_line":40,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":482,"record_offset":13964,"parent_record_offset":13924,"ancestor_record_offset":13636,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":16176,"parent_record_offset":16140,"ancestor_record_offset":13636,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":13636,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":13636,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14968,"parent_record_offset":14872,"ancestor_record_offset":13636,"inlinee":5078,"source_line":1918,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14376,"parent_record_offset":14340,"ancestor_record_offset":13636,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14060,"parent_record_offset":13964,"ancestor_record_offset":13636,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":15244,"parent_record_offset":14576,"ancestor_record_offset":13636,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":15140,"parent_record_offset":14828,"ancestor_record_offset":13636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":13636,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":13636,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":14156,"parent_record_offset":13924,"ancestor_record_offset":13636,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":14028,"parent_record_offset":13992,"ancestor_record_offset":13636,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13636,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":13992,"parent_record_offset":13964,"ancestor_record_offset":13636,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":13636,"inlinee":26729,"source_line":272,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":14340,"parent_record_offset":14312,"ancestor_record_offset":13636,"inlinee":26730,"source_line":279,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":15996,"parent_record_offset":15812,"ancestor_record_offset":13636,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":13636,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15520,"parent_record_offset":15432,"ancestor_record_offset":13636,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15884,"parent_record_offset":15812,"ancestor_record_offset":13636,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15656,"parent_record_offset":15616,"ancestor_record_offset":13636,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":482,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":13636,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":482,"record_offset":16300,"parent_record_offset":15348,"ancestor_record_offset":13636,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16352,"parent_record_offset":16328,"ancestor_record_offset":13636,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":13636,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16328,"parent_record_offset":15348,"ancestor_record_offset":13636,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16084,"parent_record_offset":15432,"ancestor_record_offset":13636,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15348,"parent_record_offset":15312,"ancestor_record_offset":13636,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16216,"parent_record_offset":15348,"ancestor_record_offset":13636,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15432,"parent_record_offset":15392,"ancestor_record_offset":13636,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15616,"parent_record_offset":15432,"ancestor_record_offset":13636,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15812,"parent_record_offset":15432,"ancestor_record_offset":13636,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15848,"parent_record_offset":15812,"ancestor_record_offset":13636,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15392,"parent_record_offset":15348,"ancestor_record_offset":13636,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15492,"parent_record_offset":15432,"ancestor_record_offset":13636,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16116,"parent_record_offset":15392,"ancestor_record_offset":13636,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":482,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":13636,"inlinee":8270,"source_line":350,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":482,"record_offset":15312,"parent_record_offset":15280,"ancestor_record_offset":13636,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$127::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":14992,"parent_record_offset":14968,"ancestor_record_offset":13636,"inlinee":26736,"source_line":681,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14448,"parent_record_offset":14416,"ancestor_record_offset":13636,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6869
// 首条位置: {"module":482,"inlinee":26729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x686A
// 首条位置: {"module":482,"inlinee":26730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14376,"parent_record_offset":14340,"ancestor_record_offset":14340,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::new
// 记录形式: inline_instance, inlinee_declaration; instances=59
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2053
// 首条位置: {"module":33,"inlinee":8275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":33,"record_offset":18796,"parent_record_offset":18764,"ancestor_record_offset":18764,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":53056,"parent_record_offset":53032,"ancestor_record_offset":52952,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":33,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18764,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":19756,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":93208,"parent_record_offset":93180,"ancestor_record_offset":93120,"inlinee":28409,"source_line":157,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":19812,"parent_record_offset":19788,"ancestor_record_offset":19756,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":33,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":18764,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":93180,"parent_record_offset":93152,"ancestor_record_offset":93120,"inlinee":28448,"source_line":185,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":475,"record_offset":53080,"parent_record_offset":53056,"ancestor_record_offset":52952,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::new_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":482,"record_offset":11188,"rva":8812912,"va":5377522032}
// 编译实例 RVAs: 0x867970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7464431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8111a8eff","size":5,"soft_fail":false,"target_va":5370059888,"va":5377524314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161b50
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7366191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8d1998fff","size":5,"soft_fail":false,"target_va":5370157904,"va":5377524090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t58657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e821e50000","size":5,"soft_fail":false,"target_va":5377582864,"va":5377524202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t87937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e881570100","size":5,"soft_fail":false,"target_va":5377611920,"va":5377523978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dca90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1527771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8db4f1700","size":5,"soft_fail":false,"target_va":5379050128,"va":5377522352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3592550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e866d13600","size":5,"soft_fail":false,"target_va":5381114688,"va":5377522133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3599702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e856ed3600","size":5,"soft_fail":false,"target_va":5381122816,"va":5377523109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3645922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8e2a13700","size":5,"soft_fail":false,"target_va":5381170352,"va":5377524425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157b150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13711087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8ef36d100","size":5,"soft_fail":false,"target_va":5391233360,"va":5377522268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141620a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14388348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e87c8cdb00","size":5,"soft_fail":false,"target_va":5391911440,"va":5377523087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34323458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e802bc0b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377523705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::pass_resources::PassResources::new_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t660964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8e4150a00","size":5,"soft_fail":false,"target_va":5378183952,"va":5377522983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_layer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3595743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8dfdd3600","size":5,"soft_fail":false,"target_va":5381118304,"va":5377522556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::layer_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3595836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e83cde3600","size":5,"soft_fail":false,"target_va":5381118368,"va":5377522527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3595746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e8e2dd3600","size":5,"soft_fail":false,"target_va":5381118240,"va":5377522489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5134704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e870594e00","size":5,"soft_fail":false,"target_va":5382657856,"va":5377523147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5230610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377522032,"raw_hex":"e812d04f00","size":5,"soft_fail":false,"target_va":5382753792,"va":5377523177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":482,"record_offset":11580,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":482,"record_offset":11524,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":13392,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":11440,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":11860,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":26714,"source_line":208,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":482,"record_offset":11804,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$64::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":482,"record_offset":11748,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":15625,"source_line":370,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":482,"record_offset":11636,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":26711,"source_line":251,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":482,"record_offset":11664,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":26712,"source_line":187,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster_read
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":482,"record_offset":11608,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":26709,"source_line":227,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":482,"record_offset":11384,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":11188,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11904,"parent_record_offset":11860,"ancestor_record_offset":11188,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":12488,"parent_record_offset":12456,"ancestor_record_offset":11188,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":12548,"parent_record_offset":12516,"ancestor_record_offset":11188,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":482,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":11188,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":482,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":11188,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":482,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":11188,"inlinee":15627,"source_line":154,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":482,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":11188,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":12516,"parent_record_offset":12456,"ancestor_record_offset":11188,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":12092,"parent_record_offset":12056,"ancestor_record_offset":11188,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":12208,"parent_record_offset":12180,"ancestor_record_offset":11188,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":11188,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":11188,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":12296,"parent_record_offset":12180,"ancestor_record_offset":11188,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12372,"parent_record_offset":12344,"ancestor_record_offset":11188,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":11188,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":12400,"parent_record_offset":12344,"ancestor_record_offset":11188,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":482,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":11188,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":482,"record_offset":11828,"parent_record_offset":11804,"ancestor_record_offset":11188,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":482,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":11188,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":12656,"parent_record_offset":11960,"ancestor_record_offset":11188,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":11960,"parent_record_offset":11860,"ancestor_record_offset":11188,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":11188,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":12256,"parent_record_offset":12232,"ancestor_record_offset":11188,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":482,"record_offset":12120,"parent_record_offset":12092,"ancestor_record_offset":11188,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":12456,"parent_record_offset":12056,"ancestor_record_offset":11188,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":11188,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":11188,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":482,"record_offset":12180,"parent_record_offset":12148,"ancestor_record_offset":11188,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":11188,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11188,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 364
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x836B
// 首条位置: {"module":595,"inlinee":33643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::handle_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":595,"record_offset":156316,"parent_record_offset":156264,"ancestor_record_offset":156264,"inlinee":28446,"source_line":81,"archive":"_pdb/B/modules/00595.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":595,"record_offset":156400,"parent_record_offset":156264,"ancestor_record_offset":156264,"inlinee":26711,"source_line":251,"archive":"_pdb/B/modules/00595.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":595,"record_offset":156344,"parent_record_offset":156264,"ancestor_record_offset":156264,"inlinee":26712,"source_line":187,"archive":"_pdb/B/modules/00595.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":595,"record_offset":156368,"parent_record_offset":156344,"ancestor_record_offset":156264,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00595.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":482,"record_offset":13480,"rva":8815472,"va":5377524592}
// 编译实例 RVAs: 0x868370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7466936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e848108eff","size":5,"soft_fail":false,"target_va":5370059888,"va":5377526819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7465172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e82c178eff","size":5,"soft_fail":false,"target_va":5370061664,"va":5377526831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161b50
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7369036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8b48e8fff","size":5,"soft_fail":false,"target_va":5370157904,"va":5377526935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2444926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e882b1daff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377526425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t84992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8004c0100","size":5,"soft_fail":false,"target_va":5377611920,"va":5377526923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140892190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t169228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e80c950200","size":5,"soft_fail":false,"target_va":5377696144,"va":5377526911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896d80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t188587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8abe00200","size":5,"soft_fail":false,"target_va":5377715584,"va":5377526992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dcd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1524394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8aa421700","size":5,"soft_fail":false,"target_va":5379050768,"va":5377526369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6b00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3597402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e85ae43600","size":5,"soft_fail":false,"target_va":5381122816,"va":5377525409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3643400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e808983700","size":5,"soft_fail":false,"target_va":5381170352,"va":5377526947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfc0d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3750070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8b6383900","size":5,"soft_fail":false,"target_va":5381275856,"va":5377525781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141524830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13353398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8b6c1cb00","size":5,"soft_fail":false,"target_va":5390878768,"va":5377525365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34321720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e838b50b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377525443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arrayvec::arrayvec::extend_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34151376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8d01b0902","size":5,"soft_fail":false,"target_va":5411676304,"va":5377524923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34015303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e847080702","size":5,"soft_fail":false,"target_va":5411541904,"va":5377526596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3662319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e8efe13700","size":5,"soft_fail":false,"target_va":5381187488,"va":5377525164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3662651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e83be33700","size":5,"soft_fail":false,"target_va":5381187456,"va":5377524800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3662688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e860e33700","size":5,"soft_fail":false,"target_va":5381187520,"va":5377524827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34011946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377524592,"raw_hex":"e82afb0602","size":5,"soft_fail":false,"target_va":5411538608,"va":5377526657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":16748,"parent_record_offset":13480,"ancestor_record_offset":13480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":13636,"parent_record_offset":13480,"ancestor_record_offset":13480,"inlinee":26715,"source_line":268,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":482,"record_offset":17084,"parent_record_offset":17052,"ancestor_record_offset":13480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":482,"record_offset":17052,"parent_record_offset":16900,"ancestor_record_offset":13480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13480,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":13480,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":13480,"inlinee":26725,"source_line":1068,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13808,"parent_record_offset":13788,"ancestor_record_offset":13480,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16584,"parent_record_offset":16560,"ancestor_record_offset":13480,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":13480,"inlinee":26738,"source_line":1031,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13900,"parent_record_offset":13872,"ancestor_record_offset":13480,"inlinee":26724,"source_line":1046,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":13480,"inlinee":26740,"source_line":1082,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13480,"inlinee":26722,"source_line":1131,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":13480,"inlinee":26734,"source_line":1141,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13480,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":16684,"parent_record_offset":16656,"ancestor_record_offset":13480,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":13716,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":482,"record_offset":14212,"parent_record_offset":13924,"ancestor_record_offset":13480,"inlinee":26727,"source_line":1115,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13480,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":13480,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":482,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":13480,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15908,"parent_record_offset":15884,"ancestor_record_offset":13480,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":13480,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":15712,"parent_record_offset":15656,"ancestor_record_offset":13480,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":482,"record_offset":17820,"parent_record_offset":17624,"ancestor_record_offset":13480,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":13480,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":482,"record_offset":15748,"parent_record_offset":15616,"ancestor_record_offset":13480,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":482,"record_offset":14872,"parent_record_offset":14828,"ancestor_record_offset":13480,"inlinee":5077,"source_line":40,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":482,"record_offset":13964,"parent_record_offset":13924,"ancestor_record_offset":13480,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":482,"record_offset":13848,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":13480,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":17412,"parent_record_offset":17384,"ancestor_record_offset":13480,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":17440,"parent_record_offset":17412,"ancestor_record_offset":13480,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":16176,"parent_record_offset":16140,"ancestor_record_offset":13480,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":16044,"parent_record_offset":16020,"ancestor_record_offset":13480,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":482,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":13480,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14284,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14968,"parent_record_offset":14872,"ancestor_record_offset":13480,"inlinee":5078,"source_line":1918,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14376,"parent_record_offset":14340,"ancestor_record_offset":13480,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":14060,"parent_record_offset":13964,"ancestor_record_offset":13480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":482,"record_offset":15244,"parent_record_offset":14576,"ancestor_record_offset":13480,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":15140,"parent_record_offset":14828,"ancestor_record_offset":13480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":13480,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":13480,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":14156,"parent_record_offset":13924,"ancestor_record_offset":13480,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":14028,"parent_record_offset":13992,"ancestor_record_offset":13480,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13480,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":13992,"parent_record_offset":13964,"ancestor_record_offset":13480,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":17752,"parent_record_offset":17720,"ancestor_record_offset":13480,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":13480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":13480,"inlinee":26729,"source_line":272,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":14340,"parent_record_offset":14312,"ancestor_record_offset":13480,"inlinee":26730,"source_line":279,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$9::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":17624,"parent_record_offset":17592,"ancestor_record_offset":13480,"inlinee":26742,"source_line":260,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":15996,"parent_record_offset":15812,"ancestor_record_offset":13480,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":482,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":13480,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15520,"parent_record_offset":15432,"ancestor_record_offset":13480,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15884,"parent_record_offset":15812,"ancestor_record_offset":13480,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":482,"record_offset":15656,"parent_record_offset":15616,"ancestor_record_offset":13480,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":482,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":13480,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":482,"record_offset":16300,"parent_record_offset":15348,"ancestor_record_offset":13480,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16352,"parent_record_offset":16328,"ancestor_record_offset":13480,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":13480,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16328,"parent_record_offset":15348,"ancestor_record_offset":13480,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16084,"parent_record_offset":15432,"ancestor_record_offset":13480,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15348,"parent_record_offset":15312,"ancestor_record_offset":13480,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16932,"parent_record_offset":16900,"ancestor_record_offset":13480,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16216,"parent_record_offset":15348,"ancestor_record_offset":13480,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":17356,"parent_record_offset":17324,"ancestor_record_offset":13480,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":13480,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15432,"parent_record_offset":15392,"ancestor_record_offset":13480,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15616,"parent_record_offset":15432,"ancestor_record_offset":13480,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15812,"parent_record_offset":15432,"ancestor_record_offset":13480,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16900,"parent_record_offset":16844,"ancestor_record_offset":13480,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15848,"parent_record_offset":15812,"ancestor_record_offset":13480,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16872,"parent_record_offset":16844,"ancestor_record_offset":13480,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15392,"parent_record_offset":15348,"ancestor_record_offset":13480,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":15492,"parent_record_offset":15432,"ancestor_record_offset":13480,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16116,"parent_record_offset":15392,"ancestor_record_offset":13480,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16964,"parent_record_offset":16932,"ancestor_record_offset":13480,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":482,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":13480,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":482,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":13480,"inlinee":8270,"source_line":350,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":482,"record_offset":15312,"parent_record_offset":15280,"ancestor_record_offset":13480,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":482,"record_offset":14548,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":482,"record_offset":14576,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":26732,"source_line":19,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$127::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":14992,"parent_record_offset":14968,"ancestor_record_offset":13480,"inlinee":26736,"source_line":681,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14416,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14492,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":8268,"source_line":3023,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":15280,"parent_record_offset":13636,"ancestor_record_offset":13480,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":14448,"parent_record_offset":14416,"ancestor_record_offset":13480,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":482,"record_offset":17720,"parent_record_offset":17692,"ancestor_record_offset":13480,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":482,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":13480,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":482,"record_offset":17664,"parent_record_offset":17624,"ancestor_record_offset":13480,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 464
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1256C
// 首条位置: {"module":1655,"inlinee":75116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":25800,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":26116,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":32072,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":26172,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":25460,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":26716,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":26480,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1655,"record_offset":28544,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1655,"record_offset":25716,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1655,"record_offset":28480,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":35296,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":31444,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":26360,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":29484,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1655,"record_offset":25592,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1655,"record_offset":31388,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1655,"record_offset":25632,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1655,"record_offset":35236,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":1655,"record_offset":26332,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":26926,"source_line":116,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1655,"record_offset":26772,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":28857,"source_line":30,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1655,"record_offset":26512,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1655,"record_offset":26396,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::mark_unused
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":25976,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":28406,"source_line":447,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":26688,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":25828,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":74584,"source_line":397,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1655,"record_offset":31884,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75120,"source_line":638,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1655,"record_offset":31816,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75119,"source_line":623,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":31940,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75121,"source_line":239,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":35456,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75123,"source_line":267,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":35928,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75125,"source_line":366,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":36720,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75130,"source_line":397,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":36288,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75127,"source_line":297,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1655,"record_offset":36468,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":75129,"source_line":328,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":32332,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":27984,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29596,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$61::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29956,"parent_record_offset":24880,"ancestor_record_offset":24880,"inlinee":11891,"source_line":2551,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":37264,"parent_record_offset":37232,"ancestor_record_offset":24880,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":37232,"parent_record_offset":37140,"ancestor_record_offset":24880,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":32176,"parent_record_offset":32152,"ancestor_record_offset":24880,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":37172,"parent_record_offset":37140,"ancestor_record_offset":24880,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":37140,"parent_record_offset":37112,"ancestor_record_offset":24880,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":32224,"parent_record_offset":32200,"ancestor_record_offset":24880,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":32200,"parent_record_offset":32176,"ancestor_record_offset":24880,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1655,"record_offset":37112,"parent_record_offset":37084,"ancestor_record_offset":24880,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":32152,"parent_record_offset":32128,"ancestor_record_offset":24880,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":32128,"parent_record_offset":32096,"ancestor_record_offset":24880,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":25740,"parent_record_offset":25716,"ancestor_record_offset":24880,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":38572,"parent_record_offset":38548,"ancestor_record_offset":24880,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":32096,"parent_record_offset":32072,"ancestor_record_offset":24880,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":37760,"parent_record_offset":37720,"ancestor_record_offset":24880,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":37000,"parent_record_offset":36972,"ancestor_record_offset":24880,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":37056,"parent_record_offset":37028,"ancestor_record_offset":24880,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":24880,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":25484,"parent_record_offset":25460,"ancestor_record_offset":24880,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":39908,"parent_record_offset":39884,"ancestor_record_offset":24880,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":25552,"parent_record_offset":25532,"ancestor_record_offset":24880,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":39884,"parent_record_offset":39860,"ancestor_record_offset":24880,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":25532,"parent_record_offset":25508,"ancestor_record_offset":24880,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1655,"record_offset":39984,"parent_record_offset":39956,"ancestor_record_offset":24880,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1655,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":24880,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1655,"record_offset":39932,"parent_record_offset":39908,"ancestor_record_offset":24880,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1655,"record_offset":39956,"parent_record_offset":39932,"ancestor_record_offset":24880,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":32040,"parent_record_offset":31940,"ancestor_record_offset":24880,"inlinee":75122,"source_line":1908,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":35552,"parent_record_offset":35456,"ancestor_record_offset":24880,"inlinee":75124,"source_line":1707,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36076,"parent_record_offset":35928,"ancestor_record_offset":24880,"inlinee":75126,"source_line":1728,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36940,"parent_record_offset":36720,"ancestor_record_offset":24880,"inlinee":75132,"source_line":588,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36436,"parent_record_offset":36288,"ancestor_record_offset":24880,"inlinee":75128,"source_line":1927,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1655,"record_offset":36688,"parent_record_offset":36468,"ancestor_record_offset":24880,"inlinee":75131,"source_line":565,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1655,"record_offset":28856,"parent_record_offset":28828,"ancestor_record_offset":24880,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1655,"record_offset":25884,"parent_record_offset":25860,"ancestor_record_offset":24880,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1655,"record_offset":25860,"parent_record_offset":25828,"ancestor_record_offset":24880,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":27580,"parent_record_offset":27556,"ancestor_record_offset":24880,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":28252,"parent_record_offset":28224,"ancestor_record_offset":24880,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":24880,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":27172,"parent_record_offset":27148,"ancestor_record_offset":24880,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1655,"record_offset":33512,"parent_record_offset":33488,"ancestor_record_offset":24880,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1655,"record_offset":33792,"parent_record_offset":33764,"ancestor_record_offset":24880,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1655,"record_offset":33692,"parent_record_offset":33668,"ancestor_record_offset":24880,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1655,"record_offset":33668,"parent_record_offset":33512,"ancestor_record_offset":24880,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33596,"parent_record_offset":33568,"ancestor_record_offset":24880,"inlinee":9025,"source_line":185,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33624,"parent_record_offset":33596,"ancestor_record_offset":24880,"inlinee":9026,"source_line":200,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33568,"parent_record_offset":33544,"ancestor_record_offset":24880,"inlinee":9023,"source_line":178,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33864,"parent_record_offset":33840,"ancestor_record_offset":24880,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33740,"parent_record_offset":33716,"ancestor_record_offset":24880,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33888,"parent_record_offset":33864,"ancestor_record_offset":24880,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33764,"parent_record_offset":33740,"ancestor_record_offset":24880,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":33936,"parent_record_offset":33888,"ancestor_record_offset":24880,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1655,"record_offset":34280,"parent_record_offset":33888,"ancestor_record_offset":24880,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":27112,"parent_record_offset":27088,"ancestor_record_offset":24880,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":27676,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1655,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":24880,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1655,"record_offset":28508,"parent_record_offset":28480,"ancestor_record_offset":24880,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1655,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":24880,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1655,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":24880,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1655,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":24880,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1655,"record_offset":39224,"parent_record_offset":39188,"ancestor_record_offset":24880,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1655,"record_offset":25908,"parent_record_offset":25884,"ancestor_record_offset":24880,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1655,"record_offset":37200,"parent_record_offset":37172,"ancestor_record_offset":24880,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1655,"record_offset":34024,"parent_record_offset":33936,"ancestor_record_offset":24880,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1655,"record_offset":33996,"parent_record_offset":33936,"ancestor_record_offset":24880,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":27320,"parent_record_offset":27292,"ancestor_record_offset":24880,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":27292,"parent_record_offset":27264,"ancestor_record_offset":24880,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":36040,"parent_record_offset":35928,"ancestor_record_offset":24880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1655,"record_offset":26740,"parent_record_offset":26716,"ancestor_record_offset":24880,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1655,"record_offset":26568,"parent_record_offset":26536,"ancestor_record_offset":24880,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1655,"record_offset":26596,"parent_record_offset":26536,"ancestor_record_offset":24880,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1655,"record_offset":28196,"parent_record_offset":28160,"ancestor_record_offset":24880,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1655,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":24880,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1655,"record_offset":37904,"parent_record_offset":37880,"ancestor_record_offset":24880,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":29072,"parent_record_offset":28636,"ancestor_record_offset":24880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":25680,"parent_record_offset":25656,"ancestor_record_offset":24880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":27440,"parent_record_offset":27416,"ancestor_record_offset":24880,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":24880,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":26244,"parent_record_offset":26172,"ancestor_record_offset":24880,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":26272,"parent_record_offset":26172,"ancestor_record_offset":24880,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":24880,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":26212,"parent_record_offset":26172,"ancestor_record_offset":24880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":26296,"parent_record_offset":26272,"ancestor_record_offset":24880,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":25932,"parent_record_offset":25908,"ancestor_record_offset":24880,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1655,"record_offset":31844,"parent_record_offset":31816,"ancestor_record_offset":24880,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":24880,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1655,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":24880,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1655,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":24880,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1655,"record_offset":25656,"parent_record_offset":25632,"ancestor_record_offset":24880,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1655,"record_offset":33116,"parent_record_offset":33088,"ancestor_record_offset":24880,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":31984,"parent_record_offset":31940,"ancestor_record_offset":24880,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":26420,"parent_record_offset":26396,"ancestor_record_offset":24880,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":24880,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":26444,"parent_record_offset":26420,"ancestor_record_offset":24880,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":24880,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1655,"record_offset":27604,"parent_record_offset":27580,"ancestor_record_offset":24880,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":31472,"parent_record_offset":31444,"ancestor_record_offset":24880,"inlinee":75117,"source_line":601,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1655,"record_offset":31700,"parent_record_offset":31672,"ancestor_record_offset":24880,"inlinee":75118,"source_line":626,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1655,"record_offset":27232,"parent_record_offset":27204,"ancestor_record_offset":24880,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1655,"record_offset":27264,"parent_record_offset":27232,"ancestor_record_offset":24880,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1655,"record_offset":30312,"parent_record_offset":30124,"ancestor_record_offset":24880,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1655,"record_offset":27204,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":27064,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":28224,"parent_record_offset":28160,"ancestor_record_offset":24880,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":27704,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":28280,"parent_record_offset":28224,"ancestor_record_offset":24880,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":30404,"parent_record_offset":30076,"ancestor_record_offset":24880,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1655,"record_offset":27148,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1655,"record_offset":27036,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1655,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":24880,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":26820,"parent_record_offset":26796,"ancestor_record_offset":24880,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":26844,"parent_record_offset":26820,"ancestor_record_offset":24880,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":31520,"parent_record_offset":31496,"ancestor_record_offset":24880,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":28012,"parent_record_offset":27984,"ancestor_record_offset":24880,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":27532,"parent_record_offset":27364,"ancestor_record_offset":24880,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":30012,"parent_record_offset":29984,"ancestor_record_offset":24880,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":26876,"parent_record_offset":26844,"ancestor_record_offset":24880,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27472,"parent_record_offset":27388,"ancestor_record_offset":24880,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27416,"parent_record_offset":27388,"ancestor_record_offset":24880,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":30504,"parent_record_offset":30076,"ancestor_record_offset":24880,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27796,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":28160,"parent_record_offset":28068,"ancestor_record_offset":24880,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":30076,"parent_record_offset":30040,"ancestor_record_offset":24880,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":24880,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":26928,"parent_record_offset":26904,"ancestor_record_offset":24880,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":26904,"parent_record_offset":26844,"ancestor_record_offset":24880,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":28040,"parent_record_offset":28012,"ancestor_record_offset":24880,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":28132,"parent_record_offset":28068,"ancestor_record_offset":24880,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":28104,"parent_record_offset":28068,"ancestor_record_offset":24880,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":26952,"parent_record_offset":26928,"ancestor_record_offset":24880,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":28068,"parent_record_offset":28040,"ancestor_record_offset":24880,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27008,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":24880,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":30040,"parent_record_offset":30012,"ancestor_record_offset":24880,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":27364,"parent_record_offset":26952,"ancestor_record_offset":24880,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1655,"record_offset":26624,"parent_record_offset":26536,"ancestor_record_offset":24880,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1655,"record_offset":26536,"parent_record_offset":26512,"ancestor_record_offset":24880,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1655,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":24880,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":39188,"parent_record_offset":39160,"ancestor_record_offset":24880,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":26796,"parent_record_offset":26772,"ancestor_record_offset":24880,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":24880,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":24880,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29016,"parent_record_offset":28636,"ancestor_record_offset":24880,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29984,"parent_record_offset":29956,"ancestor_record_offset":24880,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$87::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":28636,"parent_record_offset":28600,"ancestor_record_offset":24880,"inlinee":11275,"source_line":3230,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$88::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":29044,"parent_record_offset":28636,"ancestor_record_offset":24880,"inlinee":11277,"source_line":3246,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":28712,"parent_record_offset":28684,"ancestor_record_offset":24880,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":28828,"parent_record_offset":28772,"ancestor_record_offset":24880,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":33544,"parent_record_offset":33512,"ancestor_record_offset":24880,"inlinee":9021,"source_line":85,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":33840,"parent_record_offset":33512,"ancestor_record_offset":24880,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":33716,"parent_record_offset":33692,"ancestor_record_offset":24880,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1655,"record_offset":28684,"parent_record_offset":28636,"ancestor_record_offset":24880,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1655,"record_offset":28772,"parent_record_offset":28740,"ancestor_record_offset":24880,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1655,"record_offset":28740,"parent_record_offset":28712,"ancestor_record_offset":24880,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A9A
// 首条位置: {"module":1585,"inlinee":72346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 548
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1206E
// 首条位置: {"module":1610,"inlinee":73838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1610,"record_offset":11084,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1610,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":10944,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1610,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":10944,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":11008,"parent_record_offset":10976,"ancestor_record_offset":10944,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1610,"record_offset":11040,"parent_record_offset":11008,"ancestor_record_offset":10944,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 553
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83A3
// 首条位置: {"module":599,"inlinee":33699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":599,"record_offset":191264,"parent_record_offset":191240,"ancestor_record_offset":191240,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":599,"record_offset":191312,"parent_record_offset":191288,"ancestor_record_offset":191240,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":599,"record_offset":191288,"parent_record_offset":191264,"ancestor_record_offset":191240,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00599.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 585
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121B7
// 首条位置: {"module":1624,"inlinee":74167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":33172,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33228,"parent_record_offset":33200,"ancestor_record_offset":33172,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33256,"parent_record_offset":33228,"ancestor_record_offset":33172,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1256D
// 首条位置: {"module":1655,"inlinee":75117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":31472,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":31520,"parent_record_offset":31496,"ancestor_record_offset":31472,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1256E
// 首条位置: {"module":1655,"inlinee":75118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":31724,"parent_record_offset":31700,"ancestor_record_offset":31700,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1655,"record_offset":31748,"parent_record_offset":31724,"ancestor_record_offset":31700,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01655.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawKind
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::render::pass_builder::draw_stream::DrawKind>", "constructor": "enum2$", "arguments": ["country_core::render::pass_builder::draw_stream::DrawKind"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::render::pass_builder::draw_stream::DrawKind> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::render::pass_builder::draw_stream::DrawKind>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawPacket
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPacket>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawPassRenderCheck
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPassRenderCheck"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawStream>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawStream"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::InstancedDraw
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::render::pass_builder::draw_stream::InstancedDraw", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::RasterPass
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::RasterPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::TextureBindingKey
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "constructor": "tuple$", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 122; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>
// 信息 ID: 46bdb14a0161151c048b8a35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::ptr::drop_in_place -> ["country_core::render::pass_builder::draw_stream::DrawPassRenderCheck"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass> @ 0x6033b0 -> 0x896d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::new
// 信息 ID: e4634efaf6332bd9980fc210
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 47 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdef2d -> 0x867850 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=25444; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=28448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=30588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=32724; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=38352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=40492; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::stencil_ref
// 信息 ID: 35760a828764cf3efdc2043b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=76788; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=77344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=531, inline_record=77456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::variant
// 信息 ID: 37a4d7df2df2a7b2f8524e0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::new; module=503, inline_record=18620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
// 信息 ID: 254c1f5ace9b2aa21c7a4c62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=13636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::new
// 信息 ID: c9011f2b0848d0bdb39c8b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 42 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=18764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=19756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=24052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=27056; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=36944; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::new_impl
// 信息 ID: 5fbb9e6f29fbbf6a2a60438e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 42 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xde9ba -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x833ef8 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834266 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x8345ca -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834996 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834cea -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x835039 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x835d78 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x8360dc -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836440 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836784 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836ac4 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x837496 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x8377e7 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x837cd2 -> 0x867970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x838082 -> 0x867970 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::render
// 信息 ID: 36e49cb6843408e902734a15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 42 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdf287 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x8341b1 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834511 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834884 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834c3f -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834f9f -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x8352dc -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836031 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836391 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x8366f1 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836a31 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836d5c -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x83772b -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x837a89 -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x837f4e -> 0x868370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x8382fe -> 0x868370 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view
// 信息 ID: 573e175b69e094f8cb753958
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x372d04 -> 0x867940 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=20744; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=503, inline_record=46272; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=518, inline_record=34240; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
// 信息 ID: faa6d94c19d071ec2ef715b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$10::get_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=14108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
// 信息 ID: 43e5b069df87846c3f521a54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$10::get_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=14340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure$0
// 信息 ID: dc70c4dc8a15aa44e1b9e6f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::new_impl", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; module=595, inline_record=156264; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1360; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1560; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1623, inline_record=1760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0
// 信息 ID: 03e90b335d9cdcd83e23e09e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=24880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$0
// 信息 ID: a4560b70b188b0b5a54d456f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >; module=1585, inline_record=191368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$1
// 信息 ID: 4deb47c36a3a6128a0747041
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- hashbrown::map::impl$82::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; module=1610, inline_record=10944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$2
// 信息 ID: 28e030d934d9ebef4a51a816
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::map::impl$82::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; module=599, inline_record=191240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
// 信息 ID: 64e20b8439b8bcbe1417b353
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; module=1657, inline_record=33172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
// 信息 ID: 85b83c50351eb75b9734d85a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=31472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
// 信息 ID: 5322d3259096226b19ef3c75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1655, inline_record=31700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$14::default
// 信息 ID: 2eac806a0f946fd2de8ffa56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=25492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=28496; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=30636; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=32764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=38400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=40540; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$17::eq
// 信息 ID: 2febdb40d9341a8c8caded71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=23336; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=21452; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=503, inline_record=46984; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=518, inline_record=34948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$22::default
// 信息 ID: f20ada84b66470e4321a8b74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=482, inline_record=11860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$2::to_texture_binding_key
// 信息 ID: 6fc696b37f76394ebd7b17b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=503, inline_record=46216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$3::to_texture_binding_key
// 信息 ID: 05c98c7db08796c197bf8b0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=500, inline_record=20656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$4::to_texture_binding_key
// 信息 ID: 6e550fc34a171ab95a761ff5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=518, inline_record=34152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key
// 信息 ID: 783ed38a745fdbdda1817ad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x372ce2 -> 0x8eefd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x8fe179 -> 0x8eefd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$9::drop
// 信息 ID: 6024804a16c04ec498cc675e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass> @ 0x6033b0 -> 0x896d80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=482, inline_record=17624; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>; module=485, inline_record=930832; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>; module=485, inline_record=1038876; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
