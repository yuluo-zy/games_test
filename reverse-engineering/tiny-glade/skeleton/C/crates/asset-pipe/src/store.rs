// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset-pipe/src/store.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 7ed7a34cdb4de15789a2c7f3efda10f4c545d01919fc424726f0112be651ac94
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: store.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1680, 3255
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use async_lock; // namespace candidate
//   package identities: [{"name":"async-lock","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.2"}]
// use core; // namespace candidate
//   package identities: []
// use event_listener_strategy; // namespace candidate
//   package identities: [{"name":"event-listener-strategy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::store_path
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14332
// 首条位置: {"module":1680,"inlinee":82738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":816,"parent_record_offset":760,"ancestor_record_offset":760,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1680,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":760,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":760,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":760,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":760,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1012,"parent_record_offset":960,"ancestor_record_offset":760,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":760,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":760,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":936,"parent_record_offset":916,"ancestor_record_offset":760,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":836,"parent_record_offset":816,"ancestor_record_offset":760,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::impl$0::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":3140,"rva":7119936,"va":5375829056}
// 编译实例 RVAs: 0x6ca440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::locked_store
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":264,"rva":25193504,"va":5393902624}
// 编译实例 RVAs: 0x1806c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] asset_pipe::store::open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t79","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902624,"raw_hex":"e84f000000","size":5,"soft_fail":false,"target_va":5393902752,"va":5393902668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] event_listener_strategy::EventListenerFuture::wait
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"event-listener-strategy","version":"0.5.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14240250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902624,"raw_hex":"e8fa49d900","size":5,"soft_fail":false,"target_va":5408142912,"va":5393902657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] async_lock::mutex::Mutex::lock_blocking
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":380,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":82734,"source_line":143,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":400,"parent_record_offset":380,"ancestor_record_offset":264,"inlinee":82735,"source_line":112,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::try_locked_store
// 记录形式: compiled_procedure; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":440,"rva":25193568,"va":5393902688}
// 编译实例 RVAs: 0x1806c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] asset_pipe::store::open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t20","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902688,"raw_hex":"e814000000","size":5,"soft_fail":false,"target_va":5393902752,"va":5393902727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] async_lock::mutex::Mutex::try_lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":560,"parent_record_offset":440,"ancestor_record_offset":440,"inlinee":82736,"source_line":164,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":580,"parent_record_offset":560,"ancestor_record_offset":440,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":440,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::open
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":644,"rva":25193632,"va":5393902752}
// 编译实例 RVAs: 0x1806ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9246595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e87de872ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5393902830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-18224156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e8e4ebe9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393902855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142593f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: store.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14209565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e81dd2d800","size":5,"soft_fail":false,"target_va":5408112448,"va":5393902878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14259b7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: store.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14240426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e8aa4ad900","size":5,"soft_fail":false,"target_va":5408143296,"va":5393902865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: store.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22495728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e8f0415701","size":5,"soft_fail":false,"target_va":5416398672,"va":5393902939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::cache_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5471827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393902752,"raw_hex":"e8537e5300","size":5,"soft_fail":false,"target_va":5399374624,"va":5393902792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::store::open_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/store.rs
//     evidence={"module":1680,"record_offset":1528,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":82739,"source_line":81,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::store::store_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/store.rs
//     evidence={"module":1680,"record_offset":760,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":82738,"source_line":41,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":2992,"parent_record_offset":644,"ancestor_record_offset":644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1012,"parent_record_offset":960,"ancestor_record_offset":644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":936,"parent_record_offset":916,"ancestor_record_offset":644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":644,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":644,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":816,"parent_record_offset":760,"ancestor_record_offset":644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1680,"record_offset":1568,"parent_record_offset":1528,"ancestor_record_offset":644,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":644,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":644,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1680,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":644,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::open_at
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14333
// 首条位置: {"module":1680,"inlinee":82739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":1596,"parent_record_offset":1528,"ancestor_record_offset":1528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1680,"record_offset":1568,"parent_record_offset":1528,"ancestor_record_offset":1528,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":1528,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1528,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1528,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1680,"record_offset":1820,"parent_record_offset":1764,"ancestor_record_offset":1528,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":1528,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1528,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1680,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1528,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1528,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1680,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1528,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1680,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":1528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1528,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1680,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1528,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::store::open_at::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CF1D
// 首条位置: {"module":3255,"inlinee":118557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":2608,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3255,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2588,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03255.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3255,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":2588,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03255.jsonl.gz"}
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
// 完整结构及原始类型记录: store.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::store::StoreGuard
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset_pipe::store::StoreGuard>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset_pipe::store::StoreGuard"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: store.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: store.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]

// ------------------------------------------------------------------
// 函数: asset_pipe::store::impl$0::deref
// 信息 ID: 88cf8a09f8784ca756f2efc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 206 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2741 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x15092b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150952 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150bcc -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cd4 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cfe -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x14fde0 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150e75 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1879d5 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1875c2 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1878ea -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3be3 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3bf9 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3cb6 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x22453b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224c02 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::locked_store
// 信息 ID: 388ff5a285db11a8cad1d507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::ModSwapReloads::apply @ 0x907918 -> 0x1806c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::swap_replacement_mods::apply_preactivated_mod_textures @ 0x9086be -> 0x1806c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup @ 0x1ae2ca9 -> 0x1806c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258c205 -> 0x1806c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a45961 -> 0x1806c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a422f4 -> 0x1806c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::open
// 信息 ID: 12ba7797b6f771bd8bfa4f25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::store::locked_store @ 0x1806c4c -> 0x1806ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::store::try_locked_store @ 0x1806c87 -> 0x1806ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::open_at
// 信息 ID: 3659fcad38e125ee0308aaf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::store::open; module=1680, inline_record=1528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::open_at::closure$0
// 信息 ID: c806027016355e063392c618
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::store::open_at", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::store::open_at::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::sync::Arc<turbosloth::store::AssetStore,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::store::open_at::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::sync::Arc<turbosloth::store::AssetStore,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::store::open_at::closure_env$0>; module=3255, inline_record=2588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::store_path
// 信息 ID: d483cc975bf2494d8ecf3295
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::store::open; module=1680, inline_record=760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::store::try_locked_store
// 信息 ID: fdb99497299967f823ae44f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2cfff -> 0x1806c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
