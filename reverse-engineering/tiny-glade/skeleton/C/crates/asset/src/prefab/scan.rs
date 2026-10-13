// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/prefab/scan.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 171763cdaa076552c4481682a78d0ca76a76d917b93195ee3fc3c3dea61d64a0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1805, 3384
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1579F
// 首条位置: {"module":1805,"inlinee":87967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":1148,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":87969,"source_line":20,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":1864,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":444,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":396,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":880,"parent_record_offset":676,"ancestor_record_offset":396,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1805,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":396,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":1004,"parent_record_offset":952,"ancestor_record_offset":396,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1805,"record_offset":588,"parent_record_offset":556,"ancestor_record_offset":396,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":708,"parent_record_offset":676,"ancestor_record_offset":396,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":796,"parent_record_offset":676,"ancestor_record_offset":396,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":396,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":396,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":1116,"parent_record_offset":444,"ancestor_record_offset":396,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":396,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":396,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":396,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":396,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":952,"parent_record_offset":556,"ancestor_record_offset":396,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":396,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":396,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":396,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":396,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":528,"parent_record_offset":500,"ancestor_record_offset":396,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157A1
// 首条位置: {"module":1805,"inlinee":87969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":1148,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":1148,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":1616,"parent_record_offset":1412,"ancestor_record_offset":1148,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1805,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1148,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":1744,"parent_record_offset":1688,"ancestor_record_offset":1148,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1805,"record_offset":1324,"parent_record_offset":1288,"ancestor_record_offset":1148,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":1444,"parent_record_offset":1412,"ancestor_record_offset":1148,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":1532,"parent_record_offset":1412,"ancestor_record_offset":1148,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":1148,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":1148,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1148,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1148,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":1688,"parent_record_offset":1288,"ancestor_record_offset":1148,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":1148,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":1148,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":1412,"parent_record_offset":1380,"ancestor_record_offset":1148,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":1288,"parent_record_offset":1260,"ancestor_record_offset":1148,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":1148,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157AC
// 首条位置: {"module":1805,"inlinee":87980}
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
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1805,"record_offset":4680,"rva":29297504,"va":5398006624}
// 编译实例 RVAs: 0x1bf0b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13353214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e8023f34ff","size":5,"soft_fail":false,"target_va":5384653728,"va":5398006937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$Q$u20$as$u20$hashbrown..Equivalent$LT$K$GT$$GT$::equivalent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10877689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e8f9faa500","size":5,"soft_fail":false,"target_va":5408884544,"va":5398006850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22328275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e82d4cabfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5398006958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22328686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e8924aabfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5398007385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29284756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e86c2641fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5398006943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140998640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19236567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e82979dafe","size":5,"soft_fail":false,"target_va":5378770496,"va":5398007058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1426504c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10877169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e8f1f8a500","size":5,"soft_fail":false,"target_va":5408883904,"va":5398006730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142651190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10880143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e88f04a600","size":5,"soft_fail":false,"target_va":5408887184,"va":5398007036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18343946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e80ae81701","size":5,"soft_fail":false,"target_va":5416351056,"va":5398007105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18613577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e849051c01","size":5,"soft_fail":false,"target_va":5416620769,"va":5398007187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-22058230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398006624,"raw_hex":"e80a6baffe","size":5,"soft_fail":false,"target_va":5375949040,"va":5398007265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1805,"record_offset":6004,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1805,"record_offset":5812,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":6144,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":6116,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":4860,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":6088,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1805,"record_offset":5784,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":4680,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":4680,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":4680,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":4680,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":4680,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":4680,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":5880,"parent_record_offset":5844,"ancestor_record_offset":4680,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":6776,"parent_record_offset":6720,"ancestor_record_offset":4680,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1805,"record_offset":5844,"parent_record_offset":5812,"ancestor_record_offset":4680,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6248,"parent_record_offset":6216,"ancestor_record_offset":4680,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4680,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":4680,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":4680,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":4680,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":4680,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1805,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":4680,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":7004,"parent_record_offset":6968,"ancestor_record_offset":4680,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":4680,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":6216,"parent_record_offset":6172,"ancestor_record_offset":4680,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":4680,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":6172,"parent_record_offset":6144,"ancestor_record_offset":4680,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1805,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4680,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1805,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":4680,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1805,"record_offset":5612,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1805,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4680,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1805,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":4680,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":4680,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":6364,"parent_record_offset":6172,"ancestor_record_offset":4680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":4680,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1805,"record_offset":7036,"parent_record_offset":7004,"ancestor_record_offset":4680,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":6336,"parent_record_offset":6172,"ancestor_record_offset":4680,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":6448,"parent_record_offset":6412,"ancestor_record_offset":4680,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":4680,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":6504,"parent_record_offset":6412,"ancestor_record_offset":4680,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1805,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":4680,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1805,"record_offset":5312,"parent_record_offset":5280,"ancestor_record_offset":4680,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1805,"record_offset":5252,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1805,"record_offset":5112,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1805,"record_offset":5640,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1805,"record_offset":5196,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1805,"record_offset":5056,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1805,"record_offset":4888,"parent_record_offset":4860,"ancestor_record_offset":4680,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1805,"record_offset":5580,"parent_record_offset":5412,"ancestor_record_offset":4680,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1805,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":4680,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5520,"parent_record_offset":5436,"ancestor_record_offset":4680,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5464,"parent_record_offset":5436,"ancestor_record_offset":4680,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5732,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":4680,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":4680,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":4952,"parent_record_offset":4888,"ancestor_record_offset":4680,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4680,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5084,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":5412,"parent_record_offset":5004,"ancestor_record_offset":4680,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1805,"record_offset":260,"rva":29296480,"va":5398005600}
// 编译实例 RVAs: 0x1bf0760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-13360745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8972134ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5398005652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-22327344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8d04fabfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5398006043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28736137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8778549fe","size":5,"soft_fail":false,"target_va":5369270288,"va":5398006420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140991cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19262550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8aa13dafe","size":5,"soft_fail":false,"target_va":5378743536,"va":5398006081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1990
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16052045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8b3100bff","size":5,"soft_fail":false,"target_va":5381953936,"va":5398005976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca2550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16049657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e8071a0bff","size":5,"soft_fail":false,"target_va":5381956944,"va":5398006596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e807090000","size":5,"soft_fail":false,"target_va":5398008240,"va":5398005924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13279620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398005600,"raw_hex":"e87c5e35ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5398006303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":1980,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1805,"record_offset":2008,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":87971,"source_line":5,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":396,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":87967,"source_line":14,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":2760,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":2732,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":3232,"parent_record_offset":3168,"ancestor_record_offset":260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":3200,"parent_record_offset":3168,"ancestor_record_offset":260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":3168,"parent_record_offset":3140,"ancestor_record_offset":260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":260,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":1148,"parent_record_offset":396,"ancestor_record_offset":260,"inlinee":87969,"source_line":20,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":260,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":880,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1805,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":260,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":1004,"parent_record_offset":952,"ancestor_record_offset":260,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1805,"record_offset":588,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":708,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":1864,"parent_record_offset":396,"ancestor_record_offset":260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":796,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":1116,"parent_record_offset":444,"ancestor_record_offset":260,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":444,"parent_record_offset":396,"ancestor_record_offset":260,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":260,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":952,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":260,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":260,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":260,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":260,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":528,"parent_record_offset":500,"ancestor_record_offset":260,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1805,"record_offset":8660,"rva":29299120,"va":5398008240}
// 编译实例 RVAs: 0x1bf11b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13319554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e87ec234ff","size":5,"soft_fail":false,"target_va":5384689584,"va":5398009133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13319907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e81dc134ff","size":5,"soft_fail":false,"target_va":5384689312,"va":5398009214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13240673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e89ff635ff","size":5,"soft_fail":false,"target_va":5384768304,"va":5398008972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13352199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8f94234ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5398008434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13352606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8624134ff","size":5,"soft_fail":false,"target_va":5384656912,"va":5398009513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13352598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e86a4134ff","size":5,"soft_fail":false,"target_va":5384657312,"va":5398009905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13354609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e88f3934ff","size":5,"soft_fail":false,"target_va":5384653728,"va":5398008332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13246931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e82dde35ff","size":5,"soft_fail":false,"target_va":5384764464,"va":5398011390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-22330586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e82643abfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5398009269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-22329962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e89645abfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5398008661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29287069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8631d41fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5398009256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28678648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e808664afe","size":5,"soft_fail":false,"target_va":5369335008,"va":5398013651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28669790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8a2884afe","size":5,"soft_fail":false,"target_va":5369338848,"va":5398008633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1990
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16054912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e880050bff","size":5,"soft_fail":false,"target_va":5381953936,"va":5398008843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142650e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10875552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8a0f2a500","size":5,"soft_fail":false,"target_va":5408886384,"va":5398010827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142651c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10881139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e87308a600","size":5,"soft_fail":false,"target_va":5408889936,"va":5398008792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142651da0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10880388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e88405a600","size":5,"soft_fail":false,"target_va":5408890272,"va":5398009879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18342027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e88be01701","size":5,"soft_fail":false,"target_va":5416350384,"va":5398008352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19048289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e861a72201","size":5,"soft_fail":false,"target_va":5417057056,"va":5398008762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18608897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e801f31b01","size":5,"soft_fail":false,"target_va":5416620769,"va":5398011867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18609751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e857f61b01","size":5,"soft_fail":false,"target_va":5416620794,"va":5398011038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t-15887319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e829940dff","size":5,"soft_fail":false,"target_va":5382122800,"va":5398010114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13215760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8f05736ff","size":5,"soft_fail":false,"target_va":5384794736,"va":5398010491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18773130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e88a741e01","size":5,"soft_fail":false,"target_va":5416784672,"va":5398011537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18858370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e882c11f01","size":5,"soft_fail":false,"target_va":5416869856,"va":5398011481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8ffeeffff","size":5,"soft_fail":false,"target_va":5398006624,"va":5398010972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::load_prefab_config
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e867f7ffff","size":5,"soft_fail":false,"target_va":5398007584,"va":5398009780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e818f8ffff","size":5,"soft_fail":false,"target_va":5398008240,"va":5398010259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t18609837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8adf61b01","size":5,"soft_fail":false,"target_va":5416621600,"va":5398011758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-13179095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e829e736ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5398009170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::relative_path_buf::RelativePathBuf::push
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10883519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8bf11a600","size":5,"soft_fail":false,"target_va":5408893696,"va":5398010172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13283565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e8134f35ff","size":5,"soft_fail":false,"target_va":5384724912,"va":5398008472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13283553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008240,"raw_hex":"e81f4f35ff","size":5,"soft_fail":false,"target_va":5384725152,"va":5398008700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":8904,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":10272,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::PrefabName::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1805,"record_offset":16860,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":82913,"source_line":26,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::config::impl$1::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1805,"record_offset":13588,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":87978,"source_line":21,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1805,"record_offset":18424,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":28802,"source_line":10,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":19024,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":87980,"source_line":27,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1805,"record_offset":12264,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":12760,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":11092,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":9660,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":10092,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":20928,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1805,"record_offset":18732,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1805,"record_offset":18816,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1805,"record_offset":12948,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePath::join
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1805,"record_offset":11128,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":69925,"source_line":792,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":18536,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1805,"record_offset":11008,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":10892,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":18117,"source_line":1978,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":10028,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1805,"record_offset":9532,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1805,"record_offset":9504,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1805,"record_offset":8876,"parent_record_offset":8660,"ancestor_record_offset":8660,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":8660,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":8660,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8660,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":8660,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":8660,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":8660,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":11620,"parent_record_offset":11472,"ancestor_record_offset":8660,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":9352,"parent_record_offset":9296,"ancestor_record_offset":8660,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9012,"parent_record_offset":8980,"ancestor_record_offset":8660,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":8660,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":8660,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":8660,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":8660,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8660,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":11472,"parent_record_offset":11440,"ancestor_record_offset":8660,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":8660,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":8660,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":11508,"parent_record_offset":11472,"ancestor_record_offset":8660,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1805,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":8660,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1805,"record_offset":11320,"parent_record_offset":11292,"ancestor_record_offset":8660,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1805,"record_offset":11348,"parent_record_offset":11320,"ancestor_record_offset":8660,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1805,"record_offset":11264,"parent_record_offset":11216,"ancestor_record_offset":8660,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1805,"record_offset":11216,"parent_record_offset":11180,"ancestor_record_offset":8660,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17064,"parent_record_offset":17036,"ancestor_record_offset":8660,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17036,"parent_record_offset":17000,"ancestor_record_offset":8660,"inlinee":7216,"source_line":2245,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17000,"parent_record_offset":16972,"ancestor_record_offset":8660,"inlinee":7215,"source_line":2260,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17312,"parent_record_offset":17064,"ancestor_record_offset":8660,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":15236,"parent_record_offset":15200,"ancestor_record_offset":8660,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17280,"parent_record_offset":17248,"ancestor_record_offset":8660,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::allocate_for_slice::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":17100,"parent_record_offset":17064,"ancestor_record_offset":8660,"inlinee":7219,"source_line":2249,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$31::from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":16972,"parent_record_offset":16944,"ancestor_record_offset":8660,"inlinee":7214,"source_line":2352,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":18448,"parent_record_offset":18424,"ancestor_record_offset":8660,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":15200,"parent_record_offset":15176,"ancestor_record_offset":8660,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$70::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":16944,"parent_record_offset":16916,"ancestor_record_offset":8660,"inlinee":7212,"source_line":3903,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$72::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1805,"record_offset":16916,"parent_record_offset":16888,"ancestor_record_offset":8660,"inlinee":7210,"source_line":3941,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":8980,"parent_record_offset":8932,"ancestor_record_offset":8660,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8660,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1805,"record_offset":11384,"parent_record_offset":11348,"ancestor_record_offset":8660,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::scan_asset_tree_walker::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":12792,"parent_record_offset":12760,"ancestor_record_offset":8660,"inlinee":87976,"source_line":116,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1805,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":8660,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1805,"record_offset":12320,"parent_record_offset":12292,"ancestor_record_offset":8660,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1805,"record_offset":16888,"parent_record_offset":16860,"ancestor_record_offset":8660,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1805,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":8660,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":8660,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":12904,"parent_record_offset":12792,"ancestor_record_offset":8660,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1805,"record_offset":11748,"parent_record_offset":11348,"ancestor_record_offset":8660,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":8660,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=160
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":9128,"parent_record_offset":8932,"ancestor_record_offset":8660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":8660,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":17340,"parent_record_offset":17312,"ancestor_record_offset":8660,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1805,"record_offset":18876,"parent_record_offset":18816,"ancestor_record_offset":8660,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1805,"record_offset":15268,"parent_record_offset":15236,"ancestor_record_offset":8660,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1805,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":8660,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":9100,"parent_record_offset":8932,"ancestor_record_offset":8660,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":8660,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":17248,"parent_record_offset":17064,"ancestor_record_offset":8660,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1805,"record_offset":12348,"parent_record_offset":12320,"ancestor_record_offset":8660,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1805,"record_offset":12376,"parent_record_offset":12348,"ancestor_record_offset":8660,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1805,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":8660,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1805,"record_offset":12292,"parent_record_offset":12264,"ancestor_record_offset":8660,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":8660,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":18496,"parent_record_offset":18472,"ancestor_record_offset":8660,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":15296,"parent_record_offset":15236,"ancestor_record_offset":8660,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1805,"record_offset":15328,"parent_record_offset":15200,"ancestor_record_offset":8660,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1805,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":8660,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1805,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":8660,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::RelativePath::to_relative_path_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":1805,"record_offset":11180,"parent_record_offset":11128,"ancestor_record_offset":8660,"inlinee":21980,"source_line":852,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1805,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":8660,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":10920,"parent_record_offset":10892,"ancestor_record_offset":8660,"inlinee":18118,"source_line":1955,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":8660,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileAttr::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1805,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":8660,"inlinee":18120,"source_line":1088,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1805,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":8660,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1805,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":8660,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157A8
// 首条位置: {"module":1805,"inlinee":87976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":12904,"parent_record_offset":12792,"ancestor_record_offset":12792,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1805,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":12792,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":1805,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":12792,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1805,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":12792,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1805,"record_offset":7140,"rva":29298464,"va":5398007584}
// 编译实例 RVAs: 0x1bf0f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-22329068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e81449abfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5398007767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142651830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10881171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e89308a600","size":5,"soft_fail":false,"target_va":5408888880,"va":5398007704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13213135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e8316236ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5398007994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h73397bb27a24205c (.llvm.4873370733750390364)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18860992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e8c0cb1f01","size":5,"soft_fail":false,"target_va":5416869040,"va":5398008043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18861800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e8e8ce1f01","size":5,"soft_fail":false,"target_va":5416869712,"va":5398007907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13384137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e837c633ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5398008212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13256370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398007584,"raw_hex":"e84eb935ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5398007629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1805,"record_offset":8404,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::scan::load_prefab_config::do_load
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":7268,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":87973,"source_line":152,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":7140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":7140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":7140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7644,"parent_record_offset":7588,"ancestor_record_offset":7140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1805,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":7140,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1805,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":7140,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7616,"parent_record_offset":7588,"ancestor_record_offset":7140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1805,"record_offset":8584,"parent_record_offset":8560,"ancestor_record_offset":7140,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1805,"record_offset":8560,"parent_record_offset":8404,"ancestor_record_offset":7140,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::scan::load_prefab_config::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/scan.rs
//     evidence={"module":1805,"record_offset":8448,"parent_record_offset":8404,"ancestor_record_offset":7140,"inlinee":87974,"source_line":157,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":7140,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":7492,"parent_record_offset":7268,"ancestor_record_offset":7140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":7364,"parent_record_offset":7268,"ancestor_record_offset":7140,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":7776,"parent_record_offset":7268,"ancestor_record_offset":7140,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":1805,"record_offset":7396,"parent_record_offset":7268,"ancestor_record_offset":7140,"inlinee":10120,"source_line":119,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/options.rs
//     evidence={"module":1805,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":7140,"inlinee":9932,"source_line":59,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":7336,"parent_record_offset":7268,"ancestor_record_offset":7140,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::do_load
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157A5
// 首条位置: {"module":1805,"inlinee":87973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":7492,"parent_record_offset":7268,"ancestor_record_offset":7268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":7364,"parent_record_offset":7268,"ancestor_record_offset":7268,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1805,"record_offset":7776,"parent_record_offset":7268,"ancestor_record_offset":7268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":1805,"record_offset":7396,"parent_record_offset":7268,"ancestor_record_offset":7268,"inlinee":10120,"source_line":119,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1805,"record_offset":7336,"parent_record_offset":7268,"ancestor_record_offset":7268,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":7268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":7268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":7268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1805,"record_offset":7644,"parent_record_offset":7588,"ancestor_record_offset":7268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7616,"parent_record_offset":7588,"ancestor_record_offset":7268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1805,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":7268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/options.rs
//     evidence={"module":1805,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":7268,"inlinee":9932,"source_line":59,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::scan::load_prefab_config::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157A6
// 首条位置: {"module":1805,"inlinee":87974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1805,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":8448,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1805,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":8448,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":8448,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01805.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::prefab::scan::PrefabDependencies>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::prefab::scan::PrefabDependencies"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::scan::PrefabScanResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
// 信息 ID: a183801bfd694cccf5dadae7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa66d3d -> 0x1bf0b60 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf1c5c -> 0x1bf0b60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency
// 信息 ID: f704585613dd92c148a4f06c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=1805, inline_record=19024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$2::default
// 信息 ID: 6f435fafddaac88edd786c68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=1805, inline_record=396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::impl$3::default
// 信息 ID: b98a308adca319d8e0c48f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=1805, inline_record=1148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config
// 信息 ID: 2c614238d4e9d21dbbc05207
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf17b4 -> 0x1bf0f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf1cc1 -> 0x1bf0f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::closure$0
// 信息 ID: b1c3dfb718c9ae1235fbde8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::prefab::scan::load_prefab_config", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=1805, inline_record=8448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::load_prefab_config::do_load
// 信息 ID: 76b6b39f3a9eb67b4813d5d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::load_prefab_config; module=1805, inline_record=7268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree
// 信息 ID: 9fe5b112630d92eeffea6be8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs_impl @ 0xa6ba71 -> 0x1bf0760 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap @ 0xa6d043 -> 0x1bf0760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker
// 信息 ID: a71fba94dc59ff9d29aa270f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::prefab::scan::scan_asset_tree @ 0x1bf08a4 -> 0x1bf11b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::prefab::scan::scan_asset_tree_walker @ 0x1bf1993 -> 0x1bf11b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::scan::scan_asset_tree_walker::closure$1
// 信息 ID: d64aa5e51e28c4aff6046b36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::prefab::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=1805, inline_record=12792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
