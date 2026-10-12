// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/prefab/scan.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d652ca2bcbda571afdbb22fd5da07f092f21527dc15a4f998789be6c07a36580
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3085, 4092
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F52
// 首条位置: {"module":3085,"inlinee":106322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":3832,"parent_record_offset":3096,"ancestor_record_offset":3096,"inlinee":106324,"source_line":20,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4560,"parent_record_offset":3096,"ancestor_record_offset":3096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3132,"parent_record_offset":3096,"ancestor_record_offset":3096,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":3096,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":3096,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3085,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":3096,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3085,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":3096,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3085,"record_offset":3260,"parent_record_offset":3228,"ancestor_record_offset":3096,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":3096,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":3096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":3448,"parent_record_offset":3340,"ancestor_record_offset":3096,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":3096,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":3096,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3548,"parent_record_offset":3496,"ancestor_record_offset":3096,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3096,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3800,"parent_record_offset":3132,"ancestor_record_offset":3096,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3096,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":3096,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":3408,"parent_record_offset":3388,"ancestor_record_offset":3096,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":3096,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3604,"parent_record_offset":3228,"ancestor_record_offset":3096,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":3096,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":3096,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":3096,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":3096,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":3096,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F54
// 首条位置: {"module":3085,"inlinee":106324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3832,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":4384,"parent_record_offset":4352,"ancestor_record_offset":3832,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":4444,"parent_record_offset":4412,"ancestor_record_offset":3832,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3085,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3832,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3085,"record_offset":4412,"parent_record_offset":4352,"ancestor_record_offset":3832,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3085,"record_offset":3988,"parent_record_offset":3952,"ancestor_record_offset":3832,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":3832,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":4192,"parent_record_offset":4076,"ancestor_record_offset":3832,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4268,"parent_record_offset":4240,"ancestor_record_offset":3832,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3832,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4296,"parent_record_offset":4240,"ancestor_record_offset":3832,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3832,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3832,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":3832,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":4352,"parent_record_offset":3952,"ancestor_record_offset":3832,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3832,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3832,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":4076,"parent_record_offset":4044,"ancestor_record_offset":3832,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3832,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3832,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F5C
// 首条位置: {"module":3085,"inlinee":106332}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3085,"record_offset":604,"rva":33594864,"va":5402303984}
// 编译实例 RVAs: 0x2009df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$Q$u20$as$u20$hashbrown..Equivalent$LT$K$GT$$GT$::equivalent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4293593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8d9834100","size":5,"soft_fail":false,"target_va":5406597808,"va":5402304210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27222836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8cc9c60fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5402304319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27223246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8329b60fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402304745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088d2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24628451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e81d3388fe","size":5,"soft_fail":false,"target_va":5377675968,"va":5402304414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19563566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8d27bd5fe","size":5,"soft_fail":false,"target_va":5382740736,"va":5402304297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142422010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4293047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8b7814100","size":5,"soft_fail":false,"target_va":5406597136,"va":5402304084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424239f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4299362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8629a4100","size":5,"soft_fail":false,"target_va":5406603760,"va":5402304393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9236438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8d6ef8c00","size":5,"soft_fail":false,"target_va":5411540992,"va":5402304549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27949480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8588655fe","size":5,"soft_fail":false,"target_va":5374354992,"va":5402304467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-26981702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402303984,"raw_hex":"e8ba4a64fe","size":5,"soft_fail":false,"target_va":5375322928,"va":5402304625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":1752,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3085,"record_offset":1908,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":2048,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":2020,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":784,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":1992,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3085,"record_offset":1724,"parent_record_offset":604,"ancestor_record_offset":604,"inlinee":13531,"source_line":2045,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":604,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":604,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":604,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":1788,"parent_record_offset":1752,"ancestor_record_offset":604,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":2652,"parent_record_offset":2596,"ancestor_record_offset":604,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":604,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":604,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":604,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":604,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":604,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":604,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3085,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":604,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":2824,"parent_record_offset":2788,"ancestor_record_offset":604,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":604,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":2096,"parent_record_offset":2048,"ancestor_record_offset":604,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":1956,"parent_record_offset":1932,"ancestor_record_offset":604,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3085,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":604,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3085,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":604,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3085,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":604,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3085,"record_offset":1548,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3085,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":604,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3085,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":604,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":2244,"parent_record_offset":2048,"ancestor_record_offset":604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":604,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3085,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":604,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":604,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":2216,"parent_record_offset":2048,"ancestor_record_offset":604,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":2328,"parent_record_offset":2292,"ancestor_record_offset":604,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":604,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":2384,"parent_record_offset":2292,"ancestor_record_offset":604,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3085,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":604,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3085,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":604,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3085,"record_offset":1188,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3085,"record_offset":1048,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3085,"record_offset":1576,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3085,"record_offset":1132,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3085,"record_offset":1020,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3085,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":604,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3085,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":604,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3085,"record_offset":1516,"parent_record_offset":1348,"ancestor_record_offset":604,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3085,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":604,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":1456,"parent_record_offset":1372,"ancestor_record_offset":604,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":604,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":1668,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":604,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":604,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":892,"parent_record_offset":832,"ancestor_record_offset":604,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":604,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":992,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":1348,"parent_record_offset":940,"ancestor_record_offset":604,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3085,"record_offset":2960,"rva":33595824,"va":5402304944}
// 编译实例 RVAs: 0x200a1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=23 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27465065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e897ea5cfe","size":5,"soft_fail":false,"target_va":5374840768,"va":5402305828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27223936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e8809860fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402305435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b742a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21586124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e8349fb6fe","size":5,"soft_fail":false,"target_va":5380719264,"va":5402305383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b74970
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21584511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e881a5b6fe","size":5,"soft_fail":false,"target_va":5380721008,"va":5402305514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b75590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21581382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e8bab1b6fe","size":5,"soft_fail":false,"target_va":5380724112,"va":5402305489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e8b1020000","size":5,"soft_fail":false,"target_va":5402306032,"va":5402305338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..prefab..scan..PrefabScanResult$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4299902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e87e9c4100","size":5,"soft_fail":false,"target_va":5406605744,"va":5402305837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-19647142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e85a35d4fe","size":5,"soft_fail":false,"target_va":5382657856,"va":5402304993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-19551236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402304944,"raw_hex":"e8fcabd5fe","size":5,"soft_fail":false,"target_va":5382753792,"va":5402305023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":4676,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3085,"record_offset":4704,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":106326,"source_line":6,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":3096,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":106322,"source_line":14,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":5464,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":5436,"parent_record_offset":2960,"ancestor_record_offset":2960,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":2960,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":5936,"parent_record_offset":5872,"ancestor_record_offset":2960,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":2960,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":2960,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":2960,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":2960,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":3832,"parent_record_offset":3096,"ancestor_record_offset":2960,"inlinee":106324,"source_line":20,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2960,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":3692,"parent_record_offset":3660,"ancestor_record_offset":2960,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3085,"record_offset":3496,"parent_record_offset":3472,"ancestor_record_offset":2960,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3085,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":2960,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3085,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":2960,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3085,"record_offset":3260,"parent_record_offset":3228,"ancestor_record_offset":2960,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":2960,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":4560,"parent_record_offset":3096,"ancestor_record_offset":2960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":3448,"parent_record_offset":3340,"ancestor_record_offset":2960,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":2960,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2960,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":3548,"parent_record_offset":3496,"ancestor_record_offset":2960,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":2960,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3800,"parent_record_offset":3132,"ancestor_record_offset":2960,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":3132,"parent_record_offset":3096,"ancestor_record_offset":2960,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":2960,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":3408,"parent_record_offset":3388,"ancestor_record_offset":2960,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":2960,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3604,"parent_record_offset":3228,"ancestor_record_offset":2960,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2960,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2960,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":2960,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":2960,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2960,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3085,"record_offset":7588,"rva":33596912,"va":5402306032}
// 编译实例 RVAs: 0x200a5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19598032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e830f5d4fe","size":5,"soft_fail":false,"target_va":5382708864,"va":5402306891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19540809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8b7d4d5fe","size":5,"soft_fail":false,"target_va":5382768048,"va":5402308852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27414488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e828b05dfe","size":5,"soft_fail":false,"target_va":5374896624,"va":5402311107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27226599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8198e60fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5402308082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27224798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8229560fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402306297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b742a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21589384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e87892b6fe","size":5,"soft_fail":false,"target_va":5380719264,"va":5402308643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19602355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e84de4d4fe","size":5,"soft_fail":false,"target_va":5382704752,"va":5402307102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=9
//     target_aliases=5 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19478430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e862c8d6fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5402306153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142423000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4292616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e808804100","size":5,"soft_fail":false,"target_va":5406601216,"va":5402308595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142424d90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4302190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e86ea54100","size":5,"soft_fail":false,"target_va":5406608784,"va":5402306589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142425b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4305464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e838b24100","size":5,"soft_fail":false,"target_va":5406612224,"va":5402306755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9540437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e855939100","size":5,"soft_fail":false,"target_va":5411847168,"va":5402306726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9231581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8dddc8c00","size":5,"soft_fail":false,"target_va":5411540992,"va":5402309406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-27949997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8538455fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5402306520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t-19623812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e87c90d4fe","size":5,"soft_fail":false,"target_va":5382683584,"va":5402307391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19459158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8aa13d7fe","size":5,"soft_fail":false,"target_va":5382848896,"va":5402308049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9476862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8fe9a9000","size":5,"soft_fail":false,"target_va":5411786032,"va":5402309165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9508382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e81e169100","size":5,"soft_fail":false,"target_va":5411817328,"va":5402308941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e866edffff","size":5,"soft_fail":false,"target_va":5402303984,"va":5402308741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::load_prefab_config
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8ea120000","size":5,"soft_fail":false,"target_va":5402311248,"va":5402306401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e82ffbffff","size":5,"soft_fail":false,"target_va":5402306032,"va":5402307260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t9232419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e823e08c00","size":5,"soft_fail":false,"target_va":5411541776,"va":5402309352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9233424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e810e48c00","size":5,"soft_fail":false,"target_va":5411542880,"va":5402309451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4305395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8f3b14100","size":5,"soft_fail":false,"target_va":5406612592,"va":5402307192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-19597529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e827f7d4fe","size":5,"soft_fail":false,"target_va":5382709552,"va":5402307076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19597580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8f4f6d4fe","size":5,"soft_fail":false,"target_va":5382709424,"va":5402306999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-19598141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8c3f4d4fe","size":5,"soft_fail":false,"target_va":5382709008,"va":5402307144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19562059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8b581d5fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5402306134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19563636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e88c7bd5fe","size":5,"soft_fail":false,"target_va":5382743968,"va":5402307599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19564098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e8be79d5fe","size":5,"soft_fail":false,"target_va":5382743808,"va":5402307901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19536119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e809e7d5fe","size":5,"soft_fail":false,"target_va":5382770560,"va":5402306674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19529591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402306032,"raw_hex":"e88900d6fe","size":5,"soft_fail":false,"target_va":5382776592,"va":5402306178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5402306032,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5402306268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":9392,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":10028,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::PrefabName::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":3085,"record_offset":13528,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":74009,"source_line":26,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$1::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3085,"record_offset":9900,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":106328,"source_line":21,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":3085,"record_offset":15936,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":26267,"source_line":10,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":16644,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":106332,"source_line":26,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3085,"record_offset":12252,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3085,"record_offset":18748,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":12840,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":10856,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":8908,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":9360,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":17840,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3085,"record_offset":16244,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3085,"record_offset":16328,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3085,"record_offset":12972,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":16048,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":10824,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":10740,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":46522,"source_line":202,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":12128,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":10628,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":9928,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3085,"record_offset":7980,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3085,"record_offset":7812,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3085,"record_offset":7840,"parent_record_offset":7588,"ancestor_record_offset":7588,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":7588,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":13996,"parent_record_offset":13968,"ancestor_record_offset":7588,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":7588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":7588,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":8500,"parent_record_offset":8412,"ancestor_record_offset":7588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3085,"record_offset":8412,"parent_record_offset":8384,"ancestor_record_offset":7588,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9468,"parent_record_offset":9436,"ancestor_record_offset":7588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":7588,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":7588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":7588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":7588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":7588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13880,"parent_record_offset":13748,"ancestor_record_offset":7588,"inlinee":7968,"source_line":1945,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13748,"parent_record_offset":13708,"ancestor_record_offset":7588,"inlinee":17146,"source_line":2035,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13708,"parent_record_offset":13672,"ancestor_record_offset":7588,"inlinee":17145,"source_line":2049,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":14136,"parent_record_offset":13880,"ancestor_record_offset":7588,"inlinee":7971,"source_line":1977,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":11992,"parent_record_offset":11956,"ancestor_record_offset":7588,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":14104,"parent_record_offset":14068,"ancestor_record_offset":7588,"inlinee":7975,"source_line":1952,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$24::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13920,"parent_record_offset":13880,"ancestor_record_offset":7588,"inlinee":17148,"source_line":2039,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$27::from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13672,"parent_record_offset":13636,"ancestor_record_offset":7588,"inlinee":17144,"source_line":2141,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":7588,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":7588,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$63::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13636,"parent_record_offset":13600,"ancestor_record_offset":7588,"inlinee":17142,"source_line":3630,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$65::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3085,"record_offset":13600,"parent_record_offset":13564,"ancestor_record_offset":7588,"inlinee":17140,"source_line":3668,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3085,"record_offset":9436,"parent_record_offset":9392,"ancestor_record_offset":7588,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::scan_asset_tree_walker::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":12872,"parent_record_offset":12840,"ancestor_record_offset":7588,"inlinee":106330,"source_line":113,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3085,"record_offset":13788,"parent_record_offset":13748,"ancestor_record_offset":7588,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3085,"record_offset":8444,"parent_record_offset":8412,"ancestor_record_offset":7588,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3085,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":7588,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3085,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":7588,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3085,"record_offset":13564,"parent_record_offset":13528,"ancestor_record_offset":7588,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3085,"record_offset":14288,"parent_record_offset":13708,"ancestor_record_offset":7588,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3085,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7588,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":12928,"parent_record_offset":12872,"ancestor_record_offset":7588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=149
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":8072,"parent_record_offset":7980,"ancestor_record_offset":7588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":7588,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7588,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7588,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":7588,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":14164,"parent_record_offset":14136,"ancestor_record_offset":7588,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3085,"record_offset":12024,"parent_record_offset":11992,"ancestor_record_offset":7588,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3085,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":7588,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":14024,"parent_record_offset":13996,"ancestor_record_offset":7588,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":9556,"parent_record_offset":9392,"ancestor_record_offset":7588,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":7588,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":13844,"parent_record_offset":13748,"ancestor_record_offset":7588,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":14068,"parent_record_offset":13880,"ancestor_record_offset":7588,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3085,"record_offset":12336,"parent_record_offset":12308,"ancestor_record_offset":7588,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3085,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":7588,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3085,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":7588,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3085,"record_offset":12280,"parent_record_offset":12252,"ancestor_record_offset":7588,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":7588,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":7588,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":12052,"parent_record_offset":11992,"ancestor_record_offset":7588,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3085,"record_offset":12084,"parent_record_offset":11956,"ancestor_record_offset":7588,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3085,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":7588,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":12900,"parent_record_offset":12872,"ancestor_record_offset":7588,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":7588,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":7588,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":7588,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":7588,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":8012,"parent_record_offset":7980,"ancestor_record_offset":7588,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3085,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7588,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3085,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":7588,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3085,"record_offset":7864,"parent_record_offset":7840,"ancestor_record_offset":7588,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3085,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":7588,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3085,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":7588,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F5A
// 首条位置: {"module":3085,"inlinee":106330}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":12928,"parent_record_offset":12872,"ancestor_record_offset":12872,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3085,"record_offset":12900,"parent_record_offset":12872,"ancestor_record_offset":12872,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3085,"record_offset":22824,"rva":33602128,"va":5402311248}
// 编译实例 RVAs: 0x200ba50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27229931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e8158160fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402311430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424261f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4302627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e823a74100","size":5,"soft_fail":false,"target_va":5406614000,"va":5402311368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19471461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e89be3d6fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5402311664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h99f34d5a19ceb942 (.llvm.4711583917080959974)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9504923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e89b089100","size":5,"soft_fail":false,"target_va":5411816640,"va":5402311712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9505922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e8820c9100","size":5,"soft_fail":false,"target_va":5411817472,"va":5402311545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19656854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e86a0fd4fe","size":5,"soft_fail":false,"target_va":5382655024,"va":5402311873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19605826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402311248,"raw_hex":"e8bed6d4fe","size":5,"soft_fail":false,"target_va":5382705472,"va":5402311293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3085,"record_offset":23920,"parent_record_offset":22824,"ancestor_record_offset":22824,"inlinee":12467,"source_line":58,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::load_prefab_config::do_load
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":22952,"parent_record_offset":22824,"ancestor_record_offset":22824,"inlinee":106334,"source_line":149,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":22824,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":23328,"parent_record_offset":23272,"ancestor_record_offset":22824,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3085,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":22824,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3085,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":22824,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":22824,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23272,"parent_record_offset":23248,"ancestor_record_offset":22824,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23248,"parent_record_offset":23224,"ancestor_record_offset":22824,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":3085,"record_offset":24100,"parent_record_offset":24076,"ancestor_record_offset":22824,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3085,"record_offset":24076,"parent_record_offset":23920,"ancestor_record_offset":22824,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::load_prefab_config::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":3085,"record_offset":23964,"parent_record_offset":23920,"ancestor_record_offset":22824,"inlinee":106335,"source_line":154,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":22824,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":23176,"parent_record_offset":22952,"ancestor_record_offset":22824,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":23048,"parent_record_offset":22952,"ancestor_record_offset":22824,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":23404,"parent_record_offset":22952,"ancestor_record_offset":22824,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":3085,"record_offset":23080,"parent_record_offset":22952,"ancestor_record_offset":22824,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3085,"record_offset":23104,"parent_record_offset":23080,"ancestor_record_offset":22824,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":23020,"parent_record_offset":22952,"ancestor_record_offset":22824,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::do_load
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F5E
// 首条位置: {"module":3085,"inlinee":106334}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":23176,"parent_record_offset":22952,"ancestor_record_offset":22952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":23048,"parent_record_offset":22952,"ancestor_record_offset":22952,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3085,"record_offset":23404,"parent_record_offset":22952,"ancestor_record_offset":22952,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":3085,"record_offset":23080,"parent_record_offset":22952,"ancestor_record_offset":22952,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3085,"record_offset":23020,"parent_record_offset":22952,"ancestor_record_offset":22952,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":22952,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3085,"record_offset":23328,"parent_record_offset":23272,"ancestor_record_offset":22952,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":22952,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23272,"parent_record_offset":23248,"ancestor_record_offset":22952,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3085,"record_offset":23248,"parent_record_offset":23224,"ancestor_record_offset":22952,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":23200,"parent_record_offset":23176,"ancestor_record_offset":22952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3085,"record_offset":23104,"parent_record_offset":23080,"ancestor_record_offset":22952,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F5F
// 首条位置: {"module":3085,"inlinee":106335}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3085,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":23964,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3085,"record_offset":24036,"parent_record_offset":24012,"ancestor_record_offset":23964,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":24012,"parent_record_offset":23988,"ancestor_record_offset":23964,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03085.jsonl.gz"}
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
// 完整结构及原始类型记录: scan.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::scan::PrefabDependencies
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::scan::PrefabScanResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::prefab::scan::PrefabScanResult>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::prefab::scan::PrefabScanResult"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: scan.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: scan.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
// 信息 ID: 58d2246435399e4efcb775f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0x12a29d5 -> 0x2009df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200b085 -> 0x2009df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 信息 ID: 6a930c0f16e18b9c2832eed3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=3085, inline_record=16644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$2::default
// 信息 ID: 80db6ebac3504ff53ada981f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=3085, inline_record=3096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$3::default
// 信息 ID: cd9bc4b911f534337acd1755
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=3085, inline_record=3832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config
// 信息 ID: 25046a2243019c2802f8800f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200acab -> 0x200ba50 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200a761 -> 0x200ba50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::closure$0
// 信息 ID: 99674a4a87d1bcf1f7e33885
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::prefab::scan::load_prefab_config", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=3085, inline_record=23964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::do_load
// 信息 ID: a25ad3beeaeac21b3fdbeea7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=3085, inline_record=22952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree
// 信息 ID: 42bde7c6aaf026621a4c299f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0x12a2032 -> 0x200a1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker
// 信息 ID: f924b1ec441c1fd93d291231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree @ 0x200a33a -> 0x200a5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x200aabc -> 0x200a5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 信息 ID: e5373eb0f46240a6b5e4d8d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::prefab::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=3085, inline_record=12872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
