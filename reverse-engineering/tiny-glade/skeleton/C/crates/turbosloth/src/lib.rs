// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/turbosloth/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d7ee33fdded482d024bcfc17259cf8f3b23d78c2d877e4b686c2e1d9c4d462ac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1653, 1777, 1846, 3250, 3305, 3342, 3343, 3347, 3348, 3351, 3627, 4408, 4454, 4481, 4939
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bincode; // namespace candidate
//   package identities: [{"name":"bincode","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.3.3"}]
// use concurrent_queue; // namespace candidate
//   package identities: [{"name":"concurrent-queue","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use redb; // namespace candidate
//   package identities: [{"name":"redb","source":"registry+https://github.com/rust-lang/crates.io-index","version":"4.1.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// use tracing; // namespace candidate
//   package identities: [{"name":"tracing","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.40"}]
// use tracing_core; // namespace candidate
//   package identities: [{"name":"tracing-core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.1.32"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"tracing","source":"PDB_registry_path_only","version":"0.1.40"},{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: concurrent_queue::bounded::Bounded::push_or_else<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4408,"record_offset":264,"rva":44306064,"va":5413015184}
// 编译实例 RVAs: 0x2a40e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0b4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32725951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413015184,"raw_hex":"e841a40cfe","size":5,"soft_fail":false,"target_va":5380289760,"va":5413015706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3606093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413015184,"raw_hex":"e84d063700","size":5,"soft_fail":false,"target_va":5416621762,"va":5413015664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::yield_now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28287349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413015184,"raw_hex":"e88b5e50fe","size":5,"soft_fail":false,"target_va":5384728176,"va":5413015520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] concurrent_queue::bounded::impl$0::push::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"concurrent-queue","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/concurrent-queue-2.5.0/src/bounded.rs
//     evidence={"module":4408,"record_offset":1096,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5357,"source_line":86,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] concurrent_queue::full_fence
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"concurrent-queue","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/concurrent-queue-2.5.0/src/lib.rs
//     evidence={"module":4408,"record_offset":1036,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5354,"source_line":632,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] concurrent_queue::sync::busy_wait
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"concurrent-queue","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/concurrent-queue-2.5.0/src/sync.rs
//     evidence={"module":4408,"record_offset":1212,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5359,"source_line":49,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] concurrent_queue::sync::prelude::impl$0::with_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"concurrent-queue","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/concurrent-queue-2.5.0/src/sync.rs
//     evidence={"module":4408,"record_offset":1296,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5151,"source_line":76,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4408,"record_offset":924,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":868,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":812,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":1408,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] concurrent_queue::bounded::impl$0::push_or_else::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"concurrent-queue","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/concurrent-queue-2.5.0/src/bounded.rs
//     evidence={"module":4408,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":264,"inlinee":5361,"source_line":192,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4408,"record_offset":1064,"parent_record_offset":1036,"ancestor_record_offset":264,"inlinee":5355,"source_line":2346,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4408,"record_offset":1180,"parent_record_offset":1096,"ancestor_record_offset":264,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4408,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":264,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4408,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":264,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":264,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4408,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":264,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/04408.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1846,"record_offset":12388,"rva":30068864,"va":5398777984}
// 编译实例 RVAs: 0x1cad080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::alloc_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13994080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398777984,"raw_hex":"e8a0772aff","size":5,"soft_fail":false,"target_va":5384784192,"va":5398778267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <hashbrown::raw::Fallibility>::capacity_overflow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13994056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398777984,"raw_hex":"e8b8772aff","size":5,"soft_fail":false,"target_va":5384784144,"va":5398778195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23099558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398777984,"raw_hex":"e85a879ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5398778241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30056041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398777984,"raw_hex":"e8976135fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5398778228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18278763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398777984,"raw_hex":"e86be91601","size":5,"soft_fail":false,"target_va":5417057088,"va":5398778320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1846,"record_offset":13224,"parent_record_offset":12388,"ancestor_record_offset":12388,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::control::tag::TagSliceExt::fill_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1846,"record_offset":13252,"parent_record_offset":12388,"ancestor_record_offset":12388,"inlinee":12598,"source_line":73,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::RawTableInner::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1846,"record_offset":12756,"parent_record_offset":12388,"ancestor_record_offset":12388,"inlinee":12563,"source_line":1596,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_direct] hashbrown::raw::capacity_to_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1846,"record_offset":12560,"parent_record_offset":12388,"ancestor_record_offset":12388,"inlinee":12594,"source_line":105,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1846,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":12388,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1846,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":12388,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1846,"record_offset":13116,"parent_record_offset":13088,"ancestor_record_offset":12388,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1846,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":12388,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1846,"record_offset":12656,"parent_record_offset":12596,"ancestor_record_offset":12388,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1846,"record_offset":12956,"parent_record_offset":12808,"ancestor_record_offset":12388,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1846,"record_offset":12596,"parent_record_offset":12560,"ancestor_record_offset":12388,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1846,"record_offset":12688,"parent_record_offset":12560,"ancestor_record_offset":12388,"inlinee":12595,"source_line":3806,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1846,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":12388,"inlinee":12596,"source_line":3775,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1846,"record_offset":12624,"parent_record_offset":12596,"ancestor_record_offset":12388,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1846,"record_offset":13164,"parent_record_offset":12756,"ancestor_record_offset":12388,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1846,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":12388,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1846,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":12388,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1846,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":12388,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1846,"record_offset":12808,"parent_record_offset":12756,"ancestor_record_offset":12388,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::alloc::inner::do_alloc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\alloc.rs
//     evidence={"module":1846,"record_offset":13016,"parent_record_offset":12756,"ancestor_record_offset":12388,"inlinee":12568,"source_line":18,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1846,"record_offset":13192,"parent_record_offset":12756,"ancestor_record_offset":12388,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/01846.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::btree_mutator::MutateHelper::insert<ref$<str$>,enum2$<redb::tree_store::table_tree_base::InternalTableDefinition> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4481,"record_offset":51868,"rva":44511600,"va":5413220720}
// 编译实例 RVAs: 0x2a73170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$redb..tree_store..table_tree_base..InternalTableDefinition$u20$as$u20$redb..types..Value$GT$::as_bytes
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3564315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e81b633600","size":5,"soft_fail":false,"target_va":5416785296,"va":5413220976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-37542457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8c725c3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413221156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140996470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34461377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e83f29f2fd","size":5,"soft_fail":false,"target_va":5378761840,"va":5413223212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099dcc0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34429754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8c6a4f2fd","size":5,"soft_fail":false,"target_va":5378792640,"va":5413222389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9a110
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16619720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8386702ff","size":5,"soft_fail":false,"target_va":5396603152,"va":5413222867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3835272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e888853a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413221779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-37273893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8db3ec7fd","size":5,"soft_fail":false,"target_va":5375949040,"va":5413222928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16721983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8c1d700ff","size":5,"soft_fail":false,"target_va":5396500272,"va":5413222250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16722200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8e8d600ff","size":5,"soft_fail":false,"target_va":5396499840,"va":5413222035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_child
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16725794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8dec800ff","size":5,"soft_fail":false,"target_va":5396496304,"va":5413222093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16719520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e860e100ff","size":5,"soft_fail":false,"target_va":5396502608,"va":5413222123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16725892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e87cc800ff","size":5,"soft_fail":false,"target_va":5396495632,"va":5413221519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::offset_of_first_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16726014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e802c800ff","size":5,"soft_fail":false,"target_va":5396495520,"va":5413221529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16735855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e891a100ff","size":5,"soft_fail":false,"target_va":5396485536,"va":5413221386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16736063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8c1a000ff","size":5,"soft_fail":false,"target_va":5396485200,"va":5413221258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16735877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e87ba100ff","size":5,"soft_fail":false,"target_va":5396485424,"va":5413221296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,enum2$<redb::tree_store::table_tree_base::InternalTableDefinition> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-47621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e8fb45ffff","size":5,"soft_fail":false,"target_va":5413173440,"va":5413221056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12983280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413220720,"raw_hex":"e810e439ff","size":5,"soft_fail":false,"target_va":5400237584,"va":5413220859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":52112,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":53748,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":52292,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":52196,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":52264,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::get_page_number
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":52896,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":48414,"source_line":262,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":52756,"parent_record_offset":51868,"ancestor_record_offset":51868,"inlinee":81948,"source_line":258,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":52444,"parent_record_offset":52420,"ancestor_record_offset":51868,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":52468,"parent_record_offset":52444,"ancestor_record_offset":51868,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":52492,"parent_record_offset":52468,"ancestor_record_offset":51868,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":52420,"parent_record_offset":52364,"ancestor_record_offset":51868,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":52392,"parent_record_offset":52364,"ancestor_record_offset":51868,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":52364,"parent_record_offset":52340,"ancestor_record_offset":51868,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":52340,"parent_record_offset":52316,"ancestor_record_offset":51868,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":51868,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":52804,"parent_record_offset":52780,"ancestor_record_offset":51868,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":54620,"parent_record_offset":54596,"ancestor_record_offset":51868,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":55816,"parent_record_offset":55784,"ancestor_record_offset":51868,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":52316,"parent_record_offset":52292,"ancestor_record_offset":51868,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4481,"record_offset":52160,"parent_record_offset":52136,"ancestor_record_offset":51868,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":54656,"parent_record_offset":54620,"ancestor_record_offset":51868,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":54680,"parent_record_offset":54656,"ancestor_record_offset":51868,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":54712,"parent_record_offset":54620,"ancestor_record_offset":51868,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::cached_file::WritablePage::mem
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/cached_file.rs
//     evidence={"module":4481,"record_offset":52780,"parent_record_offset":52756,"ancestor_record_offset":51868,"inlinee":81949,"source_line":25,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::btree_mutator::MutateHelper::insert<ref$<str$>,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4481,"record_offset":47128,"rva":44506784,"va":5413215904}
// 编译实例 RVAs: 0x2a71ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-37538675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e88d34c3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413217374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140996470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34456257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e83f3df2fd","size":5,"soft_fail":false,"target_va":5378761840,"va":5413218092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099dcc0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34424708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e87cb8f2fd","size":5,"soft_fail":false,"target_va":5378792640,"va":5413217343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b800
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16608808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e8d89102ff","size":5,"soft_fail":false,"target_va":5396609024,"va":5413217827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3840288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e820993a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413216763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16716937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e877eb00ff","size":5,"soft_fail":false,"target_va":5396500272,"va":5413217204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16717154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e89eea00ff","size":5,"soft_fail":false,"target_va":5396499840,"va":5413216989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_child
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16720748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e894dc00ff","size":5,"soft_fail":false,"target_va":5396496304,"va":5413217047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16714474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e816f500ff","size":5,"soft_fail":false,"target_va":5396502608,"va":5413217077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16720971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e8b5db00ff","size":5,"soft_fail":false,"target_va":5396495632,"va":5413216598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::offset_of_first_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16721093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e83bdb00ff","size":5,"soft_fail":false,"target_va":5396495520,"va":5413216608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16730925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e8d3b400ff","size":5,"soft_fail":false,"target_va":5396485536,"va":5413216456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16731134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e802b400ff","size":5,"soft_fail":false,"target_va":5396485200,"va":5413216329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16730947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e8bdb400ff","size":5,"soft_fail":false,"target_va":5396485424,"va":5413216366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e86e8fffff","size":5,"soft_fail":false,"target_va":5413187376,"va":5413216189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12978469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413215904,"raw_hex":"e8dbf639ff","size":5,"soft_fail":false,"target_va":5400237584,"va":5413216048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":47308,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":47864,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":48036,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":47392,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":47836,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::get_page_number
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":47808,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":48414,"source_line":262,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":47668,"parent_record_offset":47128,"ancestor_record_offset":47128,"inlinee":81948,"source_line":258,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":48216,"parent_record_offset":48192,"ancestor_record_offset":47128,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":48240,"parent_record_offset":48216,"ancestor_record_offset":47128,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":48264,"parent_record_offset":48240,"ancestor_record_offset":47128,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":48192,"parent_record_offset":48136,"ancestor_record_offset":47128,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":48164,"parent_record_offset":48136,"ancestor_record_offset":47128,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":48136,"parent_record_offset":48112,"ancestor_record_offset":47128,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":48112,"parent_record_offset":48088,"ancestor_record_offset":47128,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":47332,"parent_record_offset":47308,"ancestor_record_offset":47128,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":47716,"parent_record_offset":47692,"ancestor_record_offset":47128,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":49212,"parent_record_offset":49180,"ancestor_record_offset":47128,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":48088,"parent_record_offset":48064,"ancestor_record_offset":47128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4481,"record_offset":47356,"parent_record_offset":47332,"ancestor_record_offset":47128,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::cached_file::WritablePage::mem
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/cached_file.rs
//     evidence={"module":4481,"record_offset":47692,"parent_record_offset":47668,"ancestor_record_offset":47128,"inlinee":81949,"source_line":25,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::btree_mutator::MutateHelper::insert<u128,ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4481,"record_offset":49408,"rva":44509184,"va":5413218304}
// 编译实例 RVAs: 0x2a72800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-37541082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8262bc3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413219781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140996470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34458673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8cf33f2fd","size":5,"soft_fail":false,"target_va":5378761840,"va":5413220508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099dcc0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34427115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e815aff2fd","size":5,"soft_fail":false,"target_va":5378792640,"va":5413219750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9a110
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16617087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8817102ff","size":5,"soft_fail":false,"target_va":5396603152,"va":5413220234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3837890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8c28f3a00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413219161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16719344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e810e200ff","size":5,"soft_fail":false,"target_va":5396500272,"va":5413219611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16719561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e837e100ff","size":5,"soft_fail":false,"target_va":5396499840,"va":5413219396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_child
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16723155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e82dd300ff","size":5,"soft_fail":false,"target_va":5396496304,"va":5413219454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16716881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8afeb00ff","size":5,"soft_fail":false,"target_va":5396502608,"va":5413219484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16723375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e851d200ff","size":5,"soft_fail":false,"target_va":5396495632,"va":5413219002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::offset_of_first_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16723497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8d7d100ff","size":5,"soft_fail":false,"target_va":5396495520,"va":5413219012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16733326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e872ab00ff","size":5,"soft_fail":false,"target_va":5396485536,"va":5413218857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16733536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8a0aa00ff","size":5,"soft_fail":false,"target_va":5396485200,"va":5413218731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16733348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e85cab00ff","size":5,"soft_fail":false,"target_va":5396485424,"va":5413218767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_mutator::MutateHelper::insert_helper<u128,ref$<slice2$<u8> > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e8adbcffff","size":5,"soft_fail":false,"target_va":5413201360,"va":5413218590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12980869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413218304,"raw_hex":"e87bed39ff","size":5,"soft_fail":false,"target_va":5400237584,"va":5413218448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":49600,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":50324,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":50496,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":49684,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":50296,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::get_page_number
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":50268,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":48414,"source_line":262,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":50128,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":81948,"source_line":258,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::types::impl$40::as_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":4481,"record_offset":49712,"parent_record_offset":49408,"ancestor_record_offset":49408,"inlinee":81956,"source_line":625,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":50676,"parent_record_offset":50652,"ancestor_record_offset":49408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":50700,"parent_record_offset":50676,"ancestor_record_offset":49408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":50724,"parent_record_offset":50700,"ancestor_record_offset":49408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":50652,"parent_record_offset":50596,"ancestor_record_offset":49408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":50624,"parent_record_offset":50596,"ancestor_record_offset":49408,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":50596,"parent_record_offset":50572,"ancestor_record_offset":49408,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":49408,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":49624,"parent_record_offset":49600,"ancestor_record_offset":49408,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":50176,"parent_record_offset":50152,"ancestor_record_offset":49408,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_le_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":49736,"parent_record_offset":49712,"ancestor_record_offset":49408,"inlinee":81957,"source_line":3888,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":49760,"parent_record_offset":49736,"ancestor_record_offset":49408,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":51672,"parent_record_offset":51640,"ancestor_record_offset":49408,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":49408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4481,"record_offset":49648,"parent_record_offset":49624,"ancestor_record_offset":49408,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::cached_file::WritablePage::mem
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/cached_file.rs
//     evidence={"module":4481,"record_offset":50152,"parent_record_offset":50128,"ancestor_record_offset":49408,"inlinee":81949,"source_line":25,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4481,"record_offset":15712,"rva":44478256,"va":5413187376}
// 编译实例 RVAs: 0x2a6af30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$str$u20$as$u20$redb..types..Key$GT$::compare
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14707856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e870931fff","size":5,"soft_fail":false,"target_va":5398480320,"va":5413188171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-37513499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8e596c3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5413192182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-37510565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e85ba2c3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413189264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44469977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8277159fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5413192164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=4
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43950745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8675d61fd","size":5,"soft_fail":false,"target_va":5369240448,"va":5413191188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43934255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d19d61fd","size":5,"soft_fail":false,"target_va":5369265120,"va":5413199370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400faa90
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43455212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e814ed68fd","size":5,"soft_fail":false,"target_va":5369735824,"va":5413191031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140996470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34438879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e82181f2fd","size":5,"soft_fail":false,"target_va":5378761840,"va":5413200714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099dcc0
//     owner=unresolved_namespace; count=17
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34396585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e85726f3fd","size":5,"soft_fail":false,"target_va":5378792640,"va":5413189220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b800
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16587376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e890e502ff","size":5,"soft_fail":false,"target_va":5396609024,"va":5413196395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16589896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8b8db02ff","size":5,"soft_fail":false,"target_va":5396609392,"va":5413199283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3868781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e86d083b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413188270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3365375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8ff593300","size":5,"soft_fail":false,"target_va":5416557632,"va":5413192252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t3431113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8c95a3400","size":5,"soft_fail":false,"target_va":5416620794,"va":5413189676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t-37239733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e84bc4c7fd","size":5,"soft_fail":false,"target_va":5375949040,"va":5413188768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t3423375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e88f3c3400","size":5,"soft_fail":false,"target_va":5416621600,"va":5413198220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3423932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8bc3e3400","size":5,"soft_fail":false,"target_va":5416622208,"va":5413198271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3422887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8a73a3400","size":5,"soft_fail":false,"target_va":5416621762,"va":5413198870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3422859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e88b3a3400","size":5,"soft_fail":false,"target_va":5416621632,"va":5413198768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t3424001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8013f3400","size":5,"soft_fail":false,"target_va":5416622320,"va":5413198314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchAccessor$LT$T$GT$::child_for_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4367617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8ff5abdff","size":5,"soft_fail":false,"target_va":5408819952,"va":5413187564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchAccessor$LT$T$GT$::key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4372542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8c247bdff","size":5,"soft_fail":false,"target_va":5408820352,"va":5413192889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16693586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8ae4601ff","size":5,"soft_fail":false,"target_va":5396500272,"va":5413193853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::build_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16697113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8e73801ff","size":5,"soft_fail":false,"target_va":5396496416,"va":5413193524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16691416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8284f01ff","size":5,"soft_fail":false,"target_va":5396499840,"va":5413191251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_child
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16695140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e89c4001ff","size":5,"soft_fail":false,"target_va":5396496304,"va":5413191439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::push_key
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16690162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e80e5401ff","size":5,"soft_fail":false,"target_va":5396502608,"va":5413192765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchBuilder::should_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16693620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e88c4601ff","size":5,"soft_fail":false,"target_va":5396499776,"va":5413193391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchMutator::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16686033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e82f6401ff","size":5,"soft_fail":false,"target_va":5396502896,"va":5413188924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::BranchMutator::write_child_page
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16686327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8096301ff","size":5,"soft_fail":false,"target_va":5396502688,"va":5413189010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16693901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8734501ff","size":5,"soft_fail":false,"target_va":5396495728,"va":5413189624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::key_unchecked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16693487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8114701ff","size":5,"soft_fail":false,"target_va":5396494656,"va":5413188138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::last_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16698496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8803301ff","size":5,"soft_fail":false,"target_va":5396493456,"va":5413191947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::new
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16692419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e83d4b01ff","size":5,"soft_fail":false,"target_va":5396495632,"va":5413188046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::offset_of_first_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16696336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8f03b01ff","size":5,"soft_fail":false,"target_va":5396495520,"va":5413191851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::offset_of_value
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16696930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e89e3901ff","size":5,"soft_fail":false,"target_va":5396495504,"va":5413192429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::total_length
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16694828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d44101ff","size":5,"soft_fail":false,"target_va":5396494480,"va":5413189303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafAccessor::value_range
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16697280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8403801ff","size":5,"soft_fail":false,"target_va":5396493536,"va":5413190811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::build
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16705118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8a21901ff","size":5,"soft_fail":false,"target_va":5396485536,"va":5413190649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::build_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16708491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8750c01ff","size":5,"soft_fail":false,"target_va":5396481824,"va":5413190310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16704693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e84b1b01ff","size":5,"soft_fail":false,"target_va":5396485200,"va":5413189888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::push
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16704568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8c81b01ff","size":5,"soft_fail":false,"target_va":5396485424,"va":5413189987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafBuilder::should_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16705325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d31801ff","size":5,"soft_fail":false,"target_va":5396484864,"va":5413190184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafMutator::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16708733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8830b01ff","size":5,"soft_fail":false,"target_va":5396488176,"va":5413196904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafMutator::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16708680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8b80b01ff","size":5,"soft_fail":false,"target_va":5396488064,"va":5413196739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafMutator::replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16704542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8e21b01ff","size":5,"soft_fail":false,"target_va":5396492240,"va":5413196777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_base::LeafMutator::sufficient_replace_inplace_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4370349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e85350bdff","size":5,"soft_fail":false,"target_va":5408819216,"va":5413189560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_mutator::MutateHelper::conditional_free<ref$<str$>,enum2$<redb::tree_store::table_tree_base::InternalTableDefinition> >
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d35a0000","size":5,"soft_fail":false,"target_va":5413215392,"va":5413192136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d8fdffff","size":5,"soft_fail":false,"target_va":5413187376,"va":5413187923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::base::PageMut::memory_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16719217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e88fe200ff","size":5,"soft_fail":false,"target_va":5396469696,"va":5413188908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::base::PageNumber::from_le_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16721704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8d8d800ff","size":5,"soft_fail":false,"target_va":5396469648,"va":5413191347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::free
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12960764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8043c3aff","size":5,"soft_fail":false,"target_va":5400230336,"va":5413191095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12950206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e842653aff","size":5,"soft_fail":false,"target_va":5400237584,"va":5413187785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12984167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e899e039ff","size":5,"soft_fail":false,"target_va":5400204640,"va":5413188802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::get_page_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12982977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e83fe539ff","size":5,"soft_fail":false,"target_va":5400206368,"va":5413189340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::page_store::page_manager::TransactionalMemory::uncommitted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12985864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413187376,"raw_hex":"e8f8d939ff","size":5,"soft_fail":false,"target_va":5400202816,"va":5413188675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4481,"record_offset":18832,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":16528,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":21988,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":22468,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4481,"record_offset":18612,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4481,"record_offset":16964,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":16788,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":17976,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":17268,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":19936,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":16612,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":21500,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::AccessGuard::with_arc_page
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":20036,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81947,"source_line":185,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::AccessGuard::with_page
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":22208,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81954,"source_line":175,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::AccessGuardMutInPlace::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":20936,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81951,"source_line":379,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::BranchAccessor::child_checksum
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":16300,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48426,"source_line":1366,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::BranchAccessor::child_page
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":20208,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48425,"source_line":1379,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::BranchAccessor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":16120,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48422,"source_line":1266,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::EntryAccessor::key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":20964,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81953,"source_line":415,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::LeafAccessor::position
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":16680,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81938,"source_line":459,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_mutator::impl$0::insert_helper::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_mutator.rs
//     evidence={"module":4481,"record_offset":17784,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81942,"source_line":300,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::PageImpl::to_arc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":19508,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81945,"source_line":187,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::get_page_number
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":17240,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48414,"source_line":262,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$10::memory
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":20796,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":81948,"source_line":258,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$18::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":16852,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48411,"source_line":32,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$7::get_page_number
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":16880,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48447,"source_line":216,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::page_store::base::impl$7::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":15980,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":48420,"source_line":212,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":4481,"record_offset":19980,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":12499,"source_line":735,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":18356,"parent_record_offset":18332,"ancestor_record_offset":15712,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":15712,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":17496,"parent_record_offset":17472,"ancestor_record_offset":15712,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":17520,"parent_record_offset":17496,"ancestor_record_offset":15712,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":18404,"parent_record_offset":18380,"ancestor_record_offset":15712,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":17544,"parent_record_offset":17520,"ancestor_record_offset":15712,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":18332,"parent_record_offset":18180,"ancestor_record_offset":15712,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4481,"record_offset":17472,"parent_record_offset":17416,"ancestor_record_offset":15712,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":22084,"parent_record_offset":22052,"ancestor_record_offset":15712,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":18116,"parent_record_offset":18088,"ancestor_record_offset":15712,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":17444,"parent_record_offset":17416,"ancestor_record_offset":15712,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":17416,"parent_record_offset":17392,"ancestor_record_offset":15712,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":22132,"parent_record_offset":22108,"ancestor_record_offset":15712,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":22108,"parent_record_offset":22084,"ancestor_record_offset":15712,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":18180,"parent_record_offset":18144,"ancestor_record_offset":15712,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":18144,"parent_record_offset":18116,"ancestor_record_offset":15712,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":15712,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4481,"record_offset":18220,"parent_record_offset":18180,"ancestor_record_offset":15712,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4481,"record_offset":18028,"parent_record_offset":17976,"ancestor_record_offset":15712,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4481,"record_offset":18056,"parent_record_offset":18028,"ancestor_record_offset":15712,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_inner_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":19656,"parent_record_offset":19532,"ancestor_record_offset":15712,"inlinee":10497,"source_line":309,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":16052,"parent_record_offset":16028,"ancestor_record_offset":15712,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":15712,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":15712,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":17036,"parent_record_offset":17012,"ancestor_record_offset":15712,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4481,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":15712,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":22052,"parent_record_offset":22012,"ancestor_record_offset":15712,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":15712,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":15712,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":22012,"parent_record_offset":21988,"ancestor_record_offset":15712,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4481,"record_offset":18088,"parent_record_offset":18056,"ancestor_record_offset":15712,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4481,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":15712,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":4481,"record_offset":16236,"parent_record_offset":16156,"ancestor_record_offset":15712,"inlinee":9708,"source_line":258,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4481,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":15712,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4481,"record_offset":18668,"parent_record_offset":18640,"ancestor_record_offset":15712,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4481,"record_offset":16440,"parent_record_offset":16300,"ancestor_record_offset":15712,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4481,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":15712,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4481,"record_offset":18696,"parent_record_offset":18640,"ancestor_record_offset":15712,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4481,"record_offset":18640,"parent_record_offset":18612,"ancestor_record_offset":15712,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4481,"record_offset":20084,"parent_record_offset":20060,"ancestor_record_offset":15712,"inlinee":10947,"source_line":861,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":4481,"record_offset":20060,"parent_record_offset":20036,"ancestor_record_offset":15712,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":15712,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":16404,"parent_record_offset":16372,"ancestor_record_offset":15712,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::midpoint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":4481,"record_offset":16756,"parent_record_offset":16680,"ancestor_record_offset":15712,"inlinee":81940,"source_line":213,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":18720,"parent_record_offset":18696,"ancestor_record_offset":15712,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4481,"record_offset":29392,"parent_record_offset":29360,"ancestor_record_offset":15712,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4481,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":15712,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4481,"record_offset":18460,"parent_record_offset":18056,"ancestor_record_offset":15712,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":18484,"parent_record_offset":18460,"ancestor_record_offset":15712,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":15712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4481,"record_offset":16076,"parent_record_offset":16052,"ancestor_record_offset":15712,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4481,"record_offset":22172,"parent_record_offset":22012,"ancestor_record_offset":15712,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4481,"record_offset":16260,"parent_record_offset":16236,"ancestor_record_offset":15712,"inlinee":9709,"source_line":1711,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4481,"record_offset":16156,"parent_record_offset":16120,"ancestor_record_offset":15712,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4481,"record_offset":16200,"parent_record_offset":16156,"ancestor_record_offset":15712,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":19624,"parent_record_offset":19564,"ancestor_record_offset":15712,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":17132,"parent_record_offset":17072,"ancestor_record_offset":15712,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4481,"record_offset":17164,"parent_record_offset":17036,"ancestor_record_offset":15712,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::btree_base::LeafAccessor::num_pairs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":16728,"parent_record_offset":16680,"ancestor_record_offset":15712,"inlinee":81939,"source_line":555,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::btree_base::LeafMutator::sufficient_insert_inplace_space
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":4481,"record_offset":17816,"parent_record_offset":17784,"ancestor_record_offset":15712,"inlinee":81943,"source_line":975,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::base::impl$7::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":4481,"record_offset":17852,"parent_record_offset":17816,"ancestor_record_offset":15712,"inlinee":48420,"source_line":212,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::cached_file::WritablePage::mem
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/cached_file.rs
//     evidence={"module":4481,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":15712,"inlinee":81949,"source_line":25,"archive":"_pdb/C/modules/04481.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::table_tree::TableTree::get_table<ref$<str$>,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4454,"record_offset":8888,"rva":44452464,"va":5413161584}
// 编译实例 RVAs: 0x2a64a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-37483570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413161584,"raw_hex":"e8ce0bc4fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413162269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1b80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32256500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413161584,"raw_hex":"e80cce13fe","size":5,"soft_fail":false,"target_va":5380905856,"va":5413162351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142ced130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2655411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413161584,"raw_hex":"e8b3842800","size":5,"soft_fail":false,"target_va":5415817520,"va":5413162104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3894908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413161584,"raw_hex":"e87c6e3b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413162143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree::TableTree::get_table_untyped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16516546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413161584,"raw_hex":"e83efa03ff","size":5,"soft_fail":false,"target_va":5396645104,"va":5413161645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4454,"record_offset":9172,"parent_record_offset":8888,"ancestor_record_offset":8888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4454,"record_offset":9048,"parent_record_offset":8888,"ancestor_record_offset":8888,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4454,"record_offset":9080,"parent_record_offset":8888,"ancestor_record_offset":8888,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":8888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":8888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":8888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":9372,"parent_record_offset":9316,"ancestor_record_offset":8888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":8888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":9316,"parent_record_offset":9292,"ancestor_record_offset":8888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":9292,"parent_record_offset":9268,"ancestor_record_offset":8888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4454,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":8888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::table_tree::TableTree::get_table<u128,ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4454,"record_offset":7940,"rva":44451664,"va":5413160784}
// 编译实例 RVAs: 0x2a64750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-37482770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413160784,"raw_hex":"e8ee0ec4fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413161469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba1b80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32255700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413160784,"raw_hex":"e82cd113fe","size":5,"soft_fail":false,"target_va":5380905856,"va":5413161551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32103549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413160784,"raw_hex":"e8832316fe","size":5,"soft_fail":false,"target_va":5381057760,"va":5413161304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3895708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413160784,"raw_hex":"e89c713b00","size":5,"soft_fail":false,"target_va":5417057056,"va":5413161343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree::TableTree::get_table_untyped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-16515746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413160784,"raw_hex":"e85efd03ff","size":5,"soft_fail":false,"target_va":5396645104,"va":5413160845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4454,"record_offset":8232,"parent_record_offset":7940,"ancestor_record_offset":7940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4454,"record_offset":8108,"parent_record_offset":7940,"ancestor_record_offset":7940,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4454,"record_offset":8140,"parent_record_offset":7940,"ancestor_record_offset":7940,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":7940,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":7940,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":7940,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4454,"record_offset":8432,"parent_record_offset":8376,"ancestor_record_offset":7940,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":8404,"parent_record_offset":8376,"ancestor_record_offset":7940,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":8376,"parent_record_offset":8352,"ancestor_record_offset":7940,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4454,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":7940,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4454,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":7940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::table_tree_base::InternalTableDefinition::check_match<ref$<str$>,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4939,"record_offset":264,"rva":47108400,"va":5415817520}
// 编译实例 RVAs: 0x2ced130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$str$u20$as$u20$redb..types..Value$GT$::type_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17337115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e8e574f7fe","size":5,"soft_fail":false,"target_va":5398480512,"va":5415817622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$redb..types..Value$GT$::type_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17337730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e87e72f7fe","size":5,"soft_fail":false,"target_va":5398480016,"va":5415817741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-40139250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e80e869bfd","size":5,"soft_fail":false,"target_va":5375678688,"va":5415817933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-40138997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e80b879bfd","size":5,"soft_fail":false,"target_va":5375678704,"va":5415817696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47095727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e8516031fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5415817914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1239366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e846e91200","size":5,"soft_fail":false,"target_va":5417057040,"va":5415817669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1239095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e837e81200","size":5,"soft_fail":false,"target_va":5417057056,"va":5415817956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t802934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e876400c00","size":5,"soft_fail":false,"target_va":5416620794,"va":5415817855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::check_match_untyped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17342017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e8bf61f7fe","size":5,"soft_fail":false,"target_va":5398475568,"va":5415817580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::private_key_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17342145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e83f61f7fe","size":5,"soft_fail":false,"target_va":5398475472,"va":5415817612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::private_value_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17342216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415817520,"raw_hex":"e8f860f7fe","size":5,"soft_fail":false,"target_va":5398475520,"va":5415817731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":2528,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4939,"record_offset":500,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":1904,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4939,"record_offset":464,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_fixed_key_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":3976,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":36998,"source_line":284,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_fixed_value_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":4004,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":37000,"source_line":291,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_key_alignment
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":3948,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":37001,"source_line":302,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_value_alignment
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":4068,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":36999,"source_line":309,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":3052,"parent_record_offset":2896,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":1752,"parent_record_offset":1688,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":264,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":1720,"parent_record_offset":1688,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":264,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":264,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":2896,"parent_record_offset":2856,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":2940,"parent_record_offset":2896,"ancestor_record_offset":264,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":2688,"parent_record_offset":2656,"ancestor_record_offset":264,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":264,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":2752,"parent_record_offset":2720,"ancestor_record_offset":264,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":2656,"parent_record_offset":2624,"ancestor_record_offset":264,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4939,"record_offset":2624,"parent_record_offset":2592,"ancestor_record_offset":264,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":2560,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":2592,"parent_record_offset":2560,"ancestor_record_offset":264,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":580,"parent_record_offset":536,"ancestor_record_offset":264,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":724,"parent_record_offset":696,"ancestor_record_offset":264,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":264,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":2792,"parent_record_offset":2752,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":620,"parent_record_offset":600,"ancestor_record_offset":264,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":668,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":4939,"record_offset":600,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4939,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":264,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4939,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":264,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4939,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":264,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4939,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":264,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4939,"record_offset":3180,"parent_record_offset":2752,"ancestor_record_offset":264,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":1592,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4939,"record_offset":1012,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] redb::types::impl$23::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":4939,"record_offset":556,"parent_record_offset":536,"ancestor_record_offset":264,"inlinee":36996,"source_line":10,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] redb::types::impl$28::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":4939,"record_offset":536,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":36994,"source_line":38,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: redb::tree_store::table_tree_base::InternalTableDefinition::check_match<u128,ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4939,"record_offset":5320,"rva":12348640,"va":5381057760}
// 编译实例 RVAs: 0xbc6ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$$u5b$u8$u5d$$u20$as$u20$redb..types..Value$GT$::type_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t17428238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e80eef0901","size":5,"soft_fail":false,"target_va":5398486224,"va":5381057981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u128$u20$as$u20$redb..types..Value$GT$::type_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t17422245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e8a5d70901","size":5,"soft_fail":false,"target_va":5398480112,"va":5381057862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5379490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e85eeaadff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381058173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-5379237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e85bebadff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381057936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12335967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e8a1c443ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381058154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35999126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e8964d2502","size":5,"soft_fail":false,"target_va":5417057040,"va":5381057909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35998855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e8874c2502","size":5,"soft_fail":false,"target_va":5417057056,"va":5381058196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35562694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e8c6a41e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381058095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::check_match_untyped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17417743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e80fc60901","size":5,"soft_fail":false,"target_va":5398475568,"va":5381057820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::private_key_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17417615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e88fc50901","size":5,"soft_fail":false,"target_va":5398475472,"va":5381057852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::table_tree_base::InternalTableDefinition::private_value_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17417544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381057760,"raw_hex":"e848c50901","size":5,"soft_fail":false,"target_va":5398475520,"va":5381057971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":7596,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4939,"record_offset":5568,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":6972,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4939,"record_offset":5532,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_fixed_key_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":8960,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":36998,"source_line":284,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_fixed_value_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":9108,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":37000,"source_line":291,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_key_alignment
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":9136,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":37001,"source_line":302,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::table_tree_base::InternalTableDefinition::private_get_value_alignment
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/table_tree_base.rs
//     evidence={"module":4939,"record_offset":9080,"parent_record_offset":5320,"ancestor_record_offset":5320,"inlinee":36999,"source_line":309,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":5320,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":5320,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":6848,"parent_record_offset":6820,"ancestor_record_offset":5320,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":6876,"parent_record_offset":6848,"ancestor_record_offset":5320,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":5320,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":5320,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":8120,"parent_record_offset":7964,"ancestor_record_offset":5320,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4939,"record_offset":6820,"parent_record_offset":6756,"ancestor_record_offset":5320,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5320,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":7892,"parent_record_offset":7860,"ancestor_record_offset":5320,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":5320,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":5320,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5320,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5320,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":7964,"parent_record_offset":7924,"ancestor_record_offset":5320,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":7924,"parent_record_offset":7892,"ancestor_record_offset":5320,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":5320,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4939,"record_offset":8008,"parent_record_offset":7964,"ancestor_record_offset":5320,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":7756,"parent_record_offset":7724,"ancestor_record_offset":5320,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":7788,"parent_record_offset":7756,"ancestor_record_offset":5320,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":5320,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4939,"record_offset":7724,"parent_record_offset":7692,"ancestor_record_offset":5320,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4939,"record_offset":7692,"parent_record_offset":7660,"ancestor_record_offset":5320,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":7628,"parent_record_offset":7596,"ancestor_record_offset":5320,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":5320,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":4939,"record_offset":5648,"parent_record_offset":5604,"ancestor_record_offset":5320,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":5792,"parent_record_offset":5764,"ancestor_record_offset":5320,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":5764,"parent_record_offset":5736,"ancestor_record_offset":5320,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":7860,"parent_record_offset":7820,"ancestor_record_offset":5320,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":5688,"parent_record_offset":5668,"ancestor_record_offset":5320,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4939,"record_offset":5736,"parent_record_offset":5688,"ancestor_record_offset":5320,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":4939,"record_offset":5668,"parent_record_offset":5648,"ancestor_record_offset":5320,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4939,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":5320,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4939,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":5320,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4939,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":5320,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4939,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":5320,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4939,"record_offset":8248,"parent_record_offset":7820,"ancestor_record_offset":5320,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":5320,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4939,"record_offset":6660,"parent_record_offset":5568,"ancestor_record_offset":5320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4939,"record_offset":6080,"parent_record_offset":5688,"ancestor_record_offset":5320,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] redb::types::impl$23::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":4939,"record_offset":5624,"parent_record_offset":5604,"ancestor_record_offset":5320,"inlinee":36996,"source_line":10,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   [pdb_inline_transitive] redb::types::impl$28::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":4939,"record_offset":5604,"parent_record_offset":5568,"ancestor_record_offset":5320,"inlinee":36994,"source_line":38,"archive":"_pdb/C/modules/04939.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::store::DiskStore::read_build_record
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1777,"record_offset":70892,"rva":28435280,"va":5397144400}
// 编译实例 RVAs: 0x1b1e350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$redb..db..Database$u20$as$u20$redb..db..ReadableDatabase$GT$::begin_read
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11480981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8952faf00","size":5,"soft_fail":false,"target_va":5408625440,"va":5397144454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-21465891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8dd74b8fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397144590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27806160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e830b657fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397145003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140993ef0
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18392331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8f55ae7fe","size":5,"soft_fail":false,"target_va":5378752240,"va":5397144566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9a110
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-542561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e89fb8f7ff","size":5,"soft_fail":false,"target_va":5396603152,"va":5397145708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-538449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8afc8f7ff","size":5,"soft_fail":false,"target_va":5396607472,"va":5397145916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b610
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-537921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8bfcaf7ff","size":5,"soft_fail":false,"target_va":5396608528,"va":5397146444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-536611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8ddcff7ff","size":5,"soft_fail":false,"target_va":5396608704,"va":5397145310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142640010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11671638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e85618b200","size":5,"soft_fail":false,"target_va":5408817168,"va":5397145525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19912420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8e4d62f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397144631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-13919326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8a29b2bff","size":5,"soft_fail":false,"target_va":5383225232,"va":5397144553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<redb::error::TableError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e814730000","size":5,"soft_fail":false,"target_va":5397174176,"va":5397144711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19476435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8d32f2901","size":5,"soft_fail":false,"target_va":5416622320,"va":5397145880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::transactions::ReadTransaction::open_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11483519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e87f39af00","size":5,"soft_fail":false,"target_va":5408628192,"va":5397144668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree::Btree$LT$K$C$V$GT$::get_helper::hdce77fec698af1a9 (.llvm.14191508046212726293)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11676891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397144400,"raw_hex":"e8db2cb200","size":5,"soft_fail":false,"target_va":5408821792,"va":5397144896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bincode::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/lib.rs
//     evidence={"module":1777,"record_offset":74180,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":29917,"source_line":177,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":72696,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":72100,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":73228,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":71048,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] redb::table::ReadOnlyTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/table.rs
//     evidence={"module":1777,"record_offset":72128,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":86200,"source_line":499,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] redb::tree_store::btree_base::AccessGuard::value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":1777,"record_offset":72728,"parent_record_offset":70892,"ancestor_record_offset":70892,"inlinee":84099,"source_line":226,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":71468,"parent_record_offset":71444,"ancestor_record_offset":70892,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":71492,"parent_record_offset":71468,"ancestor_record_offset":70892,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":71516,"parent_record_offset":71492,"ancestor_record_offset":70892,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":71444,"parent_record_offset":71420,"ancestor_record_offset":70892,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1777,"record_offset":71420,"parent_record_offset":71172,"ancestor_record_offset":70892,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":73004,"parent_record_offset":72980,"ancestor_record_offset":70892,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":71856,"parent_record_offset":71828,"ancestor_record_offset":70892,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":71828,"parent_record_offset":71804,"ancestor_record_offset":70892,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":73052,"parent_record_offset":73028,"ancestor_record_offset":70892,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":73028,"parent_record_offset":73004,"ancestor_record_offset":70892,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":71804,"parent_record_offset":71780,"ancestor_record_offset":70892,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1777,"record_offset":71288,"parent_record_offset":71252,"ancestor_record_offset":70892,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1777,"record_offset":72864,"parent_record_offset":72840,"ancestor_record_offset":70892,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1777,"record_offset":71252,"parent_record_offset":71228,"ancestor_record_offset":70892,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":72980,"parent_record_offset":72956,"ancestor_record_offset":70892,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":72956,"parent_record_offset":72764,"ancestor_record_offset":70892,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::Options::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/mod.rs
//     evidence={"module":1777,"record_offset":74212,"parent_record_offset":74180,"ancestor_record_offset":70892,"inlinee":29841,"source_line":199,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] bincode::de::Deserializer::with_bincode_read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/de/mod.rs
//     evidence={"module":1777,"record_offset":74312,"parent_record_offset":74276,"ancestor_record_offset":70892,"inlinee":29846,"source_line":67,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] bincode::internal::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/internal.rs
//     evidence={"module":1777,"record_offset":74244,"parent_record_offset":74212,"ancestor_record_offset":70892,"inlinee":29843,"source_line":101,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] bincode::internal::deserialize_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/internal.rs
//     evidence={"module":1777,"record_offset":74276,"parent_record_offset":74244,"ancestor_record_offset":70892,"inlinee":29844,"source_line":109,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_le_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1777,"record_offset":72204,"parent_record_offset":72180,"ancestor_record_offset":70892,"inlinee":81957,"source_line":3888,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1777,"record_offset":72228,"parent_record_offset":72204,"ancestor_record_offset":70892,"inlinee":28153,"source_line":3925,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1777,"record_offset":73132,"parent_record_offset":73096,"ancestor_record_offset":70892,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":71100,"parent_record_offset":71048,"ancestor_record_offset":70892,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1777,"record_offset":71320,"parent_record_offset":71288,"ancestor_record_offset":70892,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":74396,"parent_record_offset":74276,"ancestor_record_offset":70892,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1777,"record_offset":73160,"parent_record_offset":73096,"ancestor_record_offset":70892,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1777,"record_offset":73096,"parent_record_offset":72764,"ancestor_record_offset":70892,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":71348,"parent_record_offset":71288,"ancestor_record_offset":70892,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":71380,"parent_record_offset":71252,"ancestor_record_offset":70892,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::btree::Btree::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree.rs
//     evidence={"module":1777,"record_offset":72152,"parent_record_offset":72128,"ancestor_record_offset":70892,"inlinee":84423,"source_line":877,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::btree_base::EitherPage::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/btree_base.rs
//     evidence={"module":1777,"record_offset":72764,"parent_record_offset":72728,"ancestor_record_offset":70892,"inlinee":84062,"source_line":153,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::base::impl$10::memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/base.rs
//     evidence={"module":1777,"record_offset":72816,"parent_record_offset":72764,"ancestor_record_offset":70892,"inlinee":81948,"source_line":258,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] redb::tree_store::page_store::cached_file::WritablePage::mem
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/tree_store/page_store/cached_file.rs
//     evidence={"module":1777,"record_offset":72840,"parent_record_offset":72816,"ancestor_record_offset":70892,"inlinee":81949,"source_line":25,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] redb::types::impl$40::as_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/types.rs
//     evidence={"module":1777,"record_offset":72180,"parent_record_offset":72152,"ancestor_record_offset":70892,"inlinee":81956,"source_line":625,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1777,"record_offset":74340,"parent_record_offset":74276,"ancestor_record_offset":70892,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":74364,"parent_record_offset":74340,"ancestor_record_offset":70892,"inlinee":29848,"source_line":667,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::store::DiskStore::recipe_outcome_if_fresh
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1777,"record_offset":76180,"rva":28437344,"va":5397146464}
// 编译实例 RVAs: 0x1b1eb60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t19465644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8ac052901","size":5,"soft_fail":false,"target_va":5416616112,"va":5397150463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t19468810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e80a122901","size":5,"soft_fail":false,"target_va":5416616304,"va":5397147489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21470428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e82463b8fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397149127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27915592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8b80a56fe","size":5,"soft_fail":false,"target_va":5369240448,"va":5397156035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27894990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8325b56fe","size":5,"soft_fail":false,"target_va":5369265120,"va":5397160105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27810253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e833a657fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397149096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400faa90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27420119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8299a5dfe","size":5,"soft_fail":false,"target_va":5369735824,"va":5397155938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18418418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e80ef5e6fe","size":5,"soft_fail":false,"target_va":5378739728,"va":5397158141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b1e350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e834f6ffff","size":5,"soft_fail":false,"target_va":5397144400,"va":5397146903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142582880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10885068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8cc17a600","size":5,"soft_fail":false,"target_va":5408041088,"va":5397156015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425830d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10895631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e80f41a600","size":5,"soft_fail":false,"target_va":5408043216,"va":5397147580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19909525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e895cb2f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397147526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=55
//     evidence={"assembly":"\tcallq\t19463336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8a8fc2801","size":5,"soft_fail":false,"target_va":5416621536,"va":5397158195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19471083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8eb1a2901","size":5,"soft_fail":false,"target_va":5416621632,"va":5397150544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::logger
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12538055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e839af40ff","size":5,"soft_fail":false,"target_va":5384609312,"va":5397147362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19469218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8a2132901","size":5,"soft_fail":false,"target_va":5416619696,"va":5397150473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12422794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8767142ff","size":5,"soft_fail":false,"target_va":5384724912,"va":5397147701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing::__macro_support::__is_enabled
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"tracing","version":"0.1.40","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13939270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8ba4d2bff","size":5,"soft_fail":false,"target_va":5383207728,"va":5397146993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing::__macro_support::__tracing_log
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"tracing","version":"0.1.40","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13943319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8e93d2bff","size":5,"soft_fail":false,"target_va":5383207760,"va":5397151074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing_core::callsite::DefaultCallsite::register
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19405672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8681b2801","size":5,"soft_fail":false,"target_va":5416556304,"va":5397150627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tracing_core::event::Event::dispatch
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13935277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8535d2bff","size":5,"soft_fail":false,"target_va":5383212016,"va":5397147288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::registry::registered_version
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10891719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8c731a600","size":5,"soft_fail":false,"target_va":5408039488,"va":5397147764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::DiskStore::artifact_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e804e3ffff","size":5,"soft_fail":false,"target_va":5397140256,"va":5397147671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::DiskStore::dependency_source_files_match_recorded_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e89a3d0000","size":5,"soft_fail":false,"target_va":5397164064,"va":5397148289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::DiskStore::recipe_outcome_if_fresh
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e881e2ffff","size":5,"soft_fail":false,"target_va":5397146464,"va":5397154010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::DiskStore::rewrite_record_file_dependencies
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e808340000","size":5,"soft_fail":false,"target_va":5397161712,"va":5397148387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e85e2c0000","size":5,"soft_fail":false,"target_va":5397160832,"va":5397149469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e830330000","size":5,"soft_fail":false,"target_va":5397161360,"va":5397148251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8d9290000","size":5,"soft_fail":false,"target_va":5397160656,"va":5397149938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8ff2e0000","size":5,"soft_fail":false,"target_va":5397161008,"va":5397148972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$5
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e84a2b0000","size":5,"soft_fail":false,"target_va":5397161536,"va":5397150449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$6
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e85c130000","size":5,"soft_fail":false,"target_va":5397160304,"va":5397155343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$7
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e8b9160000","size":5,"soft_fail":false,"target_va":5397160480,"va":5397154658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397146464,"raw_hex":"e89c140000","size":5,"soft_fail":false,"target_va":5397161184,"va":5397155903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t*24(%r15)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397146464,"raw_hex":"41ff5718","size":4,"soft_fail":false,"va":5397147383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":89700,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":89868,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":84848,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":79360,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=59
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":78824,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":91040,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":77952,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":76880,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1777,"record_offset":89812,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] log::MetadataBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1777,"record_offset":84932,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5210,"source_line":1162,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1777,"record_offset":84764,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1777,"record_offset":76924,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1777,"record_offset":93076,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1777,"record_offset":79796,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1777,"record_offset":79852,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1777,"record_offset":76424,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1777,"record_offset":93020,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":12499,"source_line":735,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::callsite::DefaultCallsite::interest
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":1777,"record_offset":78476,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5579,"source_line":349,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::callsite::impl$1::metadata
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":1777,"record_offset":78600,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5552,"source_line":370,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::dispatcher::has_been_set
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/dispatcher.rs
//     evidence={"module":1777,"record_offset":84680,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5200,"source_line":345,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::FieldSet::iter
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":1777,"record_offset":78656,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5554,"source_line":870,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::FieldSet::value_set
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":1777,"record_offset":78992,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5559,"source_line":883,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::field::impl$51::next
//     owner=registry_dependency_definition; count=58
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":1777,"record_offset":78712,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5557,"source_line":971,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::metadata::LevelFilter::current
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":1777,"record_offset":78364,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5544,"source_line":697,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::metadata::impl$23::le
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/metadata.rs
//     evidence={"module":1777,"record_offset":78448,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5546,"source_line":957,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] tracing_core::subscriber::Interest::is_never
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/subscriber.rs
//     evidence={"module":1777,"record_offset":78568,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":5582,"source_line":637,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::impl$14::recipe_outcome_if_fresh::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tracing","version":"0.1.40","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.40/src/macros.rs
//     evidence={"module":1777,"record_offset":79020,"parent_record_offset":76180,"ancestor_record_offset":76180,"inlinee":86202,"source_line":872,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":81964,"parent_record_offset":81940,"ancestor_record_offset":76180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":81988,"parent_record_offset":81964,"ancestor_record_offset":76180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":82012,"parent_record_offset":81988,"ancestor_record_offset":76180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":81940,"parent_record_offset":81884,"ancestor_record_offset":76180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":81912,"parent_record_offset":81884,"ancestor_record_offset":76180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":81884,"parent_record_offset":81860,"ancestor_record_offset":76180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":81860,"parent_record_offset":81836,"ancestor_record_offset":76180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1777,"record_offset":93044,"parent_record_offset":93020,"ancestor_record_offset":76180,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":79216,"parent_record_offset":79020,"ancestor_record_offset":76180,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":78784,"parent_record_offset":78760,"ancestor_record_offset":76180,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1777,"record_offset":77712,"parent_record_offset":77688,"ancestor_record_offset":76180,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1777,"record_offset":77276,"parent_record_offset":77252,"ancestor_record_offset":76180,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1777,"record_offset":77360,"parent_record_offset":77336,"ancestor_record_offset":76180,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1777,"record_offset":77004,"parent_record_offset":76980,"ancestor_record_offset":76180,"inlinee":11777,"source_line":694,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1777,"record_offset":77028,"parent_record_offset":77004,"ancestor_record_offset":76180,"inlinee":11786,"source_line":942,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1777,"record_offset":77780,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1777,"record_offset":78760,"parent_record_offset":78736,"ancestor_record_offset":76180,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1777,"record_offset":78736,"parent_record_offset":78712,"ancestor_record_offset":76180,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1777,"record_offset":77512,"parent_record_offset":77484,"ancestor_record_offset":76180,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1777,"record_offset":77484,"parent_record_offset":77456,"ancestor_record_offset":76180,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":91064,"parent_record_offset":91040,"ancestor_record_offset":76180,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1777,"record_offset":79240,"parent_record_offset":79216,"ancestor_record_offset":76180,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":77300,"parent_record_offset":77276,"ancestor_record_offset":76180,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":79948,"parent_record_offset":79852,"ancestor_record_offset":76180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1777,"record_offset":89772,"parent_record_offset":89748,"ancestor_record_offset":76180,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1777,"record_offset":77628,"parent_record_offset":77604,"ancestor_record_offset":76180,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1777,"record_offset":89836,"parent_record_offset":89812,"ancestor_record_offset":76180,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":79908,"parent_record_offset":79880,"ancestor_record_offset":76180,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1777,"record_offset":89724,"parent_record_offset":89700,"ancestor_record_offset":76180,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1777,"record_offset":89748,"parent_record_offset":89724,"ancestor_record_offset":76180,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":76800,"parent_record_offset":76772,"ancestor_record_offset":76180,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":78204,"parent_record_offset":78016,"ancestor_record_offset":76180,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":76492,"parent_record_offset":76460,"ancestor_record_offset":76180,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":78508,"parent_record_offset":78476,"ancestor_record_offset":76180,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":78296,"parent_record_offset":78264,"ancestor_record_offset":76180,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":76704,"parent_record_offset":76672,"ancestor_record_offset":76180,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":76516,"parent_record_offset":76492,"ancestor_record_offset":76180,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":76728,"parent_record_offset":76704,"ancestor_record_offset":76180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":78228,"parent_record_offset":78204,"ancestor_record_offset":76180,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1777,"record_offset":78320,"parent_record_offset":78296,"ancestor_record_offset":76180,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1777,"record_offset":77420,"parent_record_offset":77392,"ancestor_record_offset":76180,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1777,"record_offset":77456,"parent_record_offset":77420,"ancestor_record_offset":76180,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1777,"record_offset":77392,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1777,"record_offset":77252,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1777,"record_offset":77808,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1777,"record_offset":77336,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1777,"record_offset":77196,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1777,"record_offset":77688,"parent_record_offset":77664,"ancestor_record_offset":76180,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1777,"record_offset":76948,"parent_record_offset":76924,"ancestor_record_offset":76180,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1777,"record_offset":77664,"parent_record_offset":77556,"ancestor_record_offset":76180,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1777,"record_offset":76980,"parent_record_offset":76948,"ancestor_record_offset":76180,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77604,"parent_record_offset":77580,"ancestor_record_offset":76180,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77900,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77580,"parent_record_offset":77556,"ancestor_record_offset":76180,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77116,"parent_record_offset":77092,"ancestor_record_offset":76180,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77092,"parent_record_offset":76948,"ancestor_record_offset":76180,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77140,"parent_record_offset":77116,"ancestor_record_offset":76180,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77224,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1777,"record_offset":77556,"parent_record_offset":77140,"ancestor_record_offset":76180,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1777,"record_offset":79820,"parent_record_offset":79796,"ancestor_record_offset":76180,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] log::MetadataBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1777,"record_offset":79300,"parent_record_offset":79020,"ancestor_record_offset":76180,"inlinee":5210,"source_line":1162,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$58::clone
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1777,"record_offset":79324,"parent_record_offset":79300,"ancestor_record_offset":76180,"inlinee":5212,"source_line":1082,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1777,"record_offset":79132,"parent_record_offset":79020,"ancestor_record_offset":76180,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1777,"record_offset":79880,"parent_record_offset":79852,"ancestor_record_offset":76180,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1777,"record_offset":76672,"parent_record_offset":76644,"ancestor_record_offset":76180,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1777,"record_offset":76644,"parent_record_offset":76616,"ancestor_record_offset":76180,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1777,"record_offset":78016,"parent_record_offset":77980,"ancestor_record_offset":76180,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1777,"record_offset":76772,"parent_record_offset":76580,"ancestor_record_offset":76180,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1777,"record_offset":76580,"parent_record_offset":76552,"ancestor_record_offset":76180,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1777,"record_offset":76552,"parent_record_offset":76424,"ancestor_record_offset":76180,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1777,"record_offset":77980,"parent_record_offset":77952,"ancestor_record_offset":76180,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1777,"record_offset":76460,"parent_record_offset":76424,"ancestor_record_offset":76180,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1777,"record_offset":78264,"parent_record_offset":77980,"ancestor_record_offset":76180,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":1777,"record_offset":76616,"parent_record_offset":76580,"ancestor_record_offset":76180,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::callsite::impl$1::metadata
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/callsite.rs
//     evidence={"module":1777,"record_offset":79272,"parent_record_offset":79020,"ancestor_record_offset":76180,"inlinee":5552,"source_line":370,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::dispatcher::has_been_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/dispatcher.rs
//     evidence={"module":1777,"record_offset":79048,"parent_record_offset":79020,"ancestor_record_offset":76180,"inlinee":5200,"source_line":345,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] tracing_core::field::FieldSet::len
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"tracing-core","version":"0.1.32","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.32/src/field.rs
//     evidence={"module":1777,"record_offset":78680,"parent_record_offset":78656,"ancestor_record_offset":76180,"inlinee":5555,"source_line":895,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$1::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":77052,"parent_record_offset":77028,"ancestor_record_offset":76180,"inlinee":28151,"source_line":78,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":77736,"parent_record_offset":77712,"ancestor_record_offset":76180,"inlinee":38734,"source_line":50,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":89892,"parent_record_offset":89868,"ancestor_record_offset":76180,"inlinee":86204,"source_line":100,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::store::DiskStore::rewrite_record_file_dependencies
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1777,"record_offset":100508,"rva":28452592,"va":5397161712}
// 编译实例 RVAs: 0x1b226f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::io::error::Error>::_new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12500967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8194041ff","size":5,"soft_fail":false,"target_va":5384661376,"va":5397162338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::io::error::Error>::new::<&str>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12536615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8d9b440ff","size":5,"soft_fail":false,"target_va":5384625232,"va":5397161842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21483606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8aa2fb8fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397162289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-21483220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e82c31b8fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397161919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28440087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8e9094efe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397162274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990e10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18423384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8a8e1e6fe","size":5,"soft_fail":false,"target_va":5378739728,"va":5397163107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099d3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18373630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e802a4e7fe","size":5,"soft_fail":false,"target_va":5378790368,"va":5397163993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6de0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16040278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8aa3e0bff","size":5,"soft_fail":false,"target_va":5381123552,"va":5397163825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6e20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16040006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8ba3f0bff","size":5,"soft_fail":false,"target_va":5381123616,"va":5397163617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9a110
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-560093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e82374f7ff","size":5,"soft_fail":false,"target_va":5396603152,"va":5397163240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b260
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-555478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e82a86f7ff","size":5,"soft_fail":false,"target_va":5396607584,"va":5397163057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a9b500
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-554796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8d488f7ff","size":5,"soft_fail":false,"target_va":5396608256,"va":5397163047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b1e350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e810bcffff","size":5,"soft_fail":false,"target_va":5397144400,"va":5397161787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14263ef30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11649462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8b6c1b100","size":5,"soft_fail":false,"target_va":5408812848,"va":5397163381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142642f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11667192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e8f806b200","size":5,"soft_fail":false,"target_va":5408829296,"va":5397162099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19895105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e841932f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397161946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19457309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e81de52801","size":5,"soft_fail":false,"target_va":5416620769,"va":5397163455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$redb..error..TableError$GT$::h37676c2f87e7f5c4 (.llvm.7797686884345738163)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11645987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e823b4b100","size":5,"soft_fail":false,"target_va":5408809680,"va":5397163688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::db::Database::begin_write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11443563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e86b9dae00","size":5,"soft_fail":false,"target_va":5408605792,"va":5397162224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::transactions::WriteTransaction::commit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-562017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e89f6cf7ff","size":5,"soft_fail":false,"target_va":5396601296,"va":5397163308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::transactions::WriteTransaction::open_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11468439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e897feae00","size":5,"soft_fail":false,"target_va":5408630896,"va":5397162452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] redb::tree_store::btree::BtreeMut$LT$K$C$V$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11662601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e809f5b100","size":5,"soft_fail":false,"target_va":5408825520,"va":5397162914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::error::Error::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11640474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397161712,"raw_hex":"e89a9eb100","size":5,"soft_fail":false,"target_va":5408802608,"va":5397162129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bincode::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/lib.rs
//     evidence={"module":1777,"record_offset":102240,"parent_record_offset":100508,"ancestor_record_offset":100508,"inlinee":29918,"source_line":106,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":100668,"parent_record_offset":100508,"ancestor_record_offset":100508,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1777,"record_offset":102296,"parent_record_offset":100508,"ancestor_record_offset":100508,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_direct] redb::table::Table::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"redb","version":"4.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/redb-4.1.0/src/table.rs
//     evidence={"module":1777,"record_offset":103256,"parent_record_offset":100508,"ancestor_record_offset":100508,"inlinee":86122,"source_line":202,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":102688,"parent_record_offset":102664,"ancestor_record_offset":100508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":102712,"parent_record_offset":102688,"ancestor_record_offset":100508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":101056,"parent_record_offset":101032,"ancestor_record_offset":100508,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":101080,"parent_record_offset":101056,"ancestor_record_offset":100508,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":102736,"parent_record_offset":102712,"ancestor_record_offset":100508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":101104,"parent_record_offset":101080,"ancestor_record_offset":100508,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":102664,"parent_record_offset":102632,"ancestor_record_offset":100508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1777,"record_offset":101032,"parent_record_offset":100968,"ancestor_record_offset":100508,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1777,"record_offset":102632,"parent_record_offset":102588,"ancestor_record_offset":100508,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":1777,"record_offset":102552,"parent_record_offset":102516,"ancestor_record_offset":100508,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1777,"record_offset":102588,"parent_record_offset":102552,"ancestor_record_offset":100508,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":101000,"parent_record_offset":100968,"ancestor_record_offset":100508,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":100968,"parent_record_offset":100940,"ancestor_record_offset":100508,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1777,"record_offset":100940,"parent_record_offset":100912,"ancestor_record_offset":100508,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1777,"record_offset":100696,"parent_record_offset":100668,"ancestor_record_offset":100508,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] bincode::config::Options::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bincode","version":"1.3.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-1.3.3/src/config/mod.rs
//     evidence={"module":1777,"record_offset":102264,"parent_record_offset":102240,"ancestor_record_offset":100508,"inlinee":29856,"source_line":178,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1777,"record_offset":102516,"parent_record_offset":102480,"ancestor_record_offset":100508,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1777,"record_offset":101292,"parent_record_offset":101264,"ancestor_record_offset":100508,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1777,"record_offset":102412,"parent_record_offset":102364,"ancestor_record_offset":100508,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1777,"record_offset":100720,"parent_record_offset":100696,"ancestor_record_offset":100508,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::other
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1777,"record_offset":102480,"parent_record_offset":102444,"ancestor_record_offset":100508,"inlinee":22598,"source_line":603,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$14::rewrite_record_file_dependencies::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":102332,"parent_record_offset":102296,"ancestor_record_offset":100508,"inlinee":86206,"source_line":1061,"archive":"_pdb/C/modules/01777.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::redb_io_error
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1777,"record_offset":102444,"parent_record_offset":102412,"ancestor_record_offset":100508,"inlinee":86120,"source_line":1433,"archive":"_pdb/C/modules/01777.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: concurrent_queue::bounded::Bounded::push_or_else<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >
// 信息 ID: f04105d0097951af2c94c5d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] dyn$ -> ["core::any::Any", "core::marker::Send", "core::marker::Sync"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["dyn$<core::any::Any,core::marker::Send,core::marker::Sync>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::result::Result -> ["alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>", "turbosloth::asset::AssetBuildError"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::any::Any", "core::marker::Send", "core::marker::Sync"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["dyn$<core::any::Any,core::marker::Send,core::marker::Sync>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::result::Result -> ["alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>", "turbosloth::asset::AssetBuildError"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>"]; 字段布局 = unknown。
// [类型关系] concurrent_queue::bounded::impl$0::push::closure_env$0 -> ["enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"]; 字段布局 = unknown。
// [类型关系] concurrent_queue::bounded::Bounded::push_or_else -> ["enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] async_channel::Sender::try_send<enum2$<core::result::Result<alloc::sync::Arc<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > @ 0x258255d -> 0x2a40e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: hashbrown::raw::RawTableInner::fallible_with_capacity<alloc::alloc::Global>
// 信息 ID: a8dc85559a31570700ffa4f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] hashbrown::raw::RawTableInner::fallible_with_capacity -> ["alloc::alloc::Global"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bincode::de::impl$3::deserialize_map<bincode::de::read::SliceReader,bincode::config::WithOtherLimit<bincode::config::WithOtherTrailing<bincode::config::WithOtherIntEncoding<bincode::config::DefaultOptions,bincode::config::int::FixintEncoding>,bincode::config::trailing::AllowTrailing>,bincode::config::limit::Infinite>,serde_core::de::impls::impl$204::deserialize::MapVisitor<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > @ 0x263fa74 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<core::any::TypeId,alloc::sync::Arc<turbosloth::store::SingleTypeValueCache,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<core::any::TypeId,alloc::sync::Arc<turbosloth::store::SingleTypeValueCache,alloc::alloc::Global>,std::hash::random::RandomState> > @ 0x2ddaea1 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::store::AssetRecipeId,alloc::sync::Weak<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::store::AssetRecipeId,alloc::sync::Weak<dyn$<core::any::Any,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > | hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::store::AssetRecipeId,alloc::sync::Weak<turbosloth::asset::AssetPayload,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::store::AssetRecipeId,alloc::sync::Weak<turbosloth::asset::AssetPayload,alloc::alloc::Global>,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > | hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::store::AssetRecipeId,turbosloth::store::AssetContentId>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::store::AssetRecipeId,turbosloth::store::AssetContentId,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > @ 0x2ddb357 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,ref$<turbosloth::export::AssetExport> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState> > @ 0x2ddb811 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<std::path::PathBuf,alloc::sync::Arc<std::sync::poison::mutex::Mutex<enum2$<turbosloth::store::SourceFileSlot> >,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<std::path::PathBuf,alloc::sync::Arc<std::sync::poison::mutex::Mutex<enum2$<turbosloth::store::SourceFileSlot> >,alloc::alloc::Global>,std::hash::random::RandomState> > @ 0x2ddbcdd -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,ref$<core::panic::location::Location> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,ref$<core::panic::location::Location>,std::hash::random::RandomState> > @ 0x2ddc2bd -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::store::AssetRecipeId,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::store::AssetRecipeId,tuple$<>,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > @ 0x2ddc867 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::raw::RawTable::reserve_rehash<tuple$<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<turbosloth::store::AssetRecipeId,turbosloth::store::PackagedFileState,core::hash::BuildHasherDefault<turbosloth::id::AssetIdHasher> > > @ 0x2ddccd7 -> 0x1cad080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::btree_mutator::MutateHelper::insert<ref$<str$>,enum2$<redb::tree_store::table_tree_base::InternalTableDefinition> >
// 信息 ID: eec472f836ac455d7814d263
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["redb::tree_store::table_tree_base::InternalTableDefinition"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::btree_mutator::MutateHelper::insert -> ["ref$<str$>", "enum2$<redb::tree_store::table_tree_base::InternalTableDefinition>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::btree::BtreeMut::insert<ref$<str$>,enum2$<redb::tree_store::table_tree_base::InternalTableDefinition> > @ 0x2641c7f -> 0x2a73170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::btree_mutator::MutateHelper::insert<ref$<str$>,u64>
// 信息 ID: 9a9d7297f1ad70a2967180a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::btree_mutator::MutateHelper::insert -> ["ref$<str$>", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::btree::BtreeMut::insert<ref$<str$>,u64> @ 0x264178f -> 0x2a71ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::btree_mutator::MutateHelper::insert<u128,ref$<slice2$<u8> > >
// 信息 ID: 0b9e23a3f90fea1309f3a0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::btree_mutator::MutateHelper::insert -> ["u128", "ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::btree::BtreeMut::insert<u128,ref$<slice2$<u8> > > @ 0x264216f -> 0x2a72800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,u64>
// 信息 ID: 7aea21226db94e274cba24ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::btree_mutator::MutateHelper::insert_helper -> ["ref$<str$>", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::btree_mutator::MutateHelper::insert_helper<ref$<str$>,u64> @ 0x2a6b153 -> 0x2a6af30 (direct_call); 同地址别名未裁定。
// [静态直接引用] redb::tree_store::btree_mutator::MutateHelper::insert<ref$<str$>,u64> @ 0x2a71fbd -> 0x2a6af30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::table_tree::TableTree::get_table<ref$<str$>,u64>
// 信息 ID: 7d2780c5711ae09d9443219b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::table_tree::TableTree::get_table -> ["ref$<str$>", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::transactions::ReadTransaction::open_table<ref$<str$>,u64> @ 0x261194e -> 0x2a64a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] redb::tree_store::table_tree::TableTreeMut::get_or_create_table<ref$<str$>,u64> @ 0x2a64136 -> 0x2a64a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::table_tree::TableTree::get_table<u128,ref$<slice2$<u8> > >
// 信息 ID: e5b5473ac78a5b3006282616
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::table_tree::TableTree::get_table -> ["u128", "ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::transactions::ReadTransaction::open_table<u128,ref$<slice2$<u8> > > @ 0x2611e1e -> 0x2a64750 (direct_call); 同地址别名未裁定。
// [静态直接引用] redb::tree_store::table_tree::TableTreeMut::get_or_create_table<u128,ref$<slice2$<u8> > > @ 0x2a63966 -> 0x2a64750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::table_tree_base::InternalTableDefinition::check_match<ref$<str$>,u64>
// 信息 ID: 2eb9d2b8cab54fde269be22a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::table_tree_base::InternalTableDefinition::check_match -> ["ref$<str$>", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::table_tree::TableTree::get_table<ref$<str$>,u64> @ 0x2a64c78 -> 0x2ced130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: redb::tree_store::table_tree_base::InternalTableDefinition::check_match<u128,ref$<slice2$<u8> > >
// 信息 ID: e9babb51f501ccee7bbaf23e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] redb::tree_store::table_tree_base::InternalTableDefinition::check_match -> ["u128", "ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] redb::tree_store::table_tree::TableTreeMut::get_or_create_table<u128,ref$<slice2$<u8> > > @ 0xba2130 -> 0xbc6ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] redb::tree_store::table_tree::TableTree::get_table<u128,ref$<slice2$<u8> > > @ 0x2a64958 -> 0xbc6ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::store::DiskStore::read_build_record
// 信息 ID: 19eb5694483123ea8c618a3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::PackagePass::install_closure @ 0x1b1ab26 -> 0x1b1e350 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::build_record @ 0x1b1d2cd -> 0x1b1e350 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::recipe_outcome_if_fresh @ 0x1b1ed17 -> 0x1b1e350 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::rewrite_record_file_dependencies @ 0x1b2273b -> 0x1b1e350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::store::DiskStore::recipe_outcome_if_fresh
// 信息 ID: 8194306c100f3da222075e2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::AssetStore::load<asset::texture::SourceImageAsset> @ 0x1b0ce0b -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::AssetStore::load<clutter_pack_pipe::ClutterMeshAsset> @ 0x1b0d50b -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<clutter_pack_pipe::ClutterMeshAsset> @ 0x1b165f9 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<asset::texture::blob::TextureAssetBlob> @ 0x1b16b86 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<clutter_pack_pipe::ClutterGlbImport> @ 0x1b170f6 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<asset::texture::SourceImageAsset> @ 0x1b17646 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<clutter_pack_pipe::ClutterObjectList> @ 0x1b17b86 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<turbosloth::asset::FileAsset> @ 0x1b180d6 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<asset_pipe::texture::TextureAssetList> @ 0x1b18606 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::try_load<clutter_pack_pipe::ClutterIconImage> @ 0x1b18b26 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::PackagePass::install_closure @ 0x1b1aac6 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::build_record @ 0x1b1d22c -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::artifact_bytes @ 0x1b1d576 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::build_dependency @ 0x1b1dc49 -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::recipe_outcome_if_fresh @ 0x1b208da -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::recipe_outcome_if_fresh @ 0x1b20bae -> 0x1b1eb60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::store::DiskStore::rewrite_record_file_dependencies
// 信息 ID: 3118ea8e024d80aa98bf532b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::store::DiskStore::recipe_outcome_if_fresh @ 0x1b1f2e3 -> 0x1b226f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
