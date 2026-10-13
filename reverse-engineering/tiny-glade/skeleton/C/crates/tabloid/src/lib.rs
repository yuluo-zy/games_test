// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tabloid/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ddca6c5ca6fcb266c938eda8a3c33923911fdb23ff5e3408b5cdf9a1e9f27e56
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 318, 421, 457, 461, 511, 515, 518, 519, 521, 527, 550, 581, 585, 603, 607, 613, 1133, 1461, 1534, 1551, 1599, 1643, 2591, 2754, 2756, 4054, 4056, 4076, 4735
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tabloid::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5005
// 首条位置: {"module":318,"inlinee":20485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>
// 记录形式: compiled_procedure; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":45640,"rva":10255632,"va":5378964752}
// 编译实例 RVAs: 0x9c7d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3286156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e874dbcdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378964839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10242637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e8b3b563ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378964824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c908b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2919018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e86a8a2c00","size":5,"soft_fail":false,"target_va":5381884080,"va":5378965057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91500
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2922229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e8f5962c00","size":5,"soft_fail":false,"target_va":5381887232,"va":5378964998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37385811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e853763a02","size":5,"soft_fail":false,"target_va":5416350720,"va":5378964904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37655814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378964752,"raw_hex":"e806953e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378964950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":45852,"parent_record_offset":45640,"ancestor_record_offset":45640,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":46372,"parent_record_offset":45640,"ancestor_record_offset":45640,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":46460,"parent_record_offset":45640,"ancestor_record_offset":45640,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":46836,"parent_record_offset":45640,"ancestor_record_offset":45640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":45824,"parent_record_offset":45640,"ancestor_record_offset":45640,"inlinee":8490,"source_line":149,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":45972,"parent_record_offset":45948,"ancestor_record_offset":45640,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":45996,"parent_record_offset":45972,"ancestor_record_offset":45640,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":46020,"parent_record_offset":45996,"ancestor_record_offset":45640,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":45948,"parent_record_offset":45920,"ancestor_record_offset":45640,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":45920,"parent_record_offset":45880,"ancestor_record_offset":45640,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":45880,"parent_record_offset":45852,"ancestor_record_offset":45640,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":45640,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":46608,"parent_record_offset":46584,"ancestor_record_offset":45640,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":46584,"parent_record_offset":46560,"ancestor_record_offset":45640,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":46896,"parent_record_offset":46860,"ancestor_record_offset":45640,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":46216,"parent_record_offset":46188,"ancestor_record_offset":45640,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":46536,"parent_record_offset":46492,"ancestor_record_offset":45640,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":46492,"parent_record_offset":46460,"ancestor_record_offset":45640,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":46064,"parent_record_offset":45880,"ancestor_record_offset":45640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":46928,"parent_record_offset":46896,"ancestor_record_offset":45640,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":46648,"parent_record_offset":46492,"ancestor_record_offset":45640,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":46400,"parent_record_offset":46372,"ancestor_record_offset":45640,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":46252,"parent_record_offset":46216,"ancestor_record_offset":45640,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":46424,"parent_record_offset":46400,"ancestor_record_offset":45640,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":46276,"parent_record_offset":46252,"ancestor_record_offset":45640,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":46308,"parent_record_offset":46216,"ancestor_record_offset":45640,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=4
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":47028,"rva":10256000,"va":5378965120}
// 编译实例 RVAs: 0x9c7e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3286524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e804dacdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378965207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10243005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e843b463ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378965192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90c90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2919701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e8158d2c00","size":5,"soft_fail":false,"target_va":5381885072,"va":5378965366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c912b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2921210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e8fa922c00","size":5,"soft_fail":false,"target_va":5381886640,"va":5378965425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37385443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e8e3743a02","size":5,"soft_fail":false,"target_va":5416350720,"va":5378965272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37655446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965120,"raw_hex":"e896933e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378965318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":47232,"parent_record_offset":47028,"ancestor_record_offset":47028,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":47752,"parent_record_offset":47028,"ancestor_record_offset":47028,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":47840,"parent_record_offset":47028,"ancestor_record_offset":47028,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":48216,"parent_record_offset":47028,"ancestor_record_offset":47028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":47204,"parent_record_offset":47028,"ancestor_record_offset":47028,"inlinee":8490,"source_line":149,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":47352,"parent_record_offset":47328,"ancestor_record_offset":47028,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":47376,"parent_record_offset":47352,"ancestor_record_offset":47028,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":47400,"parent_record_offset":47376,"ancestor_record_offset":47028,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":47328,"parent_record_offset":47300,"ancestor_record_offset":47028,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":47300,"parent_record_offset":47260,"ancestor_record_offset":47028,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":47260,"parent_record_offset":47232,"ancestor_record_offset":47028,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":47940,"parent_record_offset":47916,"ancestor_record_offset":47028,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":47988,"parent_record_offset":47964,"ancestor_record_offset":47028,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":47964,"parent_record_offset":47940,"ancestor_record_offset":47028,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":48276,"parent_record_offset":48240,"ancestor_record_offset":47028,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":47596,"parent_record_offset":47568,"ancestor_record_offset":47028,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":47916,"parent_record_offset":47872,"ancestor_record_offset":47028,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":47872,"parent_record_offset":47840,"ancestor_record_offset":47028,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":47444,"parent_record_offset":47260,"ancestor_record_offset":47028,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":48308,"parent_record_offset":48276,"ancestor_record_offset":47028,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":48028,"parent_record_offset":47872,"ancestor_record_offset":47028,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":47780,"parent_record_offset":47752,"ancestor_record_offset":47028,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":47632,"parent_record_offset":47596,"ancestor_record_offset":47028,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":47804,"parent_record_offset":47780,"ancestor_record_offset":47028,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":47656,"parent_record_offset":47632,"ancestor_record_offset":47028,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":47688,"parent_record_offset":47596,"ancestor_record_offset":47028,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":40636,"rva":10251728,"va":5378960848}
// 编译实例 RVAs: 0x9c6dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3282807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e889e8cdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378961490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10239288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e8c8c263ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378961475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140088a20
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9693246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e8c2176cff","size":5,"soft_fail":false,"target_va":5369268768,"va":5378962009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9656818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e80ea66cff","size":5,"soft_fail":false,"target_va":5369305584,"va":5378962397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=2
//     target_aliases=34 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9300741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e8fb1472ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5378961104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409852e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-270293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e82be0fbff","size":5,"soft_fail":false,"target_va":5378691808,"va":5378962096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099c7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-174968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e88854fdff","size":5,"soft_fail":false,"target_va":5378787232,"va":5378962195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91500
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2925604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e824a42c00","size":5,"soft_fail":false,"target_va":5381887232,"va":5378961623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca5be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3009607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e847ec2d00","size":5,"soft_fail":false,"target_va":5381970912,"va":5378961300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414986f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11343733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e87517ad00","size":5,"soft_fail":false,"target_va":5390305008,"va":5378961270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414c1560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11511193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e899a5af00","size":5,"soft_fail":false,"target_va":5390472544,"va":5378961346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f2ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12762264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e898bcc200","size":5,"soft_fail":false,"target_va":5391723200,"va":5378960931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37388956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e89c823a02","size":5,"soft_fail":false,"target_va":5416350720,"va":5378961759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37658933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e835a13e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378961831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37615114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e80af63d02","size":5,"soft_fail":false,"target_va":5416576832,"va":5378961713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37614446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e86ef33d02","size":5,"soft_fail":false,"target_va":5416576144,"va":5378961693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5023791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e82fa84c00","size":5,"soft_fail":false,"target_va":5383984848,"va":5378961052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5023697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e8d1a74c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378961658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37657509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378960848,"raw_hex":"e8a59b3e02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378961846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":42732,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":41076,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":43180,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":40900,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":41316,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":41260,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":42820,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":519,"record_offset":41012,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":41104,"parent_record_offset":40636,"ancestor_record_offset":40636,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42620,"parent_record_offset":42596,"ancestor_record_offset":40636,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":40636,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42668,"parent_record_offset":42644,"ancestor_record_offset":40636,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42596,"parent_record_offset":42564,"ancestor_record_offset":40636,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":42564,"parent_record_offset":42520,"ancestor_record_offset":40636,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":42520,"parent_record_offset":42484,"ancestor_record_offset":40636,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":43272,"parent_record_offset":43248,"ancestor_record_offset":40636,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":43320,"parent_record_offset":43296,"ancestor_record_offset":40636,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":43296,"parent_record_offset":43272,"ancestor_record_offset":40636,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":42484,"parent_record_offset":41356,"ancestor_record_offset":40636,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":42868,"parent_record_offset":42844,"ancestor_record_offset":40636,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":43248,"parent_record_offset":43208,"ancestor_record_offset":40636,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":43208,"parent_record_offset":43180,"ancestor_record_offset":40636,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41696,"parent_record_offset":41668,"ancestor_record_offset":40636,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":40636,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41724,"parent_record_offset":41668,"ancestor_record_offset":40636,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":41772,"parent_record_offset":41748,"ancestor_record_offset":40636,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":40636,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":41640,"parent_record_offset":41612,"ancestor_record_offset":40636,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":42844,"parent_record_offset":42820,"ancestor_record_offset":40636,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":519,"record_offset":40972,"parent_record_offset":40948,"ancestor_record_offset":40636,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":41040,"parent_record_offset":41012,"ancestor_record_offset":40636,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":43360,"parent_record_offset":43208,"ancestor_record_offset":40636,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":519,"record_offset":40924,"parent_record_offset":40900,"ancestor_record_offset":40636,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":519,"record_offset":40948,"parent_record_offset":40924,"ancestor_record_offset":40636,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":43112,"parent_record_offset":43048,"ancestor_record_offset":40636,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":41168,"parent_record_offset":41132,"ancestor_record_offset":40636,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":42760,"parent_record_offset":42732,"ancestor_record_offset":40636,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":42904,"parent_record_offset":42868,"ancestor_record_offset":40636,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":42784,"parent_record_offset":42760,"ancestor_record_offset":40636,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":43136,"parent_record_offset":43112,"ancestor_record_offset":40636,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":41192,"parent_record_offset":41168,"ancestor_record_offset":40636,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":42928,"parent_record_offset":42904,"ancestor_record_offset":40636,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":42960,"parent_record_offset":42868,"ancestor_record_offset":40636,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$60::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":41448,"parent_record_offset":41416,"ancestor_record_offset":40636,"inlinee":30542,"source_line":216,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$6::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":41416,"parent_record_offset":41356,"ancestor_record_offset":40636,"inlinee":30540,"source_line":223,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":42176,"parent_record_offset":41356,"ancestor_record_offset":40636,"inlinee":30544,"source_line":234,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":40636,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":40636,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42324,"parent_record_offset":42296,"ancestor_record_offset":40636,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":519,"record_offset":42392,"parent_record_offset":42368,"ancestor_record_offset":40636,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":519,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":40636,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":43024,"parent_record_offset":43000,"ancestor_record_offset":40636,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":41132,"parent_record_offset":41104,"ancestor_record_offset":40636,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":43048,"parent_record_offset":43024,"ancestor_record_offset":40636,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":41224,"parent_record_offset":41132,"ancestor_record_offset":40636,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":43084,"parent_record_offset":43048,"ancestor_record_offset":40636,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":42368,"parent_record_offset":42176,"ancestor_record_offset":40636,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":42224,"parent_record_offset":42176,"ancestor_record_offset":40636,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":41496,"parent_record_offset":41448,"ancestor_record_offset":40636,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41552,"parent_record_offset":41524,"ancestor_record_offset":40636,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41668,"parent_record_offset":41612,"ancestor_record_offset":40636,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41524,"parent_record_offset":41496,"ancestor_record_offset":40636,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":519,"record_offset":41612,"parent_record_offset":41580,"ancestor_record_offset":40636,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":519,"record_offset":41580,"parent_record_offset":41552,"ancestor_record_offset":40636,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$1::publish::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":519,"record_offset":41356,"parent_record_offset":41316,"ancestor_record_offset":40636,"inlinee":30538,"source_line":111,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":43704,"rva":10253408,"va":5378962528}
// 编译实例 RVAs: 0x9c7460, 0x28d47d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3284471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e809e2cdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378963154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10240952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e848bc63ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378963139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=2
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9658863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e8119e6cff","size":5,"soft_fail":false,"target_va":5369305584,"va":5378964442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31959466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411522512,"raw_hex":"e8565618fe","size":5,"soft_fail":false,"target_va":5379563296,"va":5411522757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a634f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31921148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411522512,"raw_hex":"e804ec18fe","size":5,"soft_fail":false,"target_va":5379601648,"va":5411522791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90c90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2921489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e811942c00","size":5,"soft_fail":false,"target_va":5381885072,"va":5378963578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f3700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12763658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e80ac2c200","size":5,"soft_fail":false,"target_va":5391726336,"va":5378962673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14207bbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8752234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411522512,"raw_hex":"e896737aff","size":5,"soft_fail":false,"target_va":5402770352,"va":5411522581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37386992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e8f07a3a02","size":5,"soft_fail":false,"target_va":5416350720,"va":5378963723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5534449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411522512,"raw_hex":"e8f1725400","size":5,"soft_fail":false,"target_va":5417057056,"va":5411522602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37656907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e84b993e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378963857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37613153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e861ee3d02","size":5,"soft_fail":false,"target_va":5416576832,"va":5378963674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37612485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e8c5eb3d02","size":5,"soft_fail":false,"target_va":5416576144,"va":5378963654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5021756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e83ca04c00","size":5,"soft_fail":false,"target_va":5383984848,"va":5378963087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5021736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378962528,"raw_hex":"e828a04c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378963619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":44660,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":44116,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":45108,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":43940,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":44356,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":44300,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":44748,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":519,"record_offset":44052,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":44144,"parent_record_offset":43704,"ancestor_record_offset":43704,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":44548,"parent_record_offset":44524,"ancestor_record_offset":43704,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":44572,"parent_record_offset":44548,"ancestor_record_offset":43704,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":44596,"parent_record_offset":44572,"ancestor_record_offset":43704,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":44524,"parent_record_offset":44492,"ancestor_record_offset":43704,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":44492,"parent_record_offset":44456,"ancestor_record_offset":43704,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":44456,"parent_record_offset":44424,"ancestor_record_offset":43704,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":45208,"parent_record_offset":45184,"ancestor_record_offset":43704,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":45256,"parent_record_offset":45232,"ancestor_record_offset":43704,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":519,"record_offset":45232,"parent_record_offset":45208,"ancestor_record_offset":43704,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":44424,"parent_record_offset":44392,"ancestor_record_offset":43704,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":44796,"parent_record_offset":44772,"ancestor_record_offset":43704,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":45184,"parent_record_offset":45140,"ancestor_record_offset":43704,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":45140,"parent_record_offset":45108,"ancestor_record_offset":43704,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":4076,"record_offset":1388,"parent_record_offset":1348,"ancestor_record_offset":660,"inlinee":12972,"source_line":227,"archive":"_pdb/C/modules/04076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":44324,"parent_record_offset":44300,"ancestor_record_offset":43704,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":44772,"parent_record_offset":44748,"ancestor_record_offset":43704,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":519,"record_offset":44012,"parent_record_offset":43988,"ancestor_record_offset":43704,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":44080,"parent_record_offset":44052,"ancestor_record_offset":43704,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4076,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":660,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":45296,"parent_record_offset":45140,"ancestor_record_offset":43704,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":519,"record_offset":43964,"parent_record_offset":43940,"ancestor_record_offset":43704,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":519,"record_offset":43988,"parent_record_offset":43964,"ancestor_record_offset":43704,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":45040,"parent_record_offset":44976,"ancestor_record_offset":43704,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44208,"parent_record_offset":44172,"ancestor_record_offset":43704,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44688,"parent_record_offset":44660,"ancestor_record_offset":43704,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44832,"parent_record_offset":44796,"ancestor_record_offset":43704,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44712,"parent_record_offset":44688,"ancestor_record_offset":43704,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":45064,"parent_record_offset":45040,"ancestor_record_offset":43704,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44232,"parent_record_offset":44208,"ancestor_record_offset":43704,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44856,"parent_record_offset":44832,"ancestor_record_offset":43704,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":44888,"parent_record_offset":44796,"ancestor_record_offset":43704,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":44952,"parent_record_offset":44928,"ancestor_record_offset":43704,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":44172,"parent_record_offset":44144,"ancestor_record_offset":43704,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":44976,"parent_record_offset":44952,"ancestor_record_offset":43704,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":44264,"parent_record_offset":44172,"ancestor_record_offset":43704,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":45012,"parent_record_offset":44976,"ancestor_record_offset":43704,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$1::publish::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":519,"record_offset":44392,"parent_record_offset":44356,"ancestor_record_offset":43704,"inlinee":30538,"source_line":111,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D4F
// 首条位置: {"module":1643,"inlinee":81231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":203300,"parent_record_offset":203252,"ancestor_record_offset":203252,"inlinee":10779,"source_line":2003,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":203420,"parent_record_offset":203252,"ancestor_record_offset":203252,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":203720,"parent_record_offset":203252,"ancestor_record_offset":203252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1643,"record_offset":212024,"parent_record_offset":211552,"ancestor_record_offset":211552,"inlinee":30544,"source_line":234,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$2::apply_to
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1643,"record_offset":203664,"parent_record_offset":203252,"ancestor_record_offset":203252,"inlinee":33115,"source_line":310,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":1643,"record_offset":203448,"parent_record_offset":203252,"ancestor_record_offset":203252,"inlinee":6090,"source_line":491,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$27::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":1643,"record_offset":211968,"parent_record_offset":211552,"ancestor_record_offset":211552,"inlinee":14903,"source_line":1822,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1643,"record_offset":203324,"parent_record_offset":203300,"ancestor_record_offset":203252,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1643,"record_offset":211992,"parent_record_offset":211968,"ancestor_record_offset":211552,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":203356,"parent_record_offset":203324,"ancestor_record_offset":203252,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":203900,"parent_record_offset":203776,"ancestor_record_offset":203252,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":203512,"parent_record_offset":203476,"ancestor_record_offset":203252,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":203924,"parent_record_offset":203900,"ancestor_record_offset":203252,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":203536,"parent_record_offset":203512,"ancestor_record_offset":203252,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1643,"record_offset":203384,"parent_record_offset":203324,"ancestor_record_offset":203252,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1643,"record_offset":203688,"parent_record_offset":203664,"ancestor_record_offset":203252,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":212116,"parent_record_offset":212092,"ancestor_record_offset":211552,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":212140,"parent_record_offset":212116,"ancestor_record_offset":211552,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":212164,"parent_record_offset":212140,"ancestor_record_offset":211552,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1643,"record_offset":212232,"parent_record_offset":212208,"ancestor_record_offset":211552,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1643,"record_offset":212092,"parent_record_offset":212068,"ancestor_record_offset":211552,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$28::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":1643,"record_offset":203748,"parent_record_offset":203720,"ancestor_record_offset":203252,"inlinee":12297,"source_line":1829,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":1643,"record_offset":203568,"parent_record_offset":203476,"ancestor_record_offset":203252,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":1643,"record_offset":203812,"parent_record_offset":203776,"ancestor_record_offset":203252,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":1643,"record_offset":203476,"parent_record_offset":203448,"ancestor_record_offset":203252,"inlinee":6092,"source_line":67,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":1643,"record_offset":203776,"parent_record_offset":203748,"ancestor_record_offset":203252,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1643,"record_offset":203596,"parent_record_offset":203568,"ancestor_record_offset":203252,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1643,"record_offset":203840,"parent_record_offset":203812,"ancestor_record_offset":203252,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1643,"record_offset":212208,"parent_record_offset":212024,"ancestor_record_offset":211552,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1643,"record_offset":212068,"parent_record_offset":212024,"ancestor_record_offset":211552,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x774A
// 首条位置: {"module":519,"inlinee":30538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":42484,"parent_record_offset":41356,"ancestor_record_offset":41356,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":4076,"record_offset":1388,"parent_record_offset":1348,"ancestor_record_offset":1348,"inlinee":12972,"source_line":227,"archive":"_pdb/C/modules/04076.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$6::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":41416,"parent_record_offset":41356,"ancestor_record_offset":41356,"inlinee":30540,"source_line":223,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":42176,"parent_record_offset":41356,"ancestor_record_offset":41356,"inlinee":30544,"source_line":234,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42620,"parent_record_offset":42596,"ancestor_record_offset":41356,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":41356,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42668,"parent_record_offset":42644,"ancestor_record_offset":41356,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":519,"record_offset":42596,"parent_record_offset":42564,"ancestor_record_offset":41356,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":42564,"parent_record_offset":42520,"ancestor_record_offset":41356,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":519,"record_offset":42520,"parent_record_offset":42484,"ancestor_record_offset":41356,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41696,"parent_record_offset":41668,"ancestor_record_offset":41356,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":41356,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":41724,"parent_record_offset":41668,"ancestor_record_offset":41356,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":41772,"parent_record_offset":41748,"ancestor_record_offset":41356,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":41640,"parent_record_offset":41612,"ancestor_record_offset":41356,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4076,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":1348,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04076.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$60::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":519,"record_offset":41448,"parent_record_offset":41416,"ancestor_record_offset":41356,"inlinee":30542,"source_line":216,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":41356,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":41356,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":519,"record_offset":42324,"parent_record_offset":42296,"ancestor_record_offset":41356,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":519,"record_offset":42392,"parent_record_offset":42368,"ancestor_record_offset":41356,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":519,"record_offset":42248,"parent_record_offset":42224,"ancestor_record_offset":41356,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":42368,"parent_record_offset":42176,"ancestor_record_offset":41356,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":42224,"parent_record_offset":42176,"ancestor_record_offset":41356,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":519,"record_offset":41496,"parent_record_offset":41448,"ancestor_record_offset":41356,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41552,"parent_record_offset":41524,"ancestor_record_offset":41356,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41668,"parent_record_offset":41612,"ancestor_record_offset":41356,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":519,"record_offset":41524,"parent_record_offset":41496,"ancestor_record_offset":41356,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":519,"record_offset":41612,"parent_record_offset":41580,"ancestor_record_offset":41356,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":519,"record_offset":41580,"parent_record_offset":41552,"ancestor_record_offset":41356,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=27
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67BF
// 首条位置: {"module":421,"inlinee":26559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":421,"record_offset":35184,"parent_record_offset":34836,"ancestor_record_offset":34836,"inlinee":4851,"source_line":1644,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":421,"record_offset":34872,"parent_record_offset":34836,"ancestor_record_offset":34836,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":421,"record_offset":35152,"parent_record_offset":34836,"ancestor_record_offset":34836,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":421,"record_offset":35296,"parent_record_offset":34836,"ancestor_record_offset":34836,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":421,"record_offset":35208,"parent_record_offset":35184,"ancestor_record_offset":34836,"inlinee":4852,"source_line":1888,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":421,"record_offset":35320,"parent_record_offset":35296,"ancestor_record_offset":34836,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":421,"record_offset":35232,"parent_record_offset":35208,"ancestor_record_offset":34836,"inlinee":4853,"source_line":993,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":421,"record_offset":35256,"parent_record_offset":35232,"ancestor_record_offset":34836,"inlinee":4854,"source_line":935,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":421,"record_offset":34900,"parent_record_offset":34872,"ancestor_record_offset":34836,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":421,"record_offset":35080,"parent_record_offset":34956,"ancestor_record_offset":34836,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":421,"record_offset":35104,"parent_record_offset":35080,"ancestor_record_offset":34836,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":421,"record_offset":35412,"parent_record_offset":35320,"ancestor_record_offset":34836,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":421,"record_offset":34928,"parent_record_offset":34900,"ancestor_record_offset":34836,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":421,"record_offset":34992,"parent_record_offset":34956,"ancestor_record_offset":34836,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":421,"record_offset":34956,"parent_record_offset":34928,"ancestor_record_offset":34836,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":421,"record_offset":35020,"parent_record_offset":34992,"ancestor_record_offset":34836,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$3::deref
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67BD
// 首条位置: {"module":421,"inlinee":26557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>
// 记录形式: compiled_procedure; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":48408,"rva":10256368,"va":5378965488}
// 编译实例 RVAs: 0x9c7ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=2
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9660462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e8d2976cff","size":5,"soft_fail":false,"target_va":5369305584,"va":5378966041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91500
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2921250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e822932c00","size":5,"soft_fail":false,"target_va":5381887232,"va":5378965977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37610932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e8b4e53d02","size":5,"soft_fail":false,"target_va":5416576832,"va":5378965895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37610229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e8f5e23d02","size":5,"soft_fail":false,"target_va":5416576144,"va":5378965910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37604125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e81dcb3d02","size":5,"soft_fail":false,"target_va":5416569744,"va":5378965614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37606215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e847d33d02","size":5,"soft_fail":false,"target_va":5416572144,"va":5378965924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5019301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e8a5964c00","size":5,"soft_fail":false,"target_va":5383984848,"va":5378965542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5019662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378965488,"raw_hex":"e80e984c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378965693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":49452,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":13798,"source_line":1667,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":48596,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":48780,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":49204,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":48624,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":48864,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":12306,"source_line":457,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":49844,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":14418,"source_line":1275,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":49476,"parent_record_offset":49452,"ancestor_record_offset":48408,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":50148,"parent_record_offset":50112,"ancestor_record_offset":48408,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":50112,"parent_record_offset":50088,"ancestor_record_offset":48408,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":49868,"parent_record_offset":49844,"ancestor_record_offset":48408,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":48804,"parent_record_offset":48780,"ancestor_record_offset":48408,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":519,"record_offset":49020,"parent_record_offset":48928,"ancestor_record_offset":48408,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":50180,"parent_record_offset":50148,"ancestor_record_offset":48408,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50020,"parent_record_offset":49956,"ancestor_record_offset":48408,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":48688,"parent_record_offset":48652,"ancestor_record_offset":48408,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":49048,"parent_record_offset":48928,"ancestor_record_offset":48408,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":49384,"parent_record_offset":49260,"ancestor_record_offset":48408,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":48964,"parent_record_offset":48928,"ancestor_record_offset":48408,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50044,"parent_record_offset":50020,"ancestor_record_offset":48408,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":48712,"parent_record_offset":48688,"ancestor_record_offset":48408,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":48988,"parent_record_offset":48964,"ancestor_record_offset":48408,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":48408,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50240,"parent_record_offset":50112,"ancestor_record_offset":48408,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":49928,"parent_record_offset":49900,"ancestor_record_offset":48408,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$18::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":49232,"parent_record_offset":49204,"ancestor_record_offset":48408,"inlinee":12294,"source_line":1434,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":48652,"parent_record_offset":48624,"ancestor_record_offset":48408,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":49956,"parent_record_offset":49928,"ancestor_record_offset":48408,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":49108,"parent_record_offset":48892,"ancestor_record_offset":48408,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":49296,"parent_record_offset":49260,"ancestor_record_offset":48408,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":48928,"parent_record_offset":48892,"ancestor_record_offset":48408,"inlinee":12308,"source_line":508,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":48892,"parent_record_offset":48864,"ancestor_record_offset":48408,"inlinee":12307,"source_line":107,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":49260,"parent_record_offset":49232,"ancestor_record_offset":48408,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":48744,"parent_record_offset":48652,"ancestor_record_offset":48408,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":49324,"parent_record_offset":49296,"ancestor_record_offset":48408,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=4
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":50280,"rva":10256944,"va":5378966064}
// 编译实例 RVAs: 0x9c8230, 0x148cd70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=4
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9661054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e882956cff","size":5,"soft_fail":false,"target_va":5369305584,"va":5378966633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90c90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2918498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e862882c00","size":5,"soft_fail":false,"target_va":5381885072,"va":5378966569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37610357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e875e33d02","size":5,"soft_fail":false,"target_va":5416576832,"va":5378966470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26318194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390257520,"raw_hex":"e872959101","size":5,"soft_fail":false,"target_va":5416576144,"va":5390257945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37603550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e8dec83d02","size":5,"soft_fail":false,"target_va":5416569744,"va":5378966189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37605630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e8fed03d02","size":5,"soft_fail":false,"target_va":5416572144,"va":5378966509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5018726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e866944c00","size":5,"soft_fail":false,"target_va":5383984848,"va":5378966117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5019084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378966064,"raw_hex":"e8cc954c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378966271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t37609642","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378966064,"raw_hex":"e9aae03d02","size":5,"soft_fail":false,"target_va":5416576144,"va":5378966497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":51316,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":13798,"source_line":1667,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":50460,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":519,"record_offset":50644,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":51068,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":50488,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":50728,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":12306,"source_line":457,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":51708,"parent_record_offset":50280,"ancestor_record_offset":50280,"inlinee":14418,"source_line":1275,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":51340,"parent_record_offset":51316,"ancestor_record_offset":50280,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":52012,"parent_record_offset":51976,"ancestor_record_offset":50280,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":51976,"parent_record_offset":51952,"ancestor_record_offset":50280,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":519,"record_offset":51732,"parent_record_offset":51708,"ancestor_record_offset":50280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":50668,"parent_record_offset":50644,"ancestor_record_offset":50280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":519,"record_offset":50884,"parent_record_offset":50792,"ancestor_record_offset":50280,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":519,"record_offset":52044,"parent_record_offset":52012,"ancestor_record_offset":50280,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":51884,"parent_record_offset":51820,"ancestor_record_offset":50280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50552,"parent_record_offset":50516,"ancestor_record_offset":50280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50912,"parent_record_offset":50792,"ancestor_record_offset":50280,"inlinee":4713,"source_line":3097,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":51248,"parent_record_offset":51124,"ancestor_record_offset":50280,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50828,"parent_record_offset":50792,"ancestor_record_offset":50280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":51908,"parent_record_offset":51884,"ancestor_record_offset":50280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50576,"parent_record_offset":50552,"ancestor_record_offset":50280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":50852,"parent_record_offset":50828,"ancestor_record_offset":50280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":51272,"parent_record_offset":51248,"ancestor_record_offset":50280,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":52104,"parent_record_offset":51976,"ancestor_record_offset":50280,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":519,"record_offset":51792,"parent_record_offset":51764,"ancestor_record_offset":50280,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$18::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":51096,"parent_record_offset":51068,"ancestor_record_offset":50280,"inlinee":12294,"source_line":1434,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":50516,"parent_record_offset":50488,"ancestor_record_offset":50280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":519,"record_offset":51820,"parent_record_offset":51792,"ancestor_record_offset":50280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":50972,"parent_record_offset":50756,"ancestor_record_offset":50280,"inlinee":6094,"source_line":1148,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":51160,"parent_record_offset":51124,"ancestor_record_offset":50280,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":50792,"parent_record_offset":50756,"ancestor_record_offset":50280,"inlinee":12308,"source_line":508,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":50756,"parent_record_offset":50728,"ancestor_record_offset":50280,"inlinee":12307,"source_line":107,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":51124,"parent_record_offset":51096,"ancestor_record_offset":50280,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":50608,"parent_record_offset":50516,"ancestor_record_offset":50280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":51188,"parent_record_offset":51160,"ancestor_record_offset":50280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publication
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publication", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > >", "constructor": "alloc::sync::ArcInner", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publication", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLockWriteGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::PublicationReader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publisher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publisher", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::systems::collision::world::ColliderDiff> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "alloc::sync::ArcInner", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::systems::collision::world::ColliderDiff", "enum2$<country_core::systems::collision::world::ColliderDelta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscriber
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscription
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscription", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Subscription", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 156; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >
// 信息 ID: 4e44f31e17fd765b4b6972a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::systems::collision::world::ColliderDelta"]; 字段布局 = unknown。
// [类型关系] tabloid::Publisher::publish -> ["country_core::systems::collision::world::ColliderDiff", "enum2$<country_core::systems::collision::world::ColliderDelta>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::dispose_collider_mesh_instance @ 0xa01ecb -> 0x9c6dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::Rapier::on_collider_upserted @ 0xa02a17 -> 0x9c6dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc79a63 -> 0x9c6dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc79e14 -> 0x9c6dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 9441875efdae384332a143a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Publisher::publish -> ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::flush_colliders @ 0x9f7601 -> 0x9c7460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging @ 0xac44b0 -> 0x9c7460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0xc70868 -> 0x9c7460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0xc87741 -> 0x9c7460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0xc87790 -> 0x9c7460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c82b7 -> 0x28d47d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0x2859fc1 -> 0x28d47d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0x285a010 -> 0x28d47d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>
// 信息 ID: 3ca109101c1c3eec6ba59484
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d1b1f6201f2a09a88654812c: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)
// [类型关系] tabloid::Publisher::subscribe -> ["country_core::systems::collision::world::ColliderDiff"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::structural_graph_subscribe @ 0x8a277c -> 0x9c7d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> > @ 0x191c604 -> 0x9c7d10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> @ 0x19a0cc7 -> 0x9c7d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 393cf7510a690c2e3b7ce937
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc68acc99a848e3e9f4a896d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] tabloid::Publisher::subscribe -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::overhang_subscribe_to_tabloids @ 0x837b51 -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::stairs_subscribe_lazy @ 0x88f2b3 -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::stairs_subscribe_lazy @ 0x88f300 -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::subscribe_to_changes @ 0x9f8ac4 -> 0x9c7e80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::plant_sample_raster::PlantSampleRaster::subscribe @ 0xac5a94 -> 0x9c7e80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0xc870da -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0xc8710f -> 0x9c7e80 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0x116ba77 -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x1954c17 -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x28c6a0c -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x28c6b9b -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0x2b20e8a -> 0x9c7e80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0x2b20ebf -> 0x9c7e80 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>
// 信息 ID: ce09d1b6d061ca49e7669e8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Subscriber::take -> ["country_core::systems::collision::world::ColliderDiff"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph @ 0x8a29f2 -> 0x9c7ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0xbed034 -> 0x9c7ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 9f04a94b591903d6e8e0c731
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Subscriber::take -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0> @ 0x9c624d -> 0x9c8230 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1> @ 0x9c631d -> 0x9c8230 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x838196 -> 0x148cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags @ 0x891d73 -> 0x148cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bc507 -> 0x148cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests @ 0x28c771f -> 0x148cd70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$0::default
// 信息 ID: cfcc893ff046f4a313d4c56d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::impl$0::build; module=318, inline_record=3704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::impl$0::insert_terrain_resources; module=511, inline_record=220472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::impl$0::insert_terrain_resources; module=511, inline_record=221136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=108908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::RaycastWorld::on_level_reload; module=527, inline_record=13932; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::plant_sample_raster::PlantSampleRaster::new; module=550, inline_record=51484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146616; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$0
// 信息 ID: 05875d52612a453cb02d515a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1643, inline_record=203252; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1643, inline_record=204280; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; module=1643, inline_record=211552; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=2591, inline_record=3924; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=2756, inline_record=29052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$1
// 信息 ID: b69f39bbe656aeb93b82ab1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >; module=519, inline_record=41356; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=519, inline_record=44392; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=4076, inline_record=1348; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=4735, inline_record=1348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$2::drop
// 信息 ID: 93a5db02591e360541d1c104
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=421, inline_record=34836; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags; module=457, inline_record=26936; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph; module=461, inline_record=9308; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff> >; module=515, inline_record=101008; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=515, inline_record=1868952; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>; module=519, inline_record=19448; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>; module=519, inline_record=20340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=585, inline_record=51208; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$3::deref
// 信息 ID: d656c4338a6784abad3fadcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=421, inline_record=34784; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags; module=457, inline_record=26884; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::playermade_stairs::calculate_distortion_from_structural_graph::calculate_distortion_from_structural_graph; module=461, inline_record=5212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=585, inline_record=43280; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=4054, inline_record=7324; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::add_birdnests::add_birdnests; module=4056, inline_record=2844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
