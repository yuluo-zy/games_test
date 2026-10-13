// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/walls/material_stats.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 926e120a1c53824c34fa2480570be9e0338dd9fe35131d065c9b4ed573f3b1f7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: material_stats.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 509, 529, 542, 578, 1701
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use atomic_float; // namespace candidate
//   package identities: [{"name":"atomic_float","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FD6
// 首条位置: {"module":578,"inlinee":36822}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::init_this_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":61588,"rva":10547120,"va":5379256240}
// 编译实例 RVAs: 0xa0efb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3577585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256240,"raw_hex":"e80f69c9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379256284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::store
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62108,"parent_record_offset":61588,"ancestor_record_offset":61588,"inlinee":31784,"source_line":204,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":61756,"parent_record_offset":61588,"ancestor_record_offset":61588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":61992,"parent_record_offset":61968,"ancestor_record_offset":61588,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":62016,"parent_record_offset":61992,"ancestor_record_offset":61588,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":62040,"parent_record_offset":62016,"ancestor_record_offset":61588,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":61968,"parent_record_offset":61880,"ancestor_record_offset":61588,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":61908,"parent_record_offset":61880,"ancestor_record_offset":61588,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":61880,"parent_record_offset":61856,"ancestor_record_offset":61588,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":61856,"parent_record_offset":61836,"ancestor_record_offset":61588,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":62220,"parent_record_offset":62192,"ancestor_record_offset":61588,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":529,"record_offset":61936,"parent_record_offset":61908,"ancestor_record_offset":61588,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":61776,"parent_record_offset":61756,"ancestor_record_offset":61588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62132,"parent_record_offset":62108,"ancestor_record_offset":61588,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62156,"parent_record_offset":62132,"ancestor_record_offset":61588,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":66768,"rva":10548288,"va":5379257408}
// 编译实例 RVAs: 0xa0f440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3578799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379257408,"raw_hex":"e85164c9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379257498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37319264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379257408,"raw_hex":"e860723902","size":5,"soft_fail":false,"target_va":5416576832,"va":5379257563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4727374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379257408,"raw_hex":"e84e224800","size":5,"soft_fail":false,"target_va":5383984848,"va":5379257469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4727826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379257408,"raw_hex":"e812244800","size":5,"soft_fail":false,"target_va":5383985360,"va":5379257529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t37318555","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379257408,"raw_hex":"e99b6f3902","size":5,"soft_fail":false,"target_va":5416576144,"va":5379257584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":67096,"parent_record_offset":66768,"ancestor_record_offset":66768,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":529,"record_offset":66940,"parent_record_offset":66768,"ancestor_record_offset":66768,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":67272,"parent_record_offset":67248,"ancestor_record_offset":66768,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":67296,"parent_record_offset":67272,"ancestor_record_offset":66768,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":67320,"parent_record_offset":67296,"ancestor_record_offset":66768,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":67248,"parent_record_offset":67160,"ancestor_record_offset":66768,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":67188,"parent_record_offset":67160,"ancestor_record_offset":66768,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":67160,"parent_record_offset":67136,"ancestor_record_offset":66768,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":529,"record_offset":67136,"parent_record_offset":67116,"ancestor_record_offset":66768,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":529,"record_offset":67216,"parent_record_offset":67188,"ancestor_record_offset":66768,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":67116,"parent_record_offset":67096,"ancestor_record_offset":66768,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":67488,"parent_record_offset":67428,"ancestor_record_offset":66768,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":67004,"parent_record_offset":66968,"ancestor_record_offset":66768,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":67512,"parent_record_offset":67488,"ancestor_record_offset":66768,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":67028,"parent_record_offset":67004,"ancestor_record_offset":66768,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":529,"record_offset":67400,"parent_record_offset":67376,"ancestor_record_offset":66768,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":529,"record_offset":66968,"parent_record_offset":66940,"ancestor_record_offset":66768,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":529,"record_offset":67428,"parent_record_offset":67400,"ancestor_record_offset":66768,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":529,"record_offset":67060,"parent_record_offset":66968,"ancestor_record_offset":66768,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":529,"record_offset":67460,"parent_record_offset":67428,"ancestor_record_offset":66768,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C50
// 首条位置: {"module":1701,"inlinee":31824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1701,"record_offset":2272,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":2132,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":68532,"rva":10548576,"va":5379257696}
// 编译实例 RVAs: 0xa0f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68704,"parent_record_offset":68532,"ancestor_record_offset":68532,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68724,"parent_record_offset":68704,"ancestor_record_offset":68532,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":67848,"rva":10548544,"va":5379257664}
// 编译实例 RVAs: 0xa0f540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68020,"parent_record_offset":67848,"ancestor_record_offset":67848,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68040,"parent_record_offset":68020,"ancestor_record_offset":67848,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":69676,"rva":10548624,"va":5379257744}
// 编译实例 RVAs: 0xa0f590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69852,"parent_record_offset":69676,"ancestor_record_offset":69676,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69872,"parent_record_offset":69852,"ancestor_record_offset":69676,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":64384,"rva":10547440,"va":5379256560}
// 编译实例 RVAs: 0xa0f0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64556,"parent_record_offset":64384,"ancestor_record_offset":64384,"inlinee":31769,"source_line":536,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64968,"parent_record_offset":64384,"ancestor_record_offset":64384,"inlinee":31781,"source_line":559,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64600,"parent_record_offset":64576,"ancestor_record_offset":64384,"inlinee":31771,"source_line":458,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64576,"parent_record_offset":64556,"ancestor_record_offset":64384,"inlinee":31770,"source_line":511,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64860,"parent_record_offset":64840,"ancestor_record_offset":64384,"inlinee":31780,"source_line":537,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":65280,"parent_record_offset":65260,"ancestor_record_offset":64384,"inlinee":31783,"source_line":560,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64728,"parent_record_offset":64648,"ancestor_record_offset":64384,"inlinee":31775,"source_line":469,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64840,"parent_record_offset":64728,"ancestor_record_offset":64384,"inlinee":31778,"source_line":515,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":64752,"parent_record_offset":64728,"ancestor_record_offset":64384,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":64804,"parent_record_offset":64784,"ancestor_record_offset":64384,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":64784,"parent_record_offset":64752,"ancestor_record_offset":64384,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64892,"parent_record_offset":64648,"ancestor_record_offset":64384,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64624,"parent_record_offset":64600,"ancestor_record_offset":64384,"inlinee":31772,"source_line":3319,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64676,"parent_record_offset":64648,"ancestor_record_offset":64384,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::try_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64648,"parent_record_offset":64624,"ancestor_record_offset":64384,"inlinee":31773,"source_line":3379,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64916,"parent_record_offset":64892,"ancestor_record_offset":64384,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":64696,"parent_record_offset":64676,"ancestor_record_offset":64384,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_arches_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":62376,"rva":10547280,"va":5379256400}
// 编译实例 RVAs: 0xa0f050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62544,"parent_record_offset":62376,"ancestor_record_offset":62376,"inlinee":31769,"source_line":536,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62956,"parent_record_offset":62376,"ancestor_record_offset":62376,"inlinee":31781,"source_line":559,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62588,"parent_record_offset":62564,"ancestor_record_offset":62376,"inlinee":31771,"source_line":458,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62564,"parent_record_offset":62544,"ancestor_record_offset":62376,"inlinee":31770,"source_line":511,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62848,"parent_record_offset":62828,"ancestor_record_offset":62376,"inlinee":31780,"source_line":537,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63268,"parent_record_offset":63248,"ancestor_record_offset":62376,"inlinee":31783,"source_line":560,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62716,"parent_record_offset":62636,"ancestor_record_offset":62376,"inlinee":31775,"source_line":469,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":62828,"parent_record_offset":62716,"ancestor_record_offset":62376,"inlinee":31778,"source_line":515,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":62740,"parent_record_offset":62716,"ancestor_record_offset":62376,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":62792,"parent_record_offset":62772,"ancestor_record_offset":62376,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":62772,"parent_record_offset":62740,"ancestor_record_offset":62376,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62880,"parent_record_offset":62636,"ancestor_record_offset":62376,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62612,"parent_record_offset":62588,"ancestor_record_offset":62376,"inlinee":31772,"source_line":3319,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62664,"parent_record_offset":62636,"ancestor_record_offset":62376,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::try_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62636,"parent_record_offset":62612,"ancestor_record_offset":62376,"inlinee":31773,"source_line":3379,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62904,"parent_record_offset":62880,"ancestor_record_offset":62376,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":62684,"parent_record_offset":62664,"ancestor_record_offset":62376,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_gate_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":59580,"rva":10546960,"va":5379256080}
// 编译实例 RVAs: 0xa0ef10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":59748,"parent_record_offset":59580,"ancestor_record_offset":59580,"inlinee":31769,"source_line":536,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60160,"parent_record_offset":59580,"ancestor_record_offset":59580,"inlinee":31781,"source_line":559,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":59792,"parent_record_offset":59768,"ancestor_record_offset":59580,"inlinee":31771,"source_line":458,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":59768,"parent_record_offset":59748,"ancestor_record_offset":59580,"inlinee":31770,"source_line":511,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60052,"parent_record_offset":60032,"ancestor_record_offset":59580,"inlinee":31780,"source_line":537,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60472,"parent_record_offset":60452,"ancestor_record_offset":59580,"inlinee":31783,"source_line":560,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":59920,"parent_record_offset":59840,"ancestor_record_offset":59580,"inlinee":31775,"source_line":469,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60032,"parent_record_offset":59920,"ancestor_record_offset":59580,"inlinee":31778,"source_line":515,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":59944,"parent_record_offset":59920,"ancestor_record_offset":59580,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":59996,"parent_record_offset":59976,"ancestor_record_offset":59580,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":59976,"parent_record_offset":59944,"ancestor_record_offset":59580,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60084,"parent_record_offset":59840,"ancestor_record_offset":59580,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":59816,"parent_record_offset":59792,"ancestor_record_offset":59580,"inlinee":31772,"source_line":3319,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":59868,"parent_record_offset":59840,"ancestor_record_offset":59580,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::try_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":59840,"parent_record_offset":59816,"ancestor_record_offset":59580,"inlinee":31773,"source_line":3379,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60108,"parent_record_offset":60084,"ancestor_record_offset":59580,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":59888,"parent_record_offset":59868,"ancestor_record_offset":59580,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_sewer_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":60584,"rva":10547040,"va":5379256160}
// 编译实例 RVAs: 0xa0ef60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60752,"parent_record_offset":60584,"ancestor_record_offset":60584,"inlinee":31769,"source_line":536,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":61164,"parent_record_offset":60584,"ancestor_record_offset":60584,"inlinee":31781,"source_line":559,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60796,"parent_record_offset":60772,"ancestor_record_offset":60584,"inlinee":31771,"source_line":458,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60772,"parent_record_offset":60752,"ancestor_record_offset":60584,"inlinee":31770,"source_line":511,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":61056,"parent_record_offset":61036,"ancestor_record_offset":60584,"inlinee":31780,"source_line":537,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":61476,"parent_record_offset":61456,"ancestor_record_offset":60584,"inlinee":31783,"source_line":560,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":60924,"parent_record_offset":60844,"ancestor_record_offset":60584,"inlinee":31775,"source_line":469,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":61036,"parent_record_offset":60924,"ancestor_record_offset":60584,"inlinee":31778,"source_line":515,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":60948,"parent_record_offset":60924,"ancestor_record_offset":60584,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":61000,"parent_record_offset":60980,"ancestor_record_offset":60584,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":60980,"parent_record_offset":60948,"ancestor_record_offset":60584,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":61088,"parent_record_offset":60844,"ancestor_record_offset":60584,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60820,"parent_record_offset":60796,"ancestor_record_offset":60584,"inlinee":31772,"source_line":3319,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60872,"parent_record_offset":60844,"ancestor_record_offset":60584,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::try_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60844,"parent_record_offset":60820,"ancestor_record_offset":60584,"inlinee":31773,"source_line":3379,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":61112,"parent_record_offset":61088,"ancestor_record_offset":60584,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":60892,"parent_record_offset":60872,"ancestor_record_offset":60584,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":63380,"rva":10547360,"va":5379256480}
// 编译实例 RVAs: 0xa0f0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63548,"parent_record_offset":63380,"ancestor_record_offset":63380,"inlinee":31769,"source_line":536,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63960,"parent_record_offset":63380,"ancestor_record_offset":63380,"inlinee":31781,"source_line":559,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63592,"parent_record_offset":63568,"ancestor_record_offset":63380,"inlinee":31771,"source_line":458,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63568,"parent_record_offset":63548,"ancestor_record_offset":63380,"inlinee":31770,"source_line":511,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63852,"parent_record_offset":63832,"ancestor_record_offset":63380,"inlinee":31780,"source_line":537,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":64272,"parent_record_offset":64252,"ancestor_record_offset":63380,"inlinee":31783,"source_line":560,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63720,"parent_record_offset":63640,"ancestor_record_offset":63380,"inlinee":31775,"source_line":469,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":63832,"parent_record_offset":63720,"ancestor_record_offset":63380,"inlinee":31778,"source_line":515,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":63744,"parent_record_offset":63720,"ancestor_record_offset":63380,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":63796,"parent_record_offset":63776,"ancestor_record_offset":63380,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":63776,"parent_record_offset":63744,"ancestor_record_offset":63380,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63884,"parent_record_offset":63640,"ancestor_record_offset":63380,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63616,"parent_record_offset":63592,"ancestor_record_offset":63380,"inlinee":31772,"source_line":3319,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63668,"parent_record_offset":63640,"ancestor_record_offset":63380,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::try_update
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63640,"parent_record_offset":63616,"ancestor_record_offset":63380,"inlinee":31773,"source_line":3379,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63908,"parent_record_offset":63884,"ancestor_record_offset":63380,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":63688,"parent_record_offset":63668,"ancestor_record_offset":63380,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":69444,"rva":1697760,"va":5370406880}
// 编译实例 RVAs: 0x19e7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69620,"parent_record_offset":69444,"ancestor_record_offset":69444,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69640,"parent_record_offset":69620,"ancestor_record_offset":69444,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":68988,"rva":4800048,"va":5373509168}
// 编译实例 RVAs: 0x493e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69160,"parent_record_offset":68988,"ancestor_record_offset":68988,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69180,"parent_record_offset":69160,"ancestor_record_offset":68988,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":68304,"rva":2294240,"va":5371003360}
// 编译实例 RVAs: 0x2301e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68476,"parent_record_offset":68304,"ancestor_record_offset":68304,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68496,"parent_record_offset":68476,"ancestor_record_offset":68304,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":68760,"rva":10548592,"va":5379257712}
// 编译实例 RVAs: 0xa0f570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68932,"parent_record_offset":68760,"ancestor_record_offset":68760,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68952,"parent_record_offset":68932,"ancestor_record_offset":68760,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":69216,"rva":10548608,"va":5379257728}
// 编译实例 RVAs: 0xa0f580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69388,"parent_record_offset":69216,"ancestor_record_offset":69216,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":69408,"parent_record_offset":69388,"ancestor_record_offset":69216,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":68076,"rva":10548560,"va":5379257680}
// 编译实例 RVAs: 0xa0f550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68248,"parent_record_offset":68076,"ancestor_record_offset":68076,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":68268,"parent_record_offset":68248,"ancestor_record_offset":68076,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C62
// 首条位置: {"module":1701,"inlinee":31842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1304,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C60
// 首条位置: {"module":1701,"inlinee":31840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7828,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7828,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C29
// 首条位置: {"module":529,"inlinee":31785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70152,"parent_record_offset":70120,"ancestor_record_offset":70120,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1448,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70176,"parent_record_offset":70152,"ancestor_record_offset":70120,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70208,"parent_record_offset":70176,"ancestor_record_offset":70120,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2C
// 首条位置: {"module":529,"inlinee":31788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":71168,"parent_record_offset":71144,"ancestor_record_offset":71144,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":71192,"parent_record_offset":71168,"ancestor_record_offset":71144,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":71216,"parent_record_offset":71192,"ancestor_record_offset":71144,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2B
// 首条位置: {"module":529,"inlinee":31787}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70604,"parent_record_offset":70568,"ancestor_record_offset":70568,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":3176,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70632,"parent_record_offset":70604,"ancestor_record_offset":70568,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70664,"parent_record_offset":70632,"ancestor_record_offset":70568,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2D
// 首条位置: {"module":529,"inlinee":31789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70824,"parent_record_offset":70800,"ancestor_record_offset":70800,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":70848,"parent_record_offset":70824,"ancestor_record_offset":70800,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2E
// 首条位置: {"module":529,"inlinee":31790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70940,"parent_record_offset":70912,"ancestor_record_offset":70912,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1844,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70968,"parent_record_offset":70940,"ancestor_record_offset":70912,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":71004,"parent_record_offset":70968,"ancestor_record_offset":70912,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C5D
// 首条位置: {"module":1701,"inlinee":31837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2016,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1988,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C59
// 首条位置: {"module":1701,"inlinee":31833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":2304,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":2304,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement
// 记录形式: compiled_procedure; instances=1
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":65392,"rva":10547520,"va":5379256640}
// 编译实例 RVAs: 0xa0f140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9951798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256640,"raw_hex":"e8ca2568ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5379257377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37319552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256640,"raw_hex":"e880733902","size":5,"soft_fail":false,"target_va":5416576832,"va":5379257275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37318826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256640,"raw_hex":"e8aa703902","size":5,"soft_fail":false,"target_va":5416576144,"va":5379257313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4728144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256640,"raw_hex":"e850254800","size":5,"soft_fail":false,"target_va":5383984848,"va":5379256699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4728149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379256640,"raw_hex":"e855254800","size":5,"soft_fail":false,"target_va":5383985360,"va":5379257206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":66356,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":529,"record_offset":66036,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":65896,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":66388,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":529,"record_offset":66208,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":529,"record_offset":66120,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":529,"record_offset":65584,"parent_record_offset":65392,"ancestor_record_offset":65392,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":65992,"parent_record_offset":65968,"ancestor_record_offset":65392,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":65968,"parent_record_offset":65944,"ancestor_record_offset":65392,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":66312,"parent_record_offset":66284,"ancestor_record_offset":65392,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":529,"record_offset":65920,"parent_record_offset":65896,"ancestor_record_offset":65392,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":529,"record_offset":65944,"parent_record_offset":65920,"ancestor_record_offset":65392,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":529,"record_offset":66072,"parent_record_offset":66036,"ancestor_record_offset":65392,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":529,"record_offset":66160,"parent_record_offset":66120,"ancestor_record_offset":65392,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":66508,"parent_record_offset":66444,"ancestor_record_offset":65392,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":65648,"parent_record_offset":65612,"ancestor_record_offset":65392,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":66532,"parent_record_offset":66508,"ancestor_record_offset":65392,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":65672,"parent_record_offset":65648,"ancestor_record_offset":65392,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":529,"record_offset":66236,"parent_record_offset":66208,"ancestor_record_offset":65392,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":529,"record_offset":66416,"parent_record_offset":66388,"ancestor_record_offset":65392,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":529,"record_offset":65612,"parent_record_offset":65584,"ancestor_record_offset":65392,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":529,"record_offset":66444,"parent_record_offset":66416,"ancestor_record_offset":65392,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":529,"record_offset":65704,"parent_record_offset":65612,"ancestor_record_offset":65392,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":529,"record_offset":66480,"parent_record_offset":66444,"ancestor_record_offset":65392,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":529,"record_offset":66284,"parent_record_offset":66236,"ancestor_record_offset":65392,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2A
// 首条位置: {"module":529,"inlinee":31786}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70368,"parent_record_offset":70344,"ancestor_record_offset":70344,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":3476,"parent_record_offset":3452,"ancestor_record_offset":3428,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70392,"parent_record_offset":70368,"ancestor_record_offset":70344,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70416,"parent_record_offset":70392,"ancestor_record_offset":70344,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C2F
// 首条位置: {"module":529,"inlinee":31791}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":71376,"parent_record_offset":71340,"ancestor_record_offset":71340,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":3092,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":71404,"parent_record_offset":71376,"ancestor_record_offset":71340,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":71436,"parent_record_offset":71404,"ancestor_record_offset":71340,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C5E
// 首条位置: {"module":1701,"inlinee":31838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":1701,"record_offset":572,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":684,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1701,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":512,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":512,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1701,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":512,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71FE
// 首条位置: {"module":509,"inlinee":29182}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":509,"record_offset":48368,"parent_record_offset":48336,"ancestor_record_offset":48336,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":48392,"parent_record_offset":48368,"ancestor_record_offset":48336,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":48416,"parent_record_offset":48392,"ancestor_record_offset":48336,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most
// 记录形式: compiled_procedure; instances=1
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":69908,"rva":10548640,"va":5379257760}
// 编译实例 RVAs: 0xa0f5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":70540,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":71600,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":70344,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31786,"source_line":220,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":71144,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31788,"source_line":166,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":70120,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31785,"source_line":161,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":70800,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31789,"source_line":176,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":70568,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31787,"source_line":171,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":70912,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31790,"source_line":180,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":71340,"parent_record_offset":69908,"ancestor_record_offset":69908,"inlinee":31791,"source_line":225,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":529,"record_offset":70152,"parent_record_offset":70120,"ancestor_record_offset":69908,"inlinee":29184,"source_line":172,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":72012,"parent_record_offset":71864,"ancestor_record_offset":69908,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":71864,"parent_record_offset":71820,"ancestor_record_offset":69908,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":70848,"parent_record_offset":70824,"ancestor_record_offset":69908,"inlinee":31776,"source_line":1227,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":71976,"parent_record_offset":71952,"ancestor_record_offset":69908,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":529,"record_offset":71688,"parent_record_offset":71644,"ancestor_record_offset":69908,"inlinee":10794,"source_line":129,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":529,"record_offset":71776,"parent_record_offset":71732,"ancestor_record_offset":69908,"inlinee":5430,"source_line":139,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":71820,"parent_record_offset":71776,"ancestor_record_offset":69908,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":71644,"parent_record_offset":71600,"ancestor_record_offset":69908,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":71928,"parent_record_offset":71864,"ancestor_record_offset":69908,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":71732,"parent_record_offset":71688,"ancestor_record_offset":69908,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70176,"parent_record_offset":70152,"ancestor_record_offset":69908,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":70208,"parent_record_offset":70176,"ancestor_record_offset":69908,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":529,"record_offset":71952,"parent_record_offset":71928,"ancestor_record_offset":69908,"inlinee":31793,"source_line":310,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C31
// 首条位置: {"module":529,"inlinee":31793}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":529,"record_offset":71976,"parent_record_offset":71952,"ancestor_record_offset":71952,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00529.jsonl.gz"}
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
// 完整结构及原始类型记录: material_stats.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::material_stats::MaterialStats
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 38; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::material_stats::MaterialStats>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::material_stats::MaterialStats"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: material_stats.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: material_stats.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_arches_length
// 信息 ID: ce7678cc77170746dad65269
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee343 -> 0xa0f050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_gate_length
// 信息 ID: f7caf2ec0a5c83b42a54960a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee35f -> 0xa0ef10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
// 信息 ID: e81c126881ec5dad696a2c12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811fd9 -> 0xa0f0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_sewer_length
// 信息 ID: 006bb976e3f5b816fa8a29a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee399 -> 0xa0ef60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
// 信息 ID: 854bc531c973b8e611310fb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f24a7 -> 0xa0f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 信息 ID: 502fa1cfad860a609891f493
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=70344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 信息 ID: b17fca88ad2f1f6f05c760aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=2304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 信息 ID: b29baf2e4dba39453c5e6806
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=71144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 信息 ID: 7ffefa66f9e357da174e93ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=70120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 信息 ID: 0fdfe1d6e29c15208f101f0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=70800; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1760; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=2924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 信息 ID: 5d4e90234ac920926a799d1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1701, inline_record=7828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 信息 ID: ef346f9a1025eed2ba756a6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::init_this_update
// 信息 ID: 796e5abcfafedffb49345205
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x7fccb1 -> 0xa0efb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 信息 ID: bac8c730b81b679b4bd7a501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=48336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=48772; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio; module=509, inline_record=53680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1701, inline_record=6268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 信息 ID: 1f5b1a30a1b171c321f6f199
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=70568; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3176; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 信息 ID: c4ea93fc7cf0d771b0a5200d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=2448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement
// 信息 ID: 63842439a16eabd08400995e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details @ 0x197b25a -> 0xa0f140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 信息 ID: 4c6023226e13861d9cc2dcb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 信息 ID: a6abe3ebe64c6dea09fda3cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=2132; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=2592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
// 信息 ID: 821442b884596d781a792dfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffe63 -> 0xa0f540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count
// 信息 ID: cded748503817ac5239b1ff4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee324 -> 0x493e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
// 信息 ID: 3979cc284917ffb511f7d53f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x80dccf -> 0xa0f580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
// 信息 ID: 882dab00d6f7c1df80106fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x80dcdb -> 0xa0f550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count
// 信息 ID: c6a08cde461d51074d7f6776
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee37d -> 0x2301e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
// 信息 ID: e61fea69401c15d575d1a2bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffe4a -> 0xa0f560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count
// 信息 ID: b6b5fdb59e4edc7130a1c507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811fc6 -> 0x19e7e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
// 信息 ID: ab978c83fbad8600f10ed0b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x7fccc2 -> 0xa0f590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
// 信息 ID: 2a00622ab5ba25690a5646bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffe98 -> 0xa0f440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count
// 信息 ID: ed4d02c0ddc299f0fc737bcf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ee3b7 -> 0xa0f570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 信息 ID: cbaef8dd12756f68fb79228c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=70912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=1844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 信息 ID: 89bf9f1e82805b731f86525f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=71340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_details; module=1701, inline_record=3260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most
// 信息 ID: 9d33709539350d9d0207a490
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator @ 0x197b6d4 -> 0xa0f5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 信息 ID: 364c623ba2522874dc0c1e90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=71952; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=72088; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=72180; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=72272; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=72364; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=529, inline_record=72456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$1::default
// 信息 ID: 1630b97e366879ad9a85277a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=578, inline_record=85172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=578, inline_record=85204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
