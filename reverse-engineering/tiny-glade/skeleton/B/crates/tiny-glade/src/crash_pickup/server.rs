// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/crash_pickup/server.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d6235cc7b9e3e23c09f02208663fec5b024a1b02b6b0ee531c1dadf6bff9e43f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 20, 101
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c","version":"0.8.2"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use sentry_core; // namespace candidate
//   package identities: [{"name":"sentry-core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::Handler::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D5C
// 首条位置: {"module":101,"inlinee":11612}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::Handler::on_client_message
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D59
// 首条位置: {"module":101,"inlinee":11609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":64636,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":101,"record_offset":64608,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":64876,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":65008,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":64564,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":64664,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":64820,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":101,"record_offset":65784,"parent_record_offset":64344,"ancestor_record_offset":64344,"inlinee":10241,"source_line":151,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":65476,"parent_record_offset":65452,"ancestor_record_offset":64344,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":65452,"parent_record_offset":65396,"ancestor_record_offset":64344,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65424,"parent_record_offset":65396,"ancestor_record_offset":64344,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65396,"parent_record_offset":65372,"ancestor_record_offset":64344,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65372,"parent_record_offset":65348,"ancestor_record_offset":64344,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":66548,"parent_record_offset":66524,"ancestor_record_offset":64344,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":101,"record_offset":64844,"parent_record_offset":64820,"ancestor_record_offset":64344,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":101,"record_offset":64908,"parent_record_offset":64876,"ancestor_record_offset":64344,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":64968,"parent_record_offset":64936,"ancestor_record_offset":64344,"inlinee":8549,"source_line":2064,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":101,"record_offset":66228,"parent_record_offset":66196,"ancestor_record_offset":64344,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":65036,"parent_record_offset":65008,"ancestor_record_offset":64344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":66444,"parent_record_offset":66420,"ancestor_record_offset":64344,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":67564,"parent_record_offset":67504,"ancestor_record_offset":64344,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":64728,"parent_record_offset":64692,"ancestor_record_offset":64344,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":67588,"parent_record_offset":67564,"ancestor_record_offset":64344,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":64752,"parent_record_offset":64728,"ancestor_record_offset":64344,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":66468,"parent_record_offset":66444,"ancestor_record_offset":64344,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":67480,"parent_record_offset":67456,"ancestor_record_offset":64344,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":101,"record_offset":66420,"parent_record_offset":66396,"ancestor_record_offset":64344,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66320,"parent_record_offset":66296,"ancestor_record_offset":64344,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66396,"parent_record_offset":66368,"ancestor_record_offset":64344,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66344,"parent_record_offset":66320,"ancestor_record_offset":64344,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66368,"parent_record_offset":66344,"ancestor_record_offset":64344,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66296,"parent_record_offset":66256,"ancestor_record_offset":64344,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":64692,"parent_record_offset":64664,"ancestor_record_offset":64344,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":67504,"parent_record_offset":67480,"ancestor_record_offset":64344,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":64784,"parent_record_offset":64692,"ancestor_record_offset":64344,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":67536,"parent_record_offset":67504,"ancestor_record_offset":64344,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":101,"record_offset":66688,"parent_record_offset":66664,"ancestor_record_offset":64344,"inlinee":7263,"source_line":156,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":101,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":64344,"inlinee":10229,"source_line":145,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":101,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":64344,"inlinee":10230,"source_line":51,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::impl$0::with_active::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":101,"record_offset":66524,"parent_record_offset":66256,"ancestor_record_offset":64344,"inlinee":7261,"source_line":57,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub_impl::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":101,"record_offset":66256,"parent_record_offset":66196,"ancestor_record_offset":64344,"inlinee":7255,"source_line":149,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":101,"record_offset":65184,"parent_record_offset":65160,"ancestor_record_offset":64344,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":101,"record_offset":66196,"parent_record_offset":66160,"ancestor_record_offset":64344,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":101,"record_offset":66160,"parent_record_offset":66124,"ancestor_record_offset":64344,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":101,"record_offset":64936,"parent_record_offset":64908,"ancestor_record_offset":64344,"inlinee":11607,"source_line":108,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B93
// 首条位置: {"module":20,"inlinee":7059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B92
// 首条位置: {"module":20,"inlinee":7058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B91
// 首条位置: {"module":20,"inlinee":7057}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::create_minidump_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":57444,"rva":2807936,"va":5371517056}
// 编译实例 RVAs: 0x2ad880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3564192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e8a0623600","size":5,"soft_fail":false,"target_va":5375081504,"va":5371517307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11310627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e82396ac00","size":5,"soft_fail":false,"target_va":5382827728,"va":5371517096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11322972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e85cc6ac00","size":5,"soft_fail":false,"target_va":5382840208,"va":5371517231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11192468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e894c8aa00","size":5,"soft_fail":false,"target_va":5382709584,"va":5371517111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11191289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e8f9c3aa00","size":5,"soft_fail":false,"target_va":5382708672,"va":5371517378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11226792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e8a84eab00","size":5,"soft_fail":false,"target_va":5382744080,"va":5371517283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9841139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517056,"raw_hex":"e8f3299600","size":5,"soft_fail":false,"target_va":5381358304,"va":5371517160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":101,"record_offset":57924,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":59044,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":57896,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":58636,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":11592,"source_line":450,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":57616,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":58144,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":58004,"parent_record_offset":57444,"ancestor_record_offset":57444,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":58352,"parent_record_offset":58328,"ancestor_record_offset":57444,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":58328,"parent_record_offset":58272,"ancestor_record_offset":57444,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":101,"record_offset":57972,"parent_record_offset":57948,"ancestor_record_offset":57444,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":58300,"parent_record_offset":58272,"ancestor_record_offset":57444,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":58272,"parent_record_offset":58248,"ancestor_record_offset":57444,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":58248,"parent_record_offset":58224,"ancestor_record_offset":57444,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":101,"record_offset":57668,"parent_record_offset":57616,"ancestor_record_offset":57444,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":57948,"parent_record_offset":57924,"ancestor_record_offset":57444,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":58176,"parent_record_offset":58144,"ancestor_record_offset":57444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":57764,"parent_record_offset":57740,"ancestor_record_offset":57444,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":57812,"parent_record_offset":57788,"ancestor_record_offset":57444,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":57788,"parent_record_offset":57764,"ancestor_record_offset":57444,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":57864,"parent_record_offset":57616,"ancestor_record_offset":57444,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":57640,"parent_record_offset":57616,"ancestor_record_offset":57444,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":58660,"parent_record_offset":58636,"ancestor_record_offset":57444,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":58980,"parent_record_offset":58636,"ancestor_record_offset":57444,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":57740,"parent_record_offset":57716,"ancestor_record_offset":57444,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":57716,"parent_record_offset":57692,"ancestor_record_offset":57444,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":57692,"parent_record_offset":57668,"ancestor_record_offset":57444,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":58724,"parent_record_offset":58660,"ancestor_record_offset":57444,"inlinee":11597,"source_line":218,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":58752,"parent_record_offset":58660,"ancestor_record_offset":57444,"inlinee":11598,"source_line":215,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":58696,"parent_record_offset":58660,"ancestor_record_offset":57444,"inlinee":11596,"source_line":209,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":59900,"rva":2808544,"va":5371517664}
// 编译实例 RVAs: 0x2adae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..File$u20$as$u20$std..io..Read$GT$::read_to_end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11190328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e838c0aa00","size":5,"soft_fail":false,"target_va":5382708272,"va":5371517939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1361257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8973aebff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371519380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1185275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8fb151200","size":5,"soft_fail":false,"target_va":5372703552,"va":5371518272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3381415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8a7983300","size":5,"soft_fail":false,"target_va":5374900736,"va":5371519316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406023d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3491184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e870453500","size":5,"soft_fail":false,"target_va":5375009744,"va":5371518555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3563526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e806603600","size":5,"soft_fail":false,"target_va":5375081504,"va":5371517973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11309847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e81793ac00","size":5,"soft_fail":false,"target_va":5382827728,"va":5371517876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40328806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8665e6702","size":5,"soft_fail":false,"target_va":5411847168,"va":5371518357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8e7f4ffff","size":5,"soft_fail":false,"target_va":5371516608,"va":5371519444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<minidumper::MinidumpBinary,enum2$<minidumper::errors::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e841fcffff","size":5,"soft_fail":false,"target_va":5371516752,"va":5371517706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40024106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e82ab86202","size":5,"soft_fail":false,"target_va":5411542880,"va":5371518769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39973675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e82bf36102","size":5,"soft_fail":false,"target_va":5411492368,"va":5371518688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39976078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e88efc6102","size":5,"soft_fail":false,"target_va":5411494800,"va":5371518717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10305364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8543f9d00","size":5,"soft_fail":false,"target_va":5381823600,"va":5371518231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10305377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8613f9d00","size":5,"soft_fail":false,"target_va":5381824016,"va":5371518634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11190766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371517664,"raw_hex":"e8eec1aa00","size":5,"soft_fail":false,"target_va":5382708672,"va":5371517901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371517664,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371518097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":62268,"parent_record_offset":59900,"ancestor_record_offset":59900,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":62452,"parent_record_offset":59900,"ancestor_record_offset":59900,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":60064,"parent_record_offset":59900,"ancestor_record_offset":59900,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":62596,"parent_record_offset":59900,"ancestor_record_offset":59900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":62296,"parent_record_offset":59900,"ancestor_record_offset":59900,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60844,"parent_record_offset":60820,"ancestor_record_offset":59900,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60820,"parent_record_offset":60764,"ancestor_record_offset":59900,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":61276,"parent_record_offset":61248,"ancestor_record_offset":59900,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60792,"parent_record_offset":60764,"ancestor_record_offset":59900,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60764,"parent_record_offset":60740,"ancestor_record_offset":59900,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60740,"parent_record_offset":60716,"ancestor_record_offset":59900,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":101,"record_offset":60260,"parent_record_offset":60192,"ancestor_record_offset":59900,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":101,"record_offset":61308,"parent_record_offset":61276,"ancestor_record_offset":59900,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":101,"record_offset":60372,"parent_record_offset":60288,"ancestor_record_offset":59900,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":101,"record_offset":61332,"parent_record_offset":61308,"ancestor_record_offset":59900,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":101,"record_offset":62488,"parent_record_offset":62452,"ancestor_record_offset":59900,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":60152,"parent_record_offset":60112,"ancestor_record_offset":59900,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":62556,"parent_record_offset":62520,"ancestor_record_offset":59900,"inlinee":8549,"source_line":2064,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":60668,"parent_record_offset":60640,"ancestor_record_offset":59900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61076,"parent_record_offset":61048,"ancestor_record_offset":59900,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61124,"parent_record_offset":61100,"ancestor_record_offset":59900,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61100,"parent_record_offset":61076,"ancestor_record_offset":59900,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60640,"parent_record_offset":60192,"ancestor_record_offset":59900,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60900,"parent_record_offset":60192,"ancestor_record_offset":59900,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60600,"parent_record_offset":60192,"ancestor_record_offset":59900,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":63484,"parent_record_offset":63420,"ancestor_record_offset":59900,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":62360,"parent_record_offset":62324,"ancestor_record_offset":59900,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":63508,"parent_record_offset":63484,"ancestor_record_offset":59900,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":62384,"parent_record_offset":62360,"ancestor_record_offset":59900,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":63392,"parent_record_offset":63364,"ancestor_record_offset":59900,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":62324,"parent_record_offset":62296,"ancestor_record_offset":59900,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":63420,"parent_record_offset":63392,"ancestor_record_offset":59900,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":62416,"parent_record_offset":62324,"ancestor_record_offset":59900,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":63456,"parent_record_offset":63420,"ancestor_record_offset":59900,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60468,"parent_record_offset":60444,"ancestor_record_offset":59900,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60516,"parent_record_offset":60492,"ancestor_record_offset":59900,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60492,"parent_record_offset":60468,"ancestor_record_offset":59900,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60288,"parent_record_offset":60192,"ancestor_record_offset":59900,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60312,"parent_record_offset":60288,"ancestor_record_offset":59900,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60568,"parent_record_offset":60288,"ancestor_record_offset":59900,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61048,"parent_record_offset":61020,"ancestor_record_offset":59900,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61020,"parent_record_offset":60992,"ancestor_record_offset":59900,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":101,"record_offset":61868,"parent_record_offset":61844,"ancestor_record_offset":59900,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60444,"parent_record_offset":60420,"ancestor_record_offset":59900,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60420,"parent_record_offset":60396,"ancestor_record_offset":59900,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60396,"parent_record_offset":60372,"ancestor_record_offset":59900,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":60340,"parent_record_offset":60312,"ancestor_record_offset":59900,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":101,"record_offset":62520,"parent_record_offset":62488,"ancestor_record_offset":59900,"inlinee":11607,"source_line":108,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":60112,"parent_record_offset":60064,"ancestor_record_offset":59900,"inlinee":11601,"source_line":94,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":60192,"parent_record_offset":60152,"ancestor_record_offset":59900,"inlinee":11603,"source_line":95,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D51
// 首条位置: {"module":101,"inlinee":11601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":60152,"parent_record_offset":60112,"ancestor_record_offset":60112,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60844,"parent_record_offset":60820,"ancestor_record_offset":60112,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60820,"parent_record_offset":60764,"ancestor_record_offset":60112,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":61276,"parent_record_offset":61248,"ancestor_record_offset":60112,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60792,"parent_record_offset":60764,"ancestor_record_offset":60112,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60764,"parent_record_offset":60740,"ancestor_record_offset":60112,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60740,"parent_record_offset":60716,"ancestor_record_offset":60112,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":101,"record_offset":60260,"parent_record_offset":60192,"ancestor_record_offset":60112,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":101,"record_offset":61308,"parent_record_offset":61276,"ancestor_record_offset":60112,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":101,"record_offset":60372,"parent_record_offset":60288,"ancestor_record_offset":60112,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":101,"record_offset":61332,"parent_record_offset":61308,"ancestor_record_offset":60112,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":60668,"parent_record_offset":60640,"ancestor_record_offset":60112,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61076,"parent_record_offset":61048,"ancestor_record_offset":60112,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61124,"parent_record_offset":61100,"ancestor_record_offset":60112,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61100,"parent_record_offset":61076,"ancestor_record_offset":60112,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60640,"parent_record_offset":60192,"ancestor_record_offset":60112,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60900,"parent_record_offset":60192,"ancestor_record_offset":60112,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60600,"parent_record_offset":60192,"ancestor_record_offset":60112,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60468,"parent_record_offset":60444,"ancestor_record_offset":60112,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60516,"parent_record_offset":60492,"ancestor_record_offset":60112,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60492,"parent_record_offset":60468,"ancestor_record_offset":60112,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60288,"parent_record_offset":60192,"ancestor_record_offset":60112,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60312,"parent_record_offset":60288,"ancestor_record_offset":60112,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60568,"parent_record_offset":60288,"ancestor_record_offset":60112,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61048,"parent_record_offset":61020,"ancestor_record_offset":60112,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61020,"parent_record_offset":60992,"ancestor_record_offset":60112,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":101,"record_offset":61868,"parent_record_offset":61844,"ancestor_record_offset":60112,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60444,"parent_record_offset":60420,"ancestor_record_offset":60112,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60420,"parent_record_offset":60396,"ancestor_record_offset":60112,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60396,"parent_record_offset":60372,"ancestor_record_offset":60112,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":60340,"parent_record_offset":60312,"ancestor_record_offset":60112,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":60192,"parent_record_offset":60152,"ancestor_record_offset":60112,"inlinee":11603,"source_line":95,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D53
// 首条位置: {"module":101,"inlinee":11603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":101,"record_offset":60260,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":61772,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60640,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60900,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":60600,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60288,"parent_record_offset":60192,"ancestor_record_offset":60192,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60844,"parent_record_offset":60820,"ancestor_record_offset":60192,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":60820,"parent_record_offset":60764,"ancestor_record_offset":60192,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":61276,"parent_record_offset":61248,"ancestor_record_offset":60192,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60792,"parent_record_offset":60764,"ancestor_record_offset":60192,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60764,"parent_record_offset":60740,"ancestor_record_offset":60192,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":60740,"parent_record_offset":60716,"ancestor_record_offset":60192,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":101,"record_offset":61308,"parent_record_offset":61276,"ancestor_record_offset":60192,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":101,"record_offset":60372,"parent_record_offset":60288,"ancestor_record_offset":60192,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":101,"record_offset":61332,"parent_record_offset":61308,"ancestor_record_offset":60192,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":60668,"parent_record_offset":60640,"ancestor_record_offset":60192,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61076,"parent_record_offset":61048,"ancestor_record_offset":60192,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61124,"parent_record_offset":61100,"ancestor_record_offset":60192,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":101,"record_offset":61100,"parent_record_offset":61076,"ancestor_record_offset":60192,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60468,"parent_record_offset":60444,"ancestor_record_offset":60192,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60516,"parent_record_offset":60492,"ancestor_record_offset":60192,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":101,"record_offset":60492,"parent_record_offset":60468,"ancestor_record_offset":60192,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60312,"parent_record_offset":60288,"ancestor_record_offset":60192,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":101,"record_offset":60568,"parent_record_offset":60288,"ancestor_record_offset":60192,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61048,"parent_record_offset":61020,"ancestor_record_offset":60192,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":101,"record_offset":61020,"parent_record_offset":60992,"ancestor_record_offset":60192,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":101,"record_offset":61868,"parent_record_offset":61844,"ancestor_record_offset":60192,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60444,"parent_record_offset":60420,"ancestor_record_offset":60192,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60420,"parent_record_offset":60396,"ancestor_record_offset":60192,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":101,"record_offset":60396,"parent_record_offset":60372,"ancestor_record_offset":60192,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":101,"record_offset":60340,"parent_record_offset":60312,"ancestor_record_offset":60192,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":64208,"rva":2810352,"va":5371519472}
// 编译实例 RVAs: 0x2ae1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140056ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2454400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8808cdaff","size":5,"soft_fail":false,"target_va":5369065152,"va":5371519547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140084e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2267486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8a266ddff","size":5,"soft_fail":false,"target_va":5369253488,"va":5371520969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2145778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e80e42dfff","size":5,"soft_fail":false,"target_va":5369374496,"va":5371520269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a8150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2122929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e84f9bdfff","size":5,"soft_fail":false,"target_va":5369397584,"va":5371520508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=3
//     target_aliases=47 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1364022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8ca2febff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371522145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140174f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1283540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e82c6aecff","size":5,"soft_fail":false,"target_va":5370236800,"va":5371520335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1183659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8ab0f1200","size":5,"soft_fail":false,"target_va":5372703552,"va":5371519888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3382904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8789e3300","size":5,"soft_fail":false,"target_va":5374904480,"va":5371521571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406009e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3482247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e887223500","size":5,"soft_fail":false,"target_va":5375003104,"va":5371520852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406023d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3488707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8c33b3500","size":5,"soft_fail":false,"target_va":5375009744,"va":5371521032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3561387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8ab573600","size":5,"soft_fail":false,"target_va":5375081504,"va":5371520112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39804313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8995d5f02","size":5,"soft_fail":false,"target_va":5411324704,"va":5371520386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40327550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e87e596702","size":5,"soft_fail":false,"target_va":5411847168,"va":5371519613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8fae9ffff","size":5,"soft_fail":false,"target_va":5371516608,"va":5371522241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40021454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8cead6202","size":5,"soft_fail":false,"target_va":5411542880,"va":5371521421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39971092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e814e96102","size":5,"soft_fail":false,"target_va":5411492368,"va":5371521271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39973865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e8e9f36102","size":5,"soft_fail":false,"target_va":5411494800,"va":5371520930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10303745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e801399d00","size":5,"soft_fail":false,"target_va":5381823600,"va":5371519850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10303112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e888369d00","size":5,"soft_fail":false,"target_va":5381824016,"va":5371520899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_core::hub_impl::_$LT$impl$u20$sentry_core..hub..Hub$GT$::is_active_and_usage_safe
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3886682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e85a4e3b00","size":5,"soft_fail":false,"target_va":5375407088,"va":5371520401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11200347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e85be7aa00","size":5,"soft_fail":false,"target_va":5382720160,"va":5371519808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40015664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371519472,"raw_hex":"e830976202","size":5,"soft_fail":false,"target_va":5411537104,"va":5371521435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*49168277(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371519472,"raw_hex":"ff15953fee02","size":6,"soft_fail":false,"va":5371520085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::on_client_message
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":64344,"parent_record_offset":64208,"ancestor_record_offset":64208,"inlinee":11609,"source_line":35,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":65476,"parent_record_offset":65452,"ancestor_record_offset":64208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":65452,"parent_record_offset":65396,"ancestor_record_offset":64208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65424,"parent_record_offset":65396,"ancestor_record_offset":64208,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65396,"parent_record_offset":65372,"ancestor_record_offset":64208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":65372,"parent_record_offset":65348,"ancestor_record_offset":64208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":64636,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":101,"record_offset":64844,"parent_record_offset":64820,"ancestor_record_offset":64208,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":101,"record_offset":64608,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":101,"record_offset":64908,"parent_record_offset":64876,"ancestor_record_offset":64208,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":64876,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":64968,"parent_record_offset":64936,"ancestor_record_offset":64208,"inlinee":8549,"source_line":2064,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":101,"record_offset":66228,"parent_record_offset":66196,"ancestor_record_offset":64208,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":65008,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":64564,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":66444,"parent_record_offset":66420,"ancestor_record_offset":64208,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":67564,"parent_record_offset":67504,"ancestor_record_offset":64208,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":64728,"parent_record_offset":64692,"ancestor_record_offset":64208,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":67588,"parent_record_offset":67564,"ancestor_record_offset":64208,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":64752,"parent_record_offset":64728,"ancestor_record_offset":64208,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":66468,"parent_record_offset":66444,"ancestor_record_offset":64208,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":64664,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":64820,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":67480,"parent_record_offset":67456,"ancestor_record_offset":64208,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":101,"record_offset":66420,"parent_record_offset":66396,"ancestor_record_offset":64208,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66320,"parent_record_offset":66296,"ancestor_record_offset":64208,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66396,"parent_record_offset":66368,"ancestor_record_offset":64208,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66344,"parent_record_offset":66320,"ancestor_record_offset":64208,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66368,"parent_record_offset":66344,"ancestor_record_offset":64208,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":101,"record_offset":66296,"parent_record_offset":66256,"ancestor_record_offset":64208,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":64692,"parent_record_offset":64664,"ancestor_record_offset":64208,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":101,"record_offset":67504,"parent_record_offset":67480,"ancestor_record_offset":64208,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":64784,"parent_record_offset":64692,"ancestor_record_offset":64208,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":101,"record_offset":67536,"parent_record_offset":67504,"ancestor_record_offset":64208,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":101,"record_offset":65784,"parent_record_offset":64344,"ancestor_record_offset":64208,"inlinee":10241,"source_line":151,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":101,"record_offset":66688,"parent_record_offset":66664,"ancestor_record_offset":64208,"inlinee":7263,"source_line":156,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":101,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":64208,"inlinee":10229,"source_line":145,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":101,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":64208,"inlinee":10230,"source_line":51,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::impl$0::with_active::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":101,"record_offset":66524,"parent_record_offset":66256,"ancestor_record_offset":64208,"inlinee":7261,"source_line":57,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub_impl::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":101,"record_offset":66256,"parent_record_offset":66196,"ancestor_record_offset":64208,"inlinee":7255,"source_line":149,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":101,"record_offset":65184,"parent_record_offset":65160,"ancestor_record_offset":64208,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":101,"record_offset":66196,"parent_record_offset":66160,"ancestor_record_offset":64208,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":101,"record_offset":66160,"parent_record_offset":66124,"ancestor_record_offset":64208,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":101,"record_offset":64936,"parent_record_offset":64908,"ancestor_record_offset":64208,"inlinee":11607,"source_line":108,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":18604,"rva":9072,"va":5368718192}
// 编译实例 RVAs: 0x2370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":68888,"rva":9072,"va":5368718192}
// 编译实例 RVAs: 0x2370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":101,"record_offset":69028,"rva":2813568,"va":5371522688}
// 编译实例 RVAs: 0x2aee80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11121244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e85cb2a900","size":5,"soft_fail":false,"target_va":5382644528,"va":5371523279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d6950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1934849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8ff79e2ff","size":5,"soft_fail":false,"target_va":5369588048,"va":5371522892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t223620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e884690300","size":5,"soft_fail":false,"target_va":5371747712,"va":5371524087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e6000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t224228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8e46b0300","size":5,"soft_fail":false,"target_va":5371748352,"va":5371524119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee510
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3404536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8f8f23300","size":5,"soft_fail":false,"target_va":5374928144,"va":5371523603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f7420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3440181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8357e3400","size":5,"soft_fail":false,"target_va":5374964768,"va":5371524582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fff90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3475713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e801093500","size":5,"soft_fail":false,"target_va":5375000464,"va":5371524746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3558728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8484d3600","size":5,"soft_fail":false,"target_va":5375081488,"va":5371522755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3558497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8614c3600","size":5,"soft_fail":false,"target_va":5375081504,"va":5371523002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3992925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e85ded3c00","size":5,"soft_fail":false,"target_va":5375516032,"va":5371523102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40323253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8b5486702","size":5,"soft_fail":false,"target_va":5411847168,"va":5371523910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40016722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8529b6202","size":5,"soft_fail":false,"target_va":5411540992,"va":5371524265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40017481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8499e6202","size":5,"soft_fail":false,"target_va":5411541808,"va":5371524322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::server::Server::run
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     evidence={"assembly":"\tcallq\t4811817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371522688,"raw_hex":"e8296c4900","size":5,"soft_fail":false,"target_va":5376335360,"va":5371523538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":71608,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":11388,"source_line":2412,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":69204,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":101,"record_offset":71400,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":101,"record_offset":71976,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":71884,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":101,"record_offset":72520,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":72552,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":69788,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":69756,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":71280,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":72892,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":101,"record_offset":71920,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":11613,"source_line":241,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":101,"record_offset":71432,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":71316,"parent_record_offset":69028,"ancestor_record_offset":69028,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":69348,"parent_record_offset":69324,"ancestor_record_offset":69028,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":69372,"parent_record_offset":69348,"ancestor_record_offset":69028,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70108,"parent_record_offset":70084,"ancestor_record_offset":69028,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70776,"parent_record_offset":70740,"ancestor_record_offset":69028,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":69324,"parent_record_offset":69300,"ancestor_record_offset":69028,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70084,"parent_record_offset":70028,"ancestor_record_offset":69028,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":70740,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":69240,"parent_record_offset":69204,"ancestor_record_offset":69028,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":69268,"parent_record_offset":69240,"ancestor_record_offset":69028,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":69300,"parent_record_offset":69268,"ancestor_record_offset":69028,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":69420,"parent_record_offset":69204,"ancestor_record_offset":69028,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":70056,"parent_record_offset":70028,"ancestor_record_offset":69028,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":70028,"parent_record_offset":70004,"ancestor_record_offset":69028,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":70004,"parent_record_offset":69980,"ancestor_record_offset":69028,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":71672,"parent_record_offset":71636,"ancestor_record_offset":69028,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::is_unique
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":71636,"parent_record_offset":71608,"ancestor_record_offset":69028,"inlinee":11389,"source_line":2497,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":70540,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":72576,"parent_record_offset":72552,"ancestor_record_offset":69028,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":101,"record_offset":71944,"parent_record_offset":71920,"ancestor_record_offset":69028,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":101,"record_offset":70332,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":101,"record_offset":69444,"parent_record_offset":69420,"ancestor_record_offset":69028,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":101,"record_offset":72000,"parent_record_offset":71976,"ancestor_record_offset":69028,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":101,"record_offset":70420,"parent_record_offset":70388,"ancestor_record_offset":69028,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":101,"record_offset":70448,"parent_record_offset":70388,"ancestor_record_offset":69028,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":69836,"parent_record_offset":69788,"ancestor_record_offset":69028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":101,"record_offset":71704,"parent_record_offset":71672,"ancestor_record_offset":69028,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":72028,"parent_record_offset":72000,"ancestor_record_offset":69028,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":70884,"parent_record_offset":70856,"ancestor_record_offset":69028,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":72056,"parent_record_offset":72000,"ancestor_record_offset":69028,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":70964,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70568,"parent_record_offset":70540,"ancestor_record_offset":69028,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":72612,"parent_record_offset":72576,"ancestor_record_offset":69028,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":69028,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":71820,"parent_record_offset":71636,"ancestor_record_offset":69028,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70592,"parent_record_offset":70568,"ancestor_record_offset":69028,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":71732,"parent_record_offset":71672,"ancestor_record_offset":69028,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":69028,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":71844,"parent_record_offset":71820,"ancestor_record_offset":69028,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":72636,"parent_record_offset":72612,"ancestor_record_offset":69028,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":72668,"parent_record_offset":72576,"ancestor_record_offset":69028,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":101,"record_offset":70932,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":8077,"source_line":244,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70476,"parent_record_offset":70388,"ancestor_record_offset":69028,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":101,"record_offset":70364,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":101,"record_offset":70388,"parent_record_offset":70364,"ancestor_record_offset":69028,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70500,"parent_record_offset":70476,"ancestor_record_offset":69028,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70248,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":70712,"parent_record_offset":70176,"ancestor_record_offset":69028,"inlinee":11612,"source_line":23,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":70176,"parent_record_offset":69788,"ancestor_record_offset":69028,"inlinee":11611,"source_line":135,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D5B
// 首条位置: {"module":101,"inlinee":11611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":101,"record_offset":70740,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":101,"record_offset":70540,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":101,"record_offset":70332,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":70992,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":101,"record_offset":70964,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":101,"record_offset":70932,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":8077,"source_line":244,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":101,"record_offset":70364,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70248,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":101,"record_offset":70712,"parent_record_offset":70176,"ancestor_record_offset":70176,"inlinee":11612,"source_line":23,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70832,"parent_record_offset":70808,"ancestor_record_offset":70176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70856,"parent_record_offset":70832,"ancestor_record_offset":70176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":71208,"parent_record_offset":71184,"ancestor_record_offset":70176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70776,"parent_record_offset":70740,"ancestor_record_offset":70176,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":70808,"parent_record_offset":70776,"ancestor_record_offset":70176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":101,"record_offset":71184,"parent_record_offset":71160,"ancestor_record_offset":70176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":71160,"parent_record_offset":71136,"ancestor_record_offset":70176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":101,"record_offset":71136,"parent_record_offset":71112,"ancestor_record_offset":70176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":101,"record_offset":70420,"parent_record_offset":70388,"ancestor_record_offset":70176,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":101,"record_offset":70448,"parent_record_offset":70388,"ancestor_record_offset":70176,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":71016,"parent_record_offset":70992,"ancestor_record_offset":70176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":101,"record_offset":70884,"parent_record_offset":70856,"ancestor_record_offset":70176,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70568,"parent_record_offset":70540,"ancestor_record_offset":70176,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":70176,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70592,"parent_record_offset":70568,"ancestor_record_offset":70176,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":101,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":70176,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70476,"parent_record_offset":70388,"ancestor_record_offset":70176,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":101,"record_offset":70388,"parent_record_offset":70364,"ancestor_record_offset":70176,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00101.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":101,"record_offset":70500,"parent_record_offset":70476,"ancestor_record_offset":70176,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00101.jsonl.gz"}
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
// 完整结构及原始类型记录: server.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::server::Handler
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "minidumper::ServerHandler::message_alloc<tiny_glade::crash_pickup::server::Handler>", "constructor": "minidumper::ServerHandler::message_alloc", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "minidumper::ServerHandler::on_client_connected<tiny_glade::crash_pickup::server::Handler>", "constructor": "minidumper::ServerHandler::on_client_connected", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::server::Handler>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: server.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: server.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>

// ------------------------------------------------------------------
// 函数: bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >
// 信息 ID: 3fe155b954afae1ed78f9856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 参数类型拼写: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [符号内函数类型候选] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 参数类型拼写: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::resources::gui::DebugGui"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::resources::walls::public_walls::PublicWalls"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::systems::ui::signifiers::signifier_create::UiSignifierStream"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::resources::gui::DebugGui"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::resources::walls::public_walls::PublicWalls"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::systems::ui::signifiers::signifier_create::UiSignifierStream"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::system::function_system::FunctionSystem -> ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::system::schedule_system::impl$1::is_exclusive -> ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>"]; 字段布局 = unknown。
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
// 函数: tiny_glade::crash_pickup::server::Handler::new
// 信息 ID: 3e67a2c88a8da748cf57e63d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=101, inline_record=70712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::Handler::on_client_message
// 信息 ID: 6ed8e6c9160415e987a014ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=101, inline_record=64344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 信息 ID: 619f56a831411af1fba912a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >; module=20, inline_record=7132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$1
// 信息 ID: 8ed9a91963a7794f45b34283
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- sentry_core::hub_impl::HubImpl::with_mut<sentry_core::hub_impl::impl$3::with_current_scope_mut::closure_env$0<sentry_core::hub::impl$0::configure_scope::closure_env$1<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >; module=20, inline_record=5864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$2
// 信息 ID: 25f30687bb8fa3caa6b398e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- core::ptr::drop_in_place<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- core::ptr::drop_in_place<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >; module=20, inline_record=4844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::create_minidump_file
// 信息 ID: d1d1d527a488ea73067176c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 信息 ID: 57ba0f70ba9e52d559afb832
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
// 函数: tiny_glade::crash_pickup::server::impl$1::on_message
// 信息 ID: c87153a079ec2273c9937407
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 信息 ID: 48352bf95250450fd9fd4719
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 信息 ID: aa32a8946f55afdd058ed1a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=101, inline_record=60112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 信息 ID: e887f15be34445848733c286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=101, inline_record=60192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run
// 信息 ID: 13aada60114553cc905453e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dc5a5 -> 0x2aee80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run::closure$0
// 信息 ID: 3c5825d39a0f92383ac24ee7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=101, inline_record=70176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
