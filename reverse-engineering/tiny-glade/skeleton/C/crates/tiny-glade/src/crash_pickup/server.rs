// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/crash_pickup/server.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 79cb6cdcb01b362f8f66cb5c8f19d82e2f5723e7d1ca6f45a4ba05f52c9ac7d2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 30, 117, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50","version":"0.8.2"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
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
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D40
// 首条位置: {"module":30,"inlinee":7488}
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
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D5C
// 首条位置: {"module":30,"inlinee":7516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":25420,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":7517,"source_line":560,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":27992,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":25796,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":28232,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":25964,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":28020,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":28176,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":25852,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25712,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":30,"record_offset":29368,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":7525,"source_line":151,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":26104,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":26552,"parent_record_offset":25060,"ancestor_record_offset":25060,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":25060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":25060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":25060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25580,"parent_record_offset":25524,"ancestor_record_offset":25060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":27292,"parent_record_offset":27264,"ancestor_record_offset":25060,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25552,"parent_record_offset":25524,"ancestor_record_offset":25060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25524,"parent_record_offset":25500,"ancestor_record_offset":25060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":25060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":30704,"parent_record_offset":30680,"ancestor_record_offset":25060,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":30,"record_offset":28200,"parent_record_offset":28176,"ancestor_record_offset":25060,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":30,"record_offset":28264,"parent_record_offset":28232,"ancestor_record_offset":25060,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":25060,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":28328,"parent_record_offset":28292,"ancestor_record_offset":25060,"inlinee":7523,"source_line":2297,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":30,"record_offset":30360,"parent_record_offset":30320,"ancestor_record_offset":25060,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":25452,"parent_record_offset":25420,"ancestor_record_offset":25060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":25060,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":32396,"parent_record_offset":32332,"ancestor_record_offset":25060,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":28084,"parent_record_offset":28048,"ancestor_record_offset":25060,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":25060,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":25060,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":28108,"parent_record_offset":28084,"ancestor_record_offset":25060,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":25760,"parent_record_offset":25736,"ancestor_record_offset":25060,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":25060,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":25060,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":25060,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":25060,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":30,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":25060,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30464,"parent_record_offset":30436,"ancestor_record_offset":25060,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30552,"parent_record_offset":30520,"ancestor_record_offset":25060,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30492,"parent_record_offset":30464,"ancestor_record_offset":25060,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30520,"parent_record_offset":30492,"ancestor_record_offset":25060,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30436,"parent_record_offset":30388,"ancestor_record_offset":25060,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":28048,"parent_record_offset":28020,"ancestor_record_offset":25060,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":25060,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":28140,"parent_record_offset":28048,"ancestor_record_offset":25060,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":32368,"parent_record_offset":32332,"ancestor_record_offset":25060,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":30,"record_offset":30844,"parent_record_offset":30820,"ancestor_record_offset":25060,"inlinee":6543,"source_line":156,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":30,"record_offset":29416,"parent_record_offset":29392,"ancestor_record_offset":25060,"inlinee":7527,"source_line":145,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":30,"record_offset":29392,"parent_record_offset":29368,"ancestor_record_offset":25060,"inlinee":7526,"source_line":51,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::impl$0::with_active::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":30,"record_offset":30680,"parent_record_offset":30388,"ancestor_record_offset":25060,"inlinee":6541,"source_line":57,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub_impl::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":30,"record_offset":30388,"parent_record_offset":30320,"ancestor_record_offset":25060,"inlinee":6521,"source_line":149,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":30,"record_offset":30320,"parent_record_offset":30280,"ancestor_record_offset":25060,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":30,"record_offset":30280,"parent_record_offset":30244,"ancestor_record_offset":25060,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":30,"record_offset":28292,"parent_record_offset":28264,"ancestor_record_offset":25060,"inlinee":7521,"source_line":106,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30F4
// 首条位置: {"module":117,"inlinee":12532}
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
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30F3
// 首条位置: {"module":117,"inlinee":12531}
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
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30F2
// 首条位置: {"module":117,"inlinee":12530}
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
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":41480,"rva":1420160,"va":5370129280}
// 编译实例 RVAs: 0x15ab80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14560277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e8152cde00","size":5,"soft_fail":false,"target_va":5384689616,"va":5370129334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14573987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e8a361de00","size":5,"soft_fail":false,"target_va":5384703536,"va":5370129544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14526787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e843a9dd00","size":5,"soft_fail":false,"target_va":5384656240,"va":5370129448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5549226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e8aaac5400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370129473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14665451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e8ebc6df00","size":5,"soft_fail":false,"target_va":5384794864,"va":5370129408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13929550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129280,"raw_hex":"e84e8cd400","size":5,"soft_fail":false,"target_va":5384058928,"va":5370129373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":30,"record_offset":41788,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":42712,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":41760,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":42476,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":7543,"source_line":638,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":41648,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":41868,"parent_record_offset":41480,"ancestor_record_offset":41480,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":42080,"parent_record_offset":42056,"ancestor_record_offset":41480,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":42104,"parent_record_offset":42080,"ancestor_record_offset":41480,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":42128,"parent_record_offset":42104,"ancestor_record_offset":41480,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":42056,"parent_record_offset":42000,"ancestor_record_offset":41480,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":30,"record_offset":41836,"parent_record_offset":41812,"ancestor_record_offset":41480,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":42028,"parent_record_offset":42000,"ancestor_record_offset":41480,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":42000,"parent_record_offset":41976,"ancestor_record_offset":41480,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":41976,"parent_record_offset":41952,"ancestor_record_offset":41480,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":30,"record_offset":41700,"parent_record_offset":41648,"ancestor_record_offset":41480,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":41812,"parent_record_offset":41788,"ancestor_record_offset":41480,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":41904,"parent_record_offset":41868,"ancestor_record_offset":41480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":41728,"parent_record_offset":41648,"ancestor_record_offset":41480,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":41672,"parent_record_offset":41648,"ancestor_record_offset":41480,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":42500,"parent_record_offset":42476,"ancestor_record_offset":41480,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":42652,"parent_record_offset":42476,"ancestor_record_offset":41480,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":42564,"parent_record_offset":42500,"ancestor_record_offset":41480,"inlinee":7545,"source_line":227,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":42592,"parent_record_offset":42500,"ancestor_record_offset":41480,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":42536,"parent_record_offset":42500,"ancestor_record_offset":41480,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":36224,"rva":1417216,"va":5370126336}
// 编译实例 RVAs: 0x15a000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::File as std::io::Read>::read_to_end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14643916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8cc72df00","size":5,"soft_fail":false,"target_va":5384771200,"va":5370127279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14576293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8a56ade00","size":5,"soft_fail":false,"target_va":5384703536,"va":5370127238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t14482801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e871fddc00","size":5,"soft_fail":false,"target_va":5384609408,"va":5370126602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t5551362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e802b55400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370127337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-823259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e82570f3ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5370128838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-788567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8a9f7f3ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370127410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009bbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-782280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e83810f4ff","size":5,"soft_fail":false,"target_va":5369346992,"va":5370129267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bac40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-653929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e89705f6ff","size":5,"soft_fail":false,"target_va":5369474112,"va":5370128036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4857187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8631d4a00","size":5,"soft_fail":false,"target_va":5374984944,"va":5370127752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46929213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e83d15cc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370127838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8c5d7ffff","size":5,"soft_fail":false,"target_va":5370118624,"va":5370128918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<minidumper::MinidumpBinary,enum2$<minidumper::errors::Error> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e88fdaffff","size":5,"soft_fail":false,"target_va":5370118752,"va":5370128332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46493166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e8ee6dc502","size":5,"soft_fail":false,"target_va":5416621632,"va":5370128461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46448449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e841bfc402","size":5,"soft_fail":false,"target_va":5416576832,"va":5370128378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46447731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e873bcc402","size":5,"soft_fail":false,"target_va":5416576144,"va":5370128408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13857133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e86d71d300","size":5,"soft_fail":false,"target_va":5383984848,"va":5370127710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13857056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370126336,"raw_hex":"e82071d300","size":5,"soft_fail":false,"target_va":5383985360,"va":5370128299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*56033724(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370126336,"raw_hex":"ff15bc015703","size":6,"soft_fail":false,"va":5370127446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":39064,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":36492,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":39248,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":36828,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":36968,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":39392,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":36548,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":39092,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":36576,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":36408,"parent_record_offset":36224,"ancestor_record_offset":36224,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":36224,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":36224,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":36224,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37808,"parent_record_offset":37752,"ancestor_record_offset":36224,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":36224,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":36224,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37728,"parent_record_offset":37704,"ancestor_record_offset":36224,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":37412,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":37244,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":30,"record_offset":37524,"parent_record_offset":37440,"ancestor_record_offset":36224,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":30,"record_offset":39284,"parent_record_offset":39248,"ancestor_record_offset":36224,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":37048,"parent_record_offset":37012,"ancestor_record_offset":36224,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":36224,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":39352,"parent_record_offset":39316,"ancestor_record_offset":36224,"inlinee":7523,"source_line":2297,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":37656,"parent_record_offset":37620,"ancestor_record_offset":36224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37620,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37944,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37584,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":40644,"parent_record_offset":40580,"ancestor_record_offset":36224,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":39156,"parent_record_offset":39120,"ancestor_record_offset":36224,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":36432,"parent_record_offset":36408,"ancestor_record_offset":36224,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":36224,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":39180,"parent_record_offset":39156,"ancestor_record_offset":36224,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":36456,"parent_record_offset":36432,"ancestor_record_offset":36224,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":40552,"parent_record_offset":40524,"ancestor_record_offset":36224,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":36224,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":37300,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":36224,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":36648,"parent_record_offset":36624,"ancestor_record_offset":36224,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37160,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":39120,"parent_record_offset":39092,"ancestor_record_offset":36224,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":40580,"parent_record_offset":40552,"ancestor_record_offset":36224,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":39212,"parent_record_offset":39120,"ancestor_record_offset":36224,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":40616,"parent_record_offset":40580,"ancestor_record_offset":36224,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37440,"parent_record_offset":37084,"ancestor_record_offset":36224,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":36224,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37552,"parent_record_offset":37440,"ancestor_record_offset":36224,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":36224,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":37492,"parent_record_offset":37464,"ancestor_record_offset":36224,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":30,"record_offset":39316,"parent_record_offset":39284,"ancestor_record_offset":36224,"inlinee":7521,"source_line":106,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":37012,"parent_record_offset":36968,"ancestor_record_offset":36224,"inlinee":7529,"source_line":159,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":37084,"parent_record_offset":37048,"ancestor_record_offset":36224,"inlinee":7531,"source_line":160,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D69
// 首条位置: {"module":30,"inlinee":7529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":37048,"parent_record_offset":37012,"ancestor_record_offset":37012,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":37012,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":37012,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":37012,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37808,"parent_record_offset":37752,"ancestor_record_offset":37012,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":37012,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":37012,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37728,"parent_record_offset":37704,"ancestor_record_offset":37012,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":37412,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":37244,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":30,"record_offset":37524,"parent_record_offset":37440,"ancestor_record_offset":37012,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":37268,"parent_record_offset":37244,"ancestor_record_offset":37012,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":37656,"parent_record_offset":37620,"ancestor_record_offset":37012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37620,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37944,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37584,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":37184,"parent_record_offset":37160,"ancestor_record_offset":37012,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":37208,"parent_record_offset":37184,"ancestor_record_offset":37012,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37348,"parent_record_offset":37324,"ancestor_record_offset":37012,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":37300,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":37324,"parent_record_offset":37300,"ancestor_record_offset":37012,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37372,"parent_record_offset":37348,"ancestor_record_offset":37012,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37160,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37440,"parent_record_offset":37084,"ancestor_record_offset":37012,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":37012,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37552,"parent_record_offset":37440,"ancestor_record_offset":37012,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":37012,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":37492,"parent_record_offset":37464,"ancestor_record_offset":37012,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":37084,"parent_record_offset":37048,"ancestor_record_offset":37012,"inlinee":7531,"source_line":160,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D6B
// 首条位置: {"module":30,"inlinee":7531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":37412,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":37244,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":38008,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37620,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37944,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":37584,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":37300,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37160,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37440,"parent_record_offset":37084,"ancestor_record_offset":37084,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":37084,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":37084,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":37084,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":37808,"parent_record_offset":37752,"ancestor_record_offset":37084,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37780,"parent_record_offset":37752,"ancestor_record_offset":37084,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":37084,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":37728,"parent_record_offset":37704,"ancestor_record_offset":37084,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":30,"record_offset":37524,"parent_record_offset":37440,"ancestor_record_offset":37084,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":37268,"parent_record_offset":37244,"ancestor_record_offset":37084,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":37656,"parent_record_offset":37620,"ancestor_record_offset":37084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":37184,"parent_record_offset":37160,"ancestor_record_offset":37084,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":37208,"parent_record_offset":37184,"ancestor_record_offset":37084,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37348,"parent_record_offset":37324,"ancestor_record_offset":37084,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":37324,"parent_record_offset":37300,"ancestor_record_offset":37084,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":37372,"parent_record_offset":37348,"ancestor_record_offset":37084,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":37084,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":37552,"parent_record_offset":37440,"ancestor_record_offset":37084,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":38104,"parent_record_offset":38080,"ancestor_record_offset":37084,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":30,"record_offset":37492,"parent_record_offset":37464,"ancestor_record_offset":37084,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":24924,"rva":1410688,"va":5370119808}
// 编译实例 RVAs: 0x158680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t14489181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e85d16dd00","size":5,"soft_fail":false,"target_va":5384609408,"va":5370120222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t5557995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8ebce5400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370120704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=3
//     target_aliases=51 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-819019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8b580f3ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5370124598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-781742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e85212f4ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370120585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-781620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8cc12f4ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5370125119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009bbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-774660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8fc2df4ff","size":5,"soft_fail":false,"target_va":5369346992,"va":5370121647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b9330
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-655795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e84dfef5ff","size":5,"soft_fail":false,"target_va":5369467696,"va":5370123486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bac40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-649612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e87416f6ff","size":5,"soft_fail":false,"target_va":5369474112,"va":5370123719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140133780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-154527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e861a4fdff","size":5,"soft_fail":false,"target_va":5369968512,"va":5370123034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140133a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-153102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8f2a9fdff","size":5,"soft_fail":false,"target_va":5369969216,"va":5370122313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015c7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e89e3d0000","size":5,"soft_fail":false,"target_va":5370136512,"va":5370120733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403c4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2535703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e817b12600","size":5,"soft_fail":false,"target_va":5372659312,"va":5370123604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc2f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4863458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8e2354a00","size":5,"soft_fail":false,"target_va":5374984944,"va":5370121481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d714c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46235077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8c57dc102","size":5,"soft_fail":false,"target_va":5416359104,"va":5370124022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46935968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8a02fcc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370121083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ops::function::FnOnce::call_once::hfb9f07077c19180e (.llvm.16356646813640853151)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t950551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e817810e00","size":5,"soft_fail":false,"target_va":5371073200,"va":5370122644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8a5eaffff","size":5,"soft_fail":false,"target_va":5370118624,"va":5370124086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<sentry_types::protocol::v7::Context> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e843e9ffff","size":5,"soft_fail":false,"target_va":5370118992,"va":5370124808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14710134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e87675e000","size":5,"soft_fail":false,"target_va":5384830080,"va":5370119941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46452887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e897d0c402","size":5,"soft_fail":false,"target_va":5416576832,"va":5370123940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46452576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e860cfc402","size":5,"soft_fail":false,"target_va":5416576144,"va":5370123563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13863415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8f789d300","size":5,"soft_fail":false,"target_va":5383984848,"va":5370121428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13861824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8c083d300","size":5,"soft_fail":false,"target_va":5383985360,"va":5370123531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_core::hub_impl::_$LT$impl$u20$sentry_core..hub..Hub$GT$::is_active_and_usage_safe
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5945183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e85fb75a00","size":5,"soft_fail":false,"target_va":5376067888,"va":5370122700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14631630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e8ce42df00","size":5,"soft_fail":false,"target_va":5384751936,"va":5370120301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14602085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e865cfde00","size":5,"soft_fail":false,"target_va":5384722736,"va":5370120646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46495315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370119808,"raw_hex":"e85376c502","size":5,"soft_fail":false,"target_va":5416619360,"va":5370124040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::on_client_message
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":25060,"parent_record_offset":24924,"ancestor_record_offset":24924,"inlinee":7516,"source_line":45,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":24924,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":24924,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":24924,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":25580,"parent_record_offset":25524,"ancestor_record_offset":24924,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":27292,"parent_record_offset":27264,"ancestor_record_offset":24924,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25552,"parent_record_offset":25524,"ancestor_record_offset":24924,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25524,"parent_record_offset":25500,"ancestor_record_offset":24924,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":24924,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":25420,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":7517,"source_line":560,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":27992,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":30,"record_offset":28200,"parent_record_offset":28176,"ancestor_record_offset":24924,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":25796,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":30,"record_offset":28264,"parent_record_offset":28232,"ancestor_record_offset":24924,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":28232,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":24924,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":28328,"parent_record_offset":28292,"ancestor_record_offset":24924,"inlinee":7523,"source_line":2297,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":30,"record_offset":30360,"parent_record_offset":30320,"ancestor_record_offset":24924,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":25452,"parent_record_offset":25420,"ancestor_record_offset":24924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":24924,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":32396,"parent_record_offset":32332,"ancestor_record_offset":24924,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":28084,"parent_record_offset":28048,"ancestor_record_offset":24924,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":24924,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":24924,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":28108,"parent_record_offset":28084,"ancestor_record_offset":24924,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":25760,"parent_record_offset":25736,"ancestor_record_offset":24924,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":28020,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":28176,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":24924,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":24924,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":25852,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":24924,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":24924,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":25712,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":30,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":24924,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30464,"parent_record_offset":30436,"ancestor_record_offset":24924,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30552,"parent_record_offset":30520,"ancestor_record_offset":24924,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30492,"parent_record_offset":30464,"ancestor_record_offset":24924,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30520,"parent_record_offset":30492,"ancestor_record_offset":24924,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":30,"record_offset":30436,"parent_record_offset":30388,"ancestor_record_offset":24924,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":28048,"parent_record_offset":28020,"ancestor_record_offset":24924,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":24924,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":28140,"parent_record_offset":28048,"ancestor_record_offset":24924,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":32368,"parent_record_offset":32332,"ancestor_record_offset":24924,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":30,"record_offset":29368,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":7525,"source_line":151,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":30,"record_offset":30844,"parent_record_offset":30820,"ancestor_record_offset":24924,"inlinee":6543,"source_line":156,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":30,"record_offset":29416,"parent_record_offset":29392,"ancestor_record_offset":24924,"inlinee":7527,"source_line":145,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":30,"record_offset":29392,"parent_record_offset":29368,"ancestor_record_offset":24924,"inlinee":7526,"source_line":51,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::impl$0::with_active::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":30,"record_offset":30680,"parent_record_offset":30388,"ancestor_record_offset":24924,"inlinee":6541,"source_line":57,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub_impl::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":30,"record_offset":30388,"parent_record_offset":30320,"ancestor_record_offset":24924,"inlinee":6521,"source_line":149,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":26104,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":26552,"parent_record_offset":25060,"ancestor_record_offset":24924,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":30,"record_offset":30320,"parent_record_offset":30280,"ancestor_record_offset":24924,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":30,"record_offset":30280,"parent_record_offset":30244,"ancestor_record_offset":24924,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":30,"record_offset":28292,"parent_record_offset":28264,"ancestor_record_offset":24924,"inlinee":7521,"source_line":106,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_client_connected
// 记录形式: compiled_procedure; instances=1
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":36016,"rva":1417200,"va":5370126320}
// 编译实例 RVAs: 0x159ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":36164,"parent_record_offset":36016,"ancestor_record_offset":36016,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":36188,"parent_record_offset":36164,"ancestor_record_offset":36016,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 记录形式: compiled_procedure; instances=1
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":43452,"rva":1065216,"va":5369774336}
// 编译实例 RVAs: 0x104100
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
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":2260,"rva":1402336,"va":5370111456}
// 编译实例 RVAs: 0x1565e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14591075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e863a4de00","size":5,"soft_fail":false,"target_va":5384703584,"va":5370112504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14591428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8c4a5de00","size":5,"soft_fail":false,"target_va":5384703904,"va":5370112471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14681574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8e605e000","size":5,"soft_fail":false,"target_va":5384793264,"va":5370111685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14497217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8c135dd00","size":5,"soft_fail":false,"target_va":5384609408,"va":5370112186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidumper..ipc..windows..Socket$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6893555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8f32f6900","size":5,"soft_fail":false,"target_va":5377005376,"va":5370111816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5567156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8b4f25400","size":5,"soft_fail":false,"target_va":5375678688,"va":5370111527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5566841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e879f15400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370111858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1389325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8f3cceaff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370111512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-772911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8d134f4ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370111754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-742496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8a0abf4ff","size":5,"soft_fail":false,"target_va":5369370176,"va":5370112667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400adec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-692543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8c16ef5ff","size":5,"soft_fail":false,"target_va":5369421504,"va":5370114042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b8670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-650003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8ed14f6ff","size":5,"soft_fail":false,"target_va":5369464432,"va":5370114430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f2a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1688405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e855c31900","size":5,"soft_fail":false,"target_va":5371800176,"va":5370111766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3017863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8870c2e00","size":5,"soft_fail":false,"target_va":5373130976,"va":5370113108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140437910
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3017878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8960c2e00","size":5,"soft_fail":false,"target_va":5373131024,"va":5370113141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46944117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8754fcc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370112934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t46507475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8d3a5c502","size":5,"soft_fail":false,"target_va":5416620769,"va":5370113289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46508106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e84aa8c502","size":5,"soft_fail":false,"target_va":5416621536,"va":5370113425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::boxed::Box<tiny_glade::crash_pickup::server::Handler,alloc::alloc::Global> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8df140000","size":5,"soft_fail":false,"target_va":5370119568,"va":5370114220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46508236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8cca8c502","size":5,"soft_fail":false,"target_va":5416621632,"va":5370113391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::server::Server::run
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     evidence={"assembly":"\tcallq\t6880939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8abfe6800","size":5,"soft_fail":false,"target_va":5376993552,"va":5370112608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::windows::Socket::set_nonblocking
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     evidence={"assembly":"\tcallq\t6893064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e8082e6900","size":5,"soft_fail":false,"target_va":5377004864,"va":5370111795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14611076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e884f2de00","size":5,"soft_fail":false,"target_va":5384722736,"va":5370111655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3252768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370111456,"raw_hex":"e820a23100","size":5,"soft_fail":false,"target_va":5373365168,"va":5370112395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*56048718(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370111456,"raw_hex":"ff154e3c5703","size":6,"soft_fail":false,"va":5370112452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":6996,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7494,"source_line":2616,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":2436,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":6828,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":30,"record_offset":7288,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":8456,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":7644,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":7676,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":4132,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":6708,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":8052,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":7232,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7496,"source_line":241,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":6884,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":6744,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::server::Server::with_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/server.rs
//     evidence={"module":30,"record_offset":2988,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":7481,"source_line":83,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":2260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":2532,"parent_record_offset":2500,"ancestor_record_offset":2260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2260,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":5172,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":2472,"parent_record_offset":2436,"ancestor_record_offset":2260,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$6::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":2652,"parent_record_offset":2436,"ancestor_record_offset":2260,"inlinee":7479,"source_line":1212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":2260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":2260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":5980,"parent_record_offset":5944,"ancestor_record_offset":2260,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::is_unique
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":7020,"parent_record_offset":6996,"ancestor_record_offset":2260,"inlinee":7495,"source_line":2754,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":4972,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":2260,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":30,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":2260,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":4804,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":30,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":2260,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":30,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":2260,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":2260,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":3164,"parent_record_offset":2988,"ancestor_record_offset":2260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":30,"record_offset":6012,"parent_record_offset":5980,"ancestor_record_offset":2260,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":5500,"parent_record_offset":5372,"ancestor_record_offset":2260,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":6336,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":3192,"parent_record_offset":2988,"ancestor_record_offset":2260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":7056,"parent_record_offset":7020,"ancestor_record_offset":2260,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":2260,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5820,"parent_record_offset":5784,"ancestor_record_offset":2260,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":2260,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":7168,"parent_record_offset":7020,"ancestor_record_offset":2260,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":2260,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":2260,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":2260,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":2260,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":2260,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5876,"parent_record_offset":5784,"ancestor_record_offset":2260,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":30,"record_offset":6304,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":7493,"source_line":244,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":2260,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":4860,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":2260,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":2260,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4720,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixListener::set_nonblocking
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/windows.rs
//     evidence={"module":30,"record_offset":3220,"parent_record_offset":2988,"ancestor_record_offset":2260,"inlinee":7483,"source_line":412,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":30,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2260,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":3052,"parent_record_offset":2988,"ancestor_record_offset":2260,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":2260,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":3080,"parent_record_offset":2988,"ancestor_record_offset":2260,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":30,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2260,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":2260,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":2260,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":30,"record_offset":5372,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":7489,"source_line":125,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":5144,"parent_record_offset":4620,"ancestor_record_offset":2260,"inlinee":7488,"source_line":33,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":4620,"parent_record_offset":4132,"ancestor_record_offset":2260,"inlinee":7486,"source_line":212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D3E
// 首条位置: {"module":30,"inlinee":7486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":5172,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":4972,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":4804,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":5544,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":6336,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":30,"record_offset":6304,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":7493,"source_line":244,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":4860,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4720,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::thread::functions::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":30,"record_offset":5372,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":7489,"source_line":125,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":30,"record_offset":5144,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":7488,"source_line":33,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":4620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":4620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":4620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":4620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":4620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":5216,"parent_record_offset":5172,"ancestor_record_offset":4620,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":4620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":4620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":5980,"parent_record_offset":5944,"ancestor_record_offset":4620,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4620,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4620,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":5572,"parent_record_offset":5544,"ancestor_record_offset":4620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":30,"record_offset":6012,"parent_record_offset":5980,"ancestor_record_offset":4620,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":5500,"parent_record_offset":5372,"ancestor_record_offset":4620,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":5460,"parent_record_offset":5432,"ancestor_record_offset":4620,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":4620,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5820,"parent_record_offset":5784,"ancestor_record_offset":4620,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4620,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4620,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4620,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":4620,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":5876,"parent_record_offset":5784,"ancestor_record_offset":4620,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":4620,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":4620,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4620,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":4620,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":4620,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":4620,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42DE
// 首条位置: {"module":230,"inlinee":17118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":230,"record_offset":7688,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":7548,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":7856,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":7744,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":7604,"parent_record_offset":7516,"ancestor_record_offset":7516,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":7516,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":7516,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":7516,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":7516,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":7516,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":7516,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":7516,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00230.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::server::Handler>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "minidumper::ServerHandler::message_alloc<tiny_glade::crash_pickup::server::Handler>", "constructor": "minidumper::ServerHandler::message_alloc", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<tiny_glade::crash_pickup::server::Handler,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["tiny_glade::crash_pickup::server::Handler", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<tiny_glade::crash_pickup::server::Handler,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<tiny_glade::crash_pickup::server::Handler,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::Handler::new
// 信息 ID: 9e7b90f94dbc96ddcebcd984
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=30, inline_record=5144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::Handler::on_client_message
// 信息 ID: c1c5d0e3f4c04e5c35385b9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=30, inline_record=25060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 信息 ID: 5fc3c2b0b2e9b11dc528396e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >; module=117, inline_record=30652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$1
// 信息 ID: 12ac442a75587fba2f75adce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >; module=117, inline_record=29328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$2
// 信息 ID: e80da8d3b4ac519f492c1854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- core::ptr::drop_in_place<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- core::ptr::drop_in_place<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] sentry_core::hub::Hub::configure_scope<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >; module=117, inline_record=28308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::create_minidump_file
// 信息 ID: 217d179ff534492ac60e86c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_client_connected
// 信息 ID: a0c43996d66544c369ba088e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 信息 ID: a7a8f8fa0aa8b7857a20d702
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_message
// 信息 ID: e5edaee9240d315ec7e76f38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 信息 ID: 6edaf61e9123f5a4115fda31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 信息 ID: 9645b00994d6dd7e7f282fac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=30, inline_record=37012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 信息 ID: fb8ca2ea5d1106dfa0e7c076
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=30, inline_record=37084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run
// 信息 ID: 71abdbab0f0e462ba06052af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x6420d4 -> 0x1565e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run::closure$0
// 信息 ID: 7666e1aa79567f50eeec0780
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=30, inline_record=4620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run::closure$0::closure$0
// 信息 ID: bce1616b39772e5f597acd48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::run", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::run::closure$0::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::crash_pickup::server::run::closure$0::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::run::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::crash_pickup::server::run::closure$0::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::run::closure$0::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::server::run::closure$0::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::server::run::closure$0::closure_env$0,tuple$<> >; module=230, inline_record=7516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
